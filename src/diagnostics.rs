//! Bounded, nonblocking event capture. File I/O belongs to the background sink.
//! Contended events are counted as dropped; a game hook never waits for a logger.
//!
//! Codes keep the numbers they had in Online Deluxe, so the FX documents and
//! existing logs stay readable. `events.log` lines are
//! `sequence tick code arg0 arg1`.
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};

pub const CAPACITY: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Code {
    /// Diagnostics started. `a` is the plugin text start, `b` the game text base.
    Boot = 1,
    /// Not Smash 13.0.5; every hook skipped.
    UnsupportedVersion = 2,
    /// About to patch. `a` is the hook id or offset, `b` the address or word.
    HookInstalling = 3,
    /// Patched. `a` is the hook id or offset, `b` the address.
    HookInstalled = 4,
    /// Not patched (unreadable entry or unexpected words). `a` id/offset, `b` detail.
    HookRejected = 5,
    /// INI loaded. `a` is the online source, `b` the common mode (0 On, 1 Off,
    /// 2 Reduced) or `u64::MAX` while rows differ.
    ConfigLoaded = 6,
    /// One accepted menu edit. `a` is the option index, `b` the choice.
    MenuChanged = 8,
    /// Off substituted `null` for one request. `a` hash, `b` sampled total.
    FxSuppressed = 11,
    /// Every ten diagnostics passes (5 s). `a` is the table generation, `b` the pass.
    Heartbeat = 15,
    /// Coexistence. `a` = 1 a legacy SsbuSync host is present, 2 the DISABLE
    /// marker skipped every hook, 3 Deluxe without an online-context export
    /// (Scope row Unavailable, policy everywhere), 4 own online hooks refused.
    Compatibility = 16,
    /// First instruction word at a hook entry. `a` id, `b` word.
    HookEntryWord = 17,
    /// The INI was rewritten. `a` is the table generation, `b` 1 ok / 0 error.
    ConfigSaved = 24,
    /// ACMD effect command seen (sampled). `a` path id, `b` first hash.
    AcmdEffectSeen = 27,
    /// ACMD Off substituted `null`. `a` original hash, `b` path << 32 | total.
    AcmdEffectSuppressed = 28,
    /// ACMD argument outside the primitive set; the command was left unchanged.
    AcmdStackRejected = 29,
    /// Online source and mark. `a` is the source (1 Deluxe, 2 own hooks,
    /// 3 unavailable, 4 undetected); `b` = 1 own latency mark set, 2 cleared
    /// by a stage load, 0 for the startup record.
    FxOnlineContext = 30,
    /// Reduced substituted `null` for one decorative layer. `a` is the original hash.
    FxReducedCull = 84,
    /// Reduced kept a core effect request. `a` is the hash.
    FxReducedKeep = 85,
    /// Reduced was selected but scope blocked that matched request. `a` is the hash.
    FxReducedRestore = 86,
    /// A reduced-table name was seen on a different move, so it stayed On.
    FxReducedPreserve = 87,
    /// A sibling file was not registered or its load was rejected.
    /// `a` is the reason: 1 missing API, 2 transform, 3 size.
    /// `b` is that file's path hash. The original request stays. This is not Off.
    FxVariantUnavailable = 91,
    /// Sibling callback armed (`a` 0, `b` path hash), or one successful load of
    /// that whole file (`a` path hash, `b` output length in bytes).
    FxVariantReady = 92,
    /// Reduced forwarded one parent to its sibling. Sampled on the first swap
    /// and every 128th. `a` is the EFF path hash. `b` is the original hash.
    FxVariantUse = 93,
    /// A kill or detach of an original sibling parent also named its clone.
    /// Sampled on the first event and every 128th. `a` original, `b` alias.
    FxVariantCleanup = 94,
    /// Which cleanup boundary ran: `a` 1 `kill_kind`, 2 `detach_kind`, 3 ACMD
    /// `EFFECT_OFF_KIND`, 4 ACMD `EFFECT_DETACH_KIND`. `b` is 1 when that exact
    /// accessor, hash and kill-or-detach was already delivered on this thread.
    FxCleanupRoute = 95,
    /// Special Zoom background request (diagnostic only). `a` hash; `b` bits
    /// 0..7 path (1 `req_screen`, 2 `req_screen_system`, 3 ordinary), bit 8
    /// set when `PROBE_CRITICAL_SCREEN` suppressed it.
    FxCriticalRequest = 96,
    /// `PROBE_CRITICAL_SCREEN` latched at install. `a` is 1 when present.
    CriticalProbe = 97,
    /// Menu registration with Online Deluxe. `a` is the handle or the negative
    /// error as two's complement (`-100` no host, `-101` incompatible host),
    /// `b` the accepted descriptor version (0 on error).
    PluginRegistered = 99,
    /// `TRACE_EFFECTS` harness: the first request of a (name, actor category,
    /// kind, status) combination. `a` is the Hash40, `b` packs category, kind,
    /// status and the ordinal (`trace::pack`). At startup `a` = 0 and `b` = 1
    /// records that the marker was latched.
    FxTrace = 110,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Event {
    pub sequence: u64,
    pub tick: u64,
    pub code: u16,
    pub a: u64,
    pub b: u64,
}

struct State {
    events: [Event; CAPACITY],
    sequence: u64,
}

pub struct Recorder {
    state: Mutex<State>,
    dropped: AtomicU64,
}

#[derive(Clone)]
pub struct Snapshot {
    pub events: [Event; CAPACITY],
    pub newest: u64,
    pub dropped: u64,
}

impl Snapshot {
    /// Returns retained events oldest-first, excluding events already persisted.
    pub fn after(&self, sequence: u64) -> impl Iterator<Item = &Event> {
        let start = self.newest.saturating_sub(CAPACITY as u64).max(sequence);
        ((start + 1)..=self.newest).map(|n| &self.events[((n - 1) % CAPACITY as u64) as usize])
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Recorder {
    pub const fn new() -> Self {
        const EMPTY: Event = Event {
            sequence: 0,
            tick: 0,
            code: 0,
            a: 0,
            b: 0,
        };
        Self {
            state: Mutex::new(State {
                events: [EMPTY; CAPACITY],
                sequence: 0,
            }),
            dropped: AtomicU64::new(0),
        }
    }

    pub fn record(&self, tick: u64, code: Code, a: u64, b: u64) {
        let Ok(mut state) = self.state.try_lock() else {
            self.dropped.fetch_add(1, Ordering::Relaxed);
            return;
        };
        state.sequence += 1;
        let sequence = state.sequence;
        state.events[((sequence - 1) % CAPACITY as u64) as usize] = Event {
            sequence,
            tick,
            code: code as u16,
            a,
            b,
        };
    }

    pub fn snapshot(&self) -> Option<Snapshot> {
        let state = self.state.try_lock().ok()?;
        Some(Snapshot {
            events: state.events,
            newest: state.sequence,
            dropped: self.dropped.load(Ordering::Relaxed),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrap_retains_order_and_exposes_lost_range() {
        let recorder = Recorder::new();
        for n in 1..=150 {
            recorder.record(n, Code::Heartbeat, n * 2, 0);
        }
        let snapshot = recorder.snapshot().unwrap();
        let events: Vec<_> = snapshot.after(0).collect();
        assert_eq!(events.len(), CAPACITY);
        assert_eq!(events.first().unwrap().sequence, 23);
        assert_eq!(events.last().unwrap().a, 300);
        assert!(events
            .windows(2)
            .all(|pair| pair[1].sequence == pair[0].sequence + 1));
        assert_eq!(snapshot.after(148).count(), 2);
        assert_eq!(snapshot.after(150).count(), 0);
    }

    #[test]
    fn fx_codes_keep_their_deluxe_numbers() {
        assert_eq!(Code::Boot as u16, 1);
        assert_eq!(Code::HookInstalled as u16, 4);
        assert_eq!(Code::HookRejected as u16, 5);
        assert_eq!(Code::FxSuppressed as u16, 11);
        assert_eq!(Code::AcmdEffectSeen as u16, 27);
        assert_eq!(Code::AcmdEffectSuppressed as u16, 28);
        assert_eq!(Code::AcmdStackRejected as u16, 29);
        assert_eq!(Code::FxReducedCull as u16, 84);
        assert_eq!(Code::FxReducedKeep as u16, 85);
        assert_eq!(Code::FxReducedRestore as u16, 86);
        assert_eq!(Code::FxReducedPreserve as u16, 87);
        assert_eq!(Code::FxVariantUnavailable as u16, 91);
        assert_eq!(Code::FxVariantReady as u16, 92);
        assert_eq!(Code::FxVariantUse as u16, 93);
        assert_eq!(Code::FxVariantCleanup as u16, 94);
        assert_eq!(Code::FxCleanupRoute as u16, 95);
        assert_eq!(Code::FxCriticalRequest as u16, 96);
        assert_eq!(Code::CriticalProbe as u16, 97);
    }

    #[test]
    fn recursive_or_contended_logging_drops_instead_of_deadlocking() {
        let recorder = Recorder::new();
        let lock = recorder.state.lock().unwrap();
        recorder.record(0, Code::HookInstalled, 0, 0);
        assert!(recorder.snapshot().is_none());
        drop(lock);
        assert_eq!(recorder.snapshot().unwrap().dropped, 1);
    }

    #[test]
    fn concurrent_hooks_keep_records_coherent() {
        let recorder = std::sync::Arc::new(Recorder::new());
        let workers: Vec<_> = (0..4)
            .map(|i| {
                let r = recorder.clone();
                std::thread::spawn(move || {
                    for n in 0..1000 {
                        r.record(n, Code::FxSuppressed, i, !i);
                    }
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
        let snapshot = recorder.snapshot().unwrap();
        assert_eq!(snapshot.newest + snapshot.dropped, 4000);
        assert!(snapshot.after(0).all(|e| e.b == !e.a));
    }
}
