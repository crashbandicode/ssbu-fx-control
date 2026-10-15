//! Effect-request trace for the measurement harness (`TRACE_EFFECTS` marker).
//!
//! While the marker is present every request hook reports the first time it
//! sees a (request name, actor category, kind, status) combination as event
//! code 110, so the layers a move really requests can be read from
//! `events.log` without guessing from scripts (`tools/resolve_trace.py` turns
//! the hashes back into names). Bounded: after `CAPACITY` distinct keys the
//! trace stops. The game thread never waits: a contended lock skips the entry.
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

pub const CAPACITY: usize = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Key {
    pub hash: u64,
    pub category: u8,
    pub kind: i32,
    pub status: i32,
}

pub struct Trace {
    enabled: AtomicBool,
    seen: Mutex<Vec<Key>>,
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

impl Trace {
    pub const fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            seen: Mutex::new(Vec::new()),
        }
    }

    pub fn enable(&self, on: bool) {
        self.enabled.store(on, Ordering::Release);
    }

    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }

    /// `Some(ordinal)` (1-based) the first time `key` is seen; `None` when it
    /// was seen before, the trace is full, or the lock is busy.
    pub fn first_seen(&self, key: Key) -> Option<u16> {
        let mut seen = self.seen.try_lock().ok()?;
        if seen.contains(&key) {
            return None;
        }
        if seen.len() >= CAPACITY {
            return None;
        }
        seen.push(key);
        Some(seen.len() as u16)
    }

    pub fn len(&self) -> usize {
        self.seen.lock().map(|seen| seen.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Event argument `b`: category (bits 0–7), kind (8–23, two's complement 16
/// bits), status (24–39), ordinal (40–55).
pub fn pack(key: Key, ordinal: u16) -> u64 {
    u64::from(key.category)
        | (u64::from(key.kind as u16) << 8)
        | (u64::from(key.status as u16) << 24)
        | (u64::from(ordinal) << 40)
}

pub fn unpack(b: u64) -> (Key, u16) {
    (
        Key {
            hash: 0,
            category: (b & 0xff) as u8,
            kind: i32::from(((b >> 8) & 0xffff) as u16 as i16),
            status: i32::from(((b >> 24) & 0xffff) as u16 as i16),
        },
        ((b >> 40) & 0xffff) as u16,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_sight_only_bounded_and_packed() {
        let trace = Trace::new();
        assert!(!trace.enabled());
        trace.enable(true);
        let key = Key {
            hash: 0x1234,
            category: 1,
            kind: 0x58,
            status: -1,
        };
        assert_eq!(trace.first_seen(key), Some(1));
        assert_eq!(trace.first_seen(key), None);
        let other = Key { status: 7, ..key };
        assert_eq!(trace.first_seen(other), Some(2));
        for n in 0..CAPACITY as i32 {
            trace.first_seen(Key {
                hash: 99,
                category: 0,
                kind: n,
                status: 0,
            });
        }
        assert_eq!(trace.len(), CAPACITY);
        assert_eq!(
            trace.first_seen(Key {
                hash: 1,
                category: 0,
                kind: -5,
                status: 0
            }),
            None
        );
        let (back, ordinal) = unpack(pack(key, 300));
        assert_eq!(ordinal, 300);
        assert_eq!((back.category, back.kind, back.status), (1, 0x58, -1));
    }
}
