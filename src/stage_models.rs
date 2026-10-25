//! Bounded observations linking live stage renderers to actual resource paths.
//!
//! This owns no native references. Runtime observers must invalidate an address
//! before construction, destruction, or resource rebinding. Ambiguous ownership
//! stays unclassified until that invalidation. A missed writer disables the
//! registry rather than permitting a stale association to survive. Each of the
//! 128 buckets has its own eight-entry lock. A writer takes only that bucket's
//! write lock, and only for bookkeeping, never while calling the engine. A
//! reader does not skip because a different bucket is being written. A writer
//! in the same bucket still makes that lookup skip.
use std::sync::{
    atomic::{AtomicBool, Ordering},
    RwLock,
};

const BUCKETS: usize = 128;
const WAYS: usize = 8;

/// Read-only registry lookup. `Matched` copies the path; it does not borrow
/// the map or retain a native address. `Ambiguous` is a stored path of zero.
/// `Unhealthy` covers a null renderer, a disabled registry, or a poisoned lock.
/// This read never calls [`Registry::disable`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lookup {
    Matched(u64),
    Unregistered,
    Ambiguous,
    Contended,
    Unhealthy,
}

#[derive(Clone, Copy)]
struct Entry {
    renderer: usize,
    // Zero means unknown or conflicting paths; it is never upgraded in-place.
    path: u64,
}

impl Entry {
    const EMPTY: Self = Self {
        renderer: 0,
        path: 0,
    };
}

pub struct Registry {
    healthy: AtomicBool,
    entries: [RwLock<[Entry; WAYS]>; BUCKETS],
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub const fn new() -> Self {
        Self {
            healthy: AtomicBool::new(true),
            entries: [const { RwLock::new([Entry::EMPTY; WAYS]) }; BUCKETS],
        }
    }

    pub fn healthy(&self) -> bool {
        self.healthy.load(Ordering::Acquire)
    }

    fn bucket(renderer: usize) -> usize {
        let value = renderer >> 4;
        (value ^ (value >> 9)) & (BUCKETS - 1)
    }

    fn disable(&self) -> bool {
        self.healthy.store(false, Ordering::Release);
        false
    }

    /// Called only after the verified constructor's initial resource binding.
    /// Conflicting observations cannot overwrite an earlier resource's identity.
    pub fn observe(&self, renderer: usize, path: u64) -> bool {
        if !self.healthy() {
            return false;
        }
        if renderer == 0 {
            return true;
        }
        let Ok(mut bucket) = self.entries[Self::bucket(renderer)].write() else {
            return self.disable();
        };
        let path = path & 0xff_ffff_ffff;
        if let Some(entry) = bucket.iter_mut().find(|entry| entry.renderer == renderer) {
            if entry.path != path {
                entry.path = 0;
            }
        } else if let Some(entry) = bucket.iter_mut().find(|entry| entry.renderer == 0) {
            *entry = Entry { renderer, path };
        } else {
            return self.disable();
        }
        self.healthy()
    }

    /// A lifecycle writer waits for the short bookkeeping section rather than
    /// silently missing invalidation. No engine call is made under this lock.
    pub fn invalidate(&self, renderer: usize) -> bool {
        if !self.healthy() {
            return false;
        }
        let Ok(mut bucket) = self.entries[Self::bucket(renderer)].write() else {
            return self.disable();
        };
        for entry in &mut *bucket {
            if entry.renderer == renderer {
                *entry = Entry::EMPTY;
            }
        }
        self.healthy()
    }

    /// Hot-path lookup: at most eight entries, no allocation or native reads.
    /// Same read as [`Self::lookup`]; only a copied nonzero path is returned.
    pub fn path(&self, renderer: usize) -> Option<u64> {
        match self.lookup(renderer) {
            Lookup::Matched(path) => Some(path),
            Lookup::Unregistered | Lookup::Ambiguous | Lookup::Contended | Lookup::Unhealthy => {
                None
            }
        }
    }

    /// Detailed form of [`Self::path`]. Nonblocking on the address's bucket,
    /// and it does not disable the registry when that bucket is busy or poisoned.
    pub fn lookup(&self, renderer: usize) -> Lookup {
        if renderer == 0 || !self.healthy() {
            return Lookup::Unhealthy;
        }
        let entries = match self.entries[Self::bucket(renderer)].try_read() {
            Ok(entries) => entries,
            Err(std::sync::TryLockError::WouldBlock) => return Lookup::Contended,
            Err(std::sync::TryLockError::Poisoned(_)) => return Lookup::Unhealthy,
        };
        // Copy a nonzero path before the same trailing healthy check `path`
        // has always used. Ambiguous and missing rows stay distinct when that
        // flag drops; they already produced `None`.
        match entries.iter().find(|entry| entry.renderer == renderer) {
            Some(entry) if entry.path != 0 => {
                let path = entry.path;
                if self.healthy() {
                    Lookup::Matched(path)
                } else {
                    Lookup::Unhealthy
                }
            }
            Some(_) => Lookup::Ambiguous,
            None => Lookup::Unregistered,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_reuse_and_resource_rebind_require_a_new_observation() {
        let registry = Registry::new();
        let renderer = 0x10000;
        assert_eq!(registry.path(renderer), None);
        assert!(registry.observe(renderer, 0x2b4bb4272c));
        assert_eq!(registry.path(renderer), Some(0x2b4bb4272c));
        assert!(registry.invalidate(renderer));
        assert_eq!(registry.path(renderer), None);
        assert!(registry.observe(renderer, 0x312c350aa1));
        assert_eq!(registry.path(renderer), Some(0x312c350aa1));
        assert!(registry.invalidate(renderer));
        assert_eq!(registry.path(renderer), None);
    }

    #[test]
    fn shared_or_unresolved_paths_stay_unclassified_until_lifecycle_reset() {
        for first in [0, 0x2b4bb4272c] {
            let registry = Registry::new();
            assert!(registry.observe(0x10000, first));
            assert!(registry.observe(0x10000, 0x312c350aa1));
            assert_eq!(registry.path(0x10000), None);
            assert!(registry.observe(0x10000, first));
            assert_eq!(registry.path(0x10000), None);
            assert!(registry.invalidate(0x10000));
            assert!(registry.observe(0x10000, 0x2b4bb4272c));
            assert_eq!(registry.path(0x10000), Some(0x2b4bb4272c));
        }
    }

    #[test]
    fn repeated_same_resource_is_stable_and_other_renderers_are_independent() {
        let registry = Registry::new();
        assert!(registry.observe(0x10000, 0x2b4bb4272c));
        assert!(registry.observe(0x10000, 0xdead_be2b_4bb4_272c));
        assert!(registry.observe(0x20000, 0x312c350aa1));
        assert!(registry.invalidate(0x20000));
        assert_eq!(registry.path(0x10000), Some(0x2b4bb4272c));
    }

    #[test]
    fn capacity_failure_disables_old_matches_instead_of_eviction_or_stale_reuse() {
        let registry = Registry::new();
        let addresses: Vec<_> = (0x10000..0x100000)
            .step_by(16)
            .filter(|&address| Registry::bucket(address) == 0)
            .take(WAYS + 1)
            .collect();
        assert_eq!(addresses.len(), WAYS + 1);
        for &address in &addresses[..WAYS] {
            assert!(registry.observe(address, 0x2b4bb4272c));
        }
        assert!(!registry.observe(addresses[WAYS], 0x312c350aa1));
        assert!(!registry.healthy());
        for address in addresses {
            assert_eq!(registry.path(address), None);
            assert_eq!(registry.lookup(address), Lookup::Unhealthy);
        }
        assert!(!registry.invalidate(0x10000));
    }

    #[test]
    fn same_bucket_writer_contention_skips_lookup_and_lifecycle_waits_until_removed() {
        let registry = Registry::new();
        assert!(registry.observe(0x10000, 0x2b4bb4272c));
        std::thread::scope(|scope| {
            let held = registry.entries[Registry::bucket(0x10000)].write().unwrap();
            assert_eq!(registry.path(0x10000), None);
            assert_eq!(registry.lookup(0x10000), Lookup::Contended);
            assert!(registry.healthy());
            let (started, start) = std::sync::mpsc::channel();
            let (finished, finish) = std::sync::mpsc::channel();
            let registry = &registry;
            let worker = scope.spawn(move || {
                started.send(()).unwrap();
                assert!(registry.invalidate(0x10000));
                finished.send(()).unwrap();
            });
            start.recv().unwrap();
            assert!(matches!(
                finish.try_recv(),
                Err(std::sync::mpsc::TryRecvError::Empty)
            ));
            drop(held);
            worker.join().unwrap();
            finish.recv().unwrap();
        });
        assert!(registry.healthy());
        assert_eq!(registry.path(0x10000), None);
        assert_eq!(registry.lookup(0x10000), Lookup::Unregistered);
    }

    #[test]
    fn lookup_reports_ambiguous_null_and_a_copied_match_without_disabling() {
        let registry = Registry::new();
        assert_eq!(registry.lookup(0), Lookup::Unhealthy);
        assert_eq!(registry.path(0), None);
        assert!(registry.healthy());
        assert!(registry.observe(0x10000, 0x2b4bb4272c));
        assert!(registry.observe(0x10000, 0x312c350aa1));
        assert_eq!(registry.lookup(0x10000), Lookup::Ambiguous);
        assert_eq!(registry.path(0x10000), None);
        assert!(registry.healthy());
        assert!(registry.invalidate(0x10000));
        assert_eq!(registry.lookup(0x10000), Lookup::Unregistered);
        assert!(registry.observe(0x10000, 0x2b4bb4272c));
        let found = registry.lookup(0x10000);
        assert_eq!(found, Lookup::Matched(0x2b4bb4272c));
        assert_eq!(registry.path(0x10000), Some(0x2b4bb4272c));
        let Lookup::Matched(path) = found else {
            panic!("matched path was not copied");
        };
        let _held = registry.entries[Registry::bucket(0x10000)].write().unwrap();
        assert_eq!(path, 0x2b4bb4272c);
        assert_eq!(registry.lookup(0x10000), Lookup::Contended);
        assert!(registry.healthy());
    }

    #[test]
    fn held_read_guard_allows_same_bucket_lookup() {
        let registry = Registry::new();
        let renderer = 0x10000usize;
        let path = 0x2b4bb4272cu64;
        assert!(registry.observe(renderer, path));
        let _held = registry.entries[Registry::bucket(renderer)].read().unwrap();
        assert_eq!(registry.lookup(renderer), Lookup::Matched(path));
        assert_eq!(registry.path(renderer), Some(path));
        assert!(registry.healthy());
    }

    #[test]
    fn different_bucket_writer_does_not_block_lookup_or_lifecycle_write() {
        let registry = Registry::new();
        let target = 0x10000usize;
        let other = 0x20000usize;
        let path = 0x2b4bb4272cu64;
        assert_ne!(Registry::bucket(target), Registry::bucket(other));
        assert!(registry.observe(target, path));
        std::thread::scope(|scope| {
            let held = registry.entries[Registry::bucket(other)].write().unwrap();
            assert_eq!(registry.lookup(target), Lookup::Matched(path));
            assert_eq!(registry.path(target), Some(path));
            let (finished, finish) = std::sync::mpsc::channel();
            let registry = &registry;
            let worker = scope.spawn(move || {
                assert!(registry.invalidate(target));
                assert!(registry.observe(target, path));
                finished.send(()).unwrap();
            });
            let completed = finish.recv_timeout(std::time::Duration::from_secs(2));
            drop(held);
            worker.join().unwrap();
            completed.expect("lifecycle write blocked on a different bucket");
        });
        assert!(registry.healthy());
        assert_eq!(registry.lookup(target), Lookup::Matched(path));
        assert_eq!(registry.path(target), Some(path));
    }
}
