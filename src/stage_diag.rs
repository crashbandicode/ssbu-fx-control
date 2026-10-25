//! Observational stage diagnostics.
//!
//! Latched once from the `TRACE_STAGE_DIAGNOSTICS` marker during stage install.
//! Counters and the particle dedupe record hook calls, not rendered frames,
//! passes, or FPS. No native pointers are stored. The draw and request hooks
//! never allocate, take a blocking lock, or walk resources through this module.
use crate::{stage_models::Lookup, stages};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Mutex,
};

pub const MARKER: &str = "TRACE_STAGE_DIAGNOSTICS";
pub const HASH40_MASK: u64 = 0xff_ffff_ffff;
pub const PARTICLE_CAPACITY: usize = 512;
/// Missing return address, or one that does not fit in `u32`.
pub const CALLER_UNREPRESENTABLE: u32 = u32::MAX;
pub const FLAG_NAME_HIT: u16 = 1;
pub const FLAG_CALLER_ALLOWED: u16 = 2;
pub const FLAG_STAGE_MATCH: u16 = 4;
pub const FLAG_ROW_OFF: u16 = 8;

const STAGE_SLOTS: usize = stages::COUNT;
const REASON_SLOTS: usize = 10;
/// Samples copied into the event ring on one logger wake. The ring holds 128
/// events; a full 110-counter scan must not be published in one wake.
pub const PUBLISH_BATCH: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum ModelReason {
    Found = 1,
    Unregistered = 2,
    Ambiguous = 3,
    Contended = 4,
    Disabled = 5,
    RequestedNativeHidden = 6,
    NativeVisible = 7,
    Suppressed = 8,
    WrongVtable = 9,
    WrongStage = 10,
}

pub const MODEL_REASONS: [(u16, &str); REASON_SLOTS] = [
    (ModelReason::Found as u16, "found"),
    (ModelReason::Unregistered as u16, "unregistered"),
    (ModelReason::Ambiguous as u16, "ambiguous"),
    (ModelReason::Contended as u16, "contended"),
    (ModelReason::Disabled as u16, "disabled"),
    (
        ModelReason::RequestedNativeHidden as u16,
        "requested-native-hidden",
    ),
    (ModelReason::NativeVisible as u16, "native-visible"),
    (ModelReason::Suppressed as u16, "suppressed"),
    (ModelReason::WrongVtable as u16, "wrong-vtable"),
    (ModelReason::WrongStage as u16, "wrong-stage"),
];

/// One draw-hook observation. `leave_visible` is the existing
/// `!requested || model_visibility(...)` predicate. It is ignored unless the
/// lookup matched, the vtable matched, and the path is a target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelCall {
    pub lookup: Lookup,
    pub vtable_ok: bool,
    pub target: bool,
    pub requested: bool,
    pub leave_visible: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelCount {
    pub stage_id: i32,
    pub reason: u16,
    pub count: u64,
}

impl ModelCount {
    /// Code 118 argument `a`: `(stage_id << 32) | reason`.
    pub fn argument_a(self) -> u64 {
        (u64::from(self.stage_id as u32) << 32) | u64::from(self.reason)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParticleObserve {
    /// First time this hash/caller/stage/flags key was stored. The `i16` is
    /// the stage id that was packed into the key.
    First(i16),
    Repeat,
    /// The 512-key table is full. This key was not stored.
    Full,
    /// `try_lock` failed (busy or poisoned). The observation was skipped.
    Contended,
    /// The live stage id does not fit in a signed 16-bit field.
    StageUnrepresentable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParticleKey {
    hash: u64,
    caller: u32,
    stage: i16,
    flags: u16,
}

struct FlushState {
    last: [[u64; REASON_SLOTS]; STAGE_SLOTS],
    pending: Vec<ModelCount>,
}

pub struct Diagnostics {
    enabled: AtomicBool,
    counters: [[AtomicU64; REASON_SLOTS]; STAGE_SLOTS],
    particles: Mutex<Vec<ParticleKey>>,
    flush: Mutex<FlushState>,
}

const fn reason_row() -> [AtomicU64; REASON_SLOTS] {
    [
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
    ]
}

const fn counter_table() -> [[AtomicU64; REASON_SLOTS]; STAGE_SLOTS] {
    [
        reason_row(),
        reason_row(),
        reason_row(),
        reason_row(),
        reason_row(),
        reason_row(),
        reason_row(),
        reason_row(),
        reason_row(),
        reason_row(),
        reason_row(),
    ]
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}

impl Diagnostics {
    pub const fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            counters: counter_table(),
            particles: Mutex::new(Vec::new()),
            flush: Mutex::new(FlushState {
                last: [[0; REASON_SLOTS]; STAGE_SLOTS],
                pending: Vec::new(),
            }),
        }
    }

    pub fn latch(&self, present: bool) {
        self.enabled.store(present, Ordering::Release);
    }

    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }

    /// Blocking reserve. Call only from stage install, before hooks run.
    pub fn reserve_particles(&self) {
        if let Ok(mut seen) = self.particles.lock() {
            if seen.capacity() < PARTICLE_CAPACITY {
                seen.reserve(PARTICLE_CAPACITY);
            }
        }
    }

    pub fn tally(&self, stage_id: i32, call: ModelCall) {
        let Some(stage_index) = stages::by_game_id(stage_id) else {
            return;
        };
        let (lookup, result) = model_reasons(call);
        self.bump(stage_index, lookup);
        if let Some(result) = result {
            self.bump(stage_index, result);
        }
    }

    fn bump(&self, stage_index: usize, reason: ModelReason) {
        let slot = (reason as u16 as usize).wrapping_sub(1);
        if stage_index < STAGE_SLOTS && slot < REASON_SLOTS {
            self.counters[stage_index][slot].fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Publish changed nonzero cumulative counts.
    ///
    /// `allow_scan` starts a diff (the 5 s logger cadence). A nonempty pending
    /// list keeps draining on later wakes without rescanning, at most `limit`
    /// samples per call. Counts of zero are never published. Stage ids outside
    /// the eleven normal stages are not stored.
    pub fn drain_model_counts(&self, limit: usize, allow_scan: bool) -> Vec<ModelCount> {
        let Ok(mut state) = self.flush.lock() else {
            return Vec::new();
        };
        if state.pending.is_empty() && allow_scan {
            for stage in 0..STAGE_SLOTS {
                for slot in 0..REASON_SLOTS {
                    let count = self.counters[stage][slot].load(Ordering::Relaxed);
                    if count != state.last[stage][slot] && count != 0 {
                        state.pending.push(ModelCount {
                            stage_id: stages::ALL[stage].game_id,
                            reason: (slot as u16) + 1,
                            count,
                        });
                    }
                }
            }
        }
        let take = limit.min(state.pending.len());
        let drained: Vec<_> = state.pending.drain(..take).collect();
        for sample in &drained {
            let Some(stage) = stages::by_game_id(sample.stage_id) else {
                continue;
            };
            let slot = (sample.reason as usize).wrapping_sub(1);
            if slot < REASON_SLOTS {
                state.last[stage][slot] = sample.count;
            }
        }
        drained
    }

    /// First-seen particle predicate key. Only this method takes the dedupe lock,
    /// and only with `try_lock`. A repeat, a full table, contention, or a stage
    /// id outside `i16` produces no new key.
    pub fn observe_particle(
        &self,
        hash: u64,
        caller: u32,
        stage: i32,
        flags: u16,
    ) -> ParticleObserve {
        let Ok(stage) = i16::try_from(stage) else {
            return ParticleObserve::StageUnrepresentable;
        };
        let key = ParticleKey {
            hash: hash & HASH40_MASK,
            caller,
            stage,
            flags,
        };
        let Ok(mut seen) = self.particles.try_lock() else {
            return ParticleObserve::Contended;
        };
        if seen.contains(&key) {
            return ParticleObserve::Repeat;
        }
        if seen.len() >= PARTICLE_CAPACITY {
            return ParticleObserve::Full;
        }
        seen.push(key);
        ParticleObserve::First(stage)
    }
}

pub static DIAG: Diagnostics = Diagnostics::new();

/// Lookup class is always one reason. A matched path also gets exactly one
/// result reason. Adding every reason together double-counts matched calls:
/// `found` equals the sum of the result reasons.
pub fn model_reasons(call: ModelCall) -> (ModelReason, Option<ModelReason>) {
    match call.lookup {
        Lookup::Matched(_) => {
            let result = if !call.vtable_ok {
                ModelReason::WrongVtable
            } else if !call.target {
                ModelReason::WrongStage
            } else if !call.requested {
                ModelReason::RequestedNativeHidden
            } else if call.leave_visible {
                ModelReason::NativeVisible
            } else {
                ModelReason::Suppressed
            };
            (ModelReason::Found, Some(result))
        }
        Lookup::Unregistered => (ModelReason::Unregistered, None),
        Lookup::Ambiguous => (ModelReason::Ambiguous, None),
        Lookup::Contended => (ModelReason::Contended, None),
        Lookup::Unhealthy => (ModelReason::Disabled, None),
    }
}

/// Code 119 argument `b`: `(signed stage16 << 48) | (flags16 << 32) | caller32`.
pub fn pack_particle(stage: i16, flags: u16, caller: u32) -> u64 {
    (u64::from(stage as u16) << 48) | (u64::from(flags) << 32) | u64::from(caller)
}

pub fn unpack_particle(packed: u64) -> (i16, u16, u32) {
    let stage = (packed >> 48) as u16 as i16;
    let flags = ((packed >> 32) & 0xffff) as u16;
    let caller = packed as u32;
    (stage, flags, caller)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(
        lookup: Lookup,
        vtable_ok: bool,
        target: bool,
        requested: bool,
        leave_visible: bool,
    ) -> ModelCall {
        ModelCall {
            lookup,
            vtable_ok,
            target,
            requested,
            leave_visible,
        }
    }

    #[test]
    fn reason_codes_match_the_published_names() {
        assert_eq!(MODEL_REASONS.len(), REASON_SLOTS);
        for (index, (code, _)) in MODEL_REASONS.iter().enumerate() {
            assert_eq!(*code as usize, index + 1);
        }
        assert_eq!(ModelReason::Found as u16, 1);
        assert_eq!(ModelReason::WrongStage as u16, 10);
        assert_eq!(FLAG_NAME_HIT, 1);
        assert_ne!(ModelReason::Found as u16, FLAG_ROW_OFF);
    }

    #[test]
    fn matched_calls_partition_into_found_plus_one_result() {
        let cases = [
            (
                call(Lookup::Unregistered, true, true, true, true),
                ModelReason::Unregistered,
                None,
            ),
            (
                call(Lookup::Ambiguous, true, true, true, true),
                ModelReason::Ambiguous,
                None,
            ),
            (
                call(Lookup::Contended, false, false, false, false),
                ModelReason::Contended,
                None,
            ),
            (
                call(Lookup::Unhealthy, false, false, false, false),
                ModelReason::Disabled,
                None,
            ),
            (
                call(Lookup::Matched(0x2edb810e01), false, true, true, false),
                ModelReason::Found,
                Some(ModelReason::WrongVtable),
            ),
            (
                call(Lookup::Matched(1), true, false, true, false),
                ModelReason::Found,
                Some(ModelReason::WrongStage),
            ),
            (
                call(Lookup::Matched(1), true, true, false, false),
                ModelReason::Found,
                Some(ModelReason::RequestedNativeHidden),
            ),
            (
                call(Lookup::Matched(1), true, true, true, true),
                ModelReason::Found,
                Some(ModelReason::NativeVisible),
            ),
            (
                call(Lookup::Matched(1), true, true, true, false),
                ModelReason::Found,
                Some(ModelReason::Suppressed),
            ),
        ];
        for (call, lookup, result) in cases {
            assert_eq!(model_reasons(call), (lookup, result));
        }
    }

    #[test]
    fn only_the_eleven_stages_accumulate_and_flush_is_cumulative() {
        let diag = Diagnostics::new();
        let visible = call(Lookup::Matched(0x2edb810e01), true, true, true, true);
        let contended = call(Lookup::Contended, false, false, false, false);
        diag.tally(95, visible);
        diag.tally(95, visible);
        diag.tally(95, contended);
        diag.tally(-1, visible);
        diag.tally(999, contended);
        diag.tally(0, call(Lookup::Unregistered, false, false, false, false));
        let first = diag.drain_model_counts(PUBLISH_BATCH, true);
        let rest = diag.drain_model_counts(PUBLISH_BATCH, false);
        let mut all = first;
        all.extend(rest);
        assert!(diag.drain_model_counts(PUBLISH_BATCH, false).is_empty());
        assert!(all.iter().all(|sample| sample.count != 0));
        assert!(all
            .iter()
            .all(|sample| stages::by_game_id(sample.stage_id).is_some()));
        let found = all
            .iter()
            .find(|sample| sample.stage_id == 95 && sample.reason == ModelReason::Found as u16)
            .unwrap();
        let native = all
            .iter()
            .find(|sample| {
                sample.stage_id == 95 && sample.reason == ModelReason::NativeVisible as u16
            })
            .unwrap();
        let busy = all
            .iter()
            .find(|sample| sample.stage_id == 95 && sample.reason == ModelReason::Contended as u16)
            .unwrap();
        assert_eq!(found.count, 2);
        assert_eq!(native.count, 2);
        assert_eq!(busy.count, 1);
        assert_eq!(
            found.argument_a(),
            (95u64 << 32) | u64::from(ModelReason::Found as u16)
        );
        diag.tally(95, visible);
        let again = diag.drain_model_counts(PUBLISH_BATCH, true);
        assert_eq!(
            again
                .iter()
                .find(|sample| sample.reason == ModelReason::Found as u16)
                .unwrap()
                .count,
            3
        );
        assert!(diag.drain_model_counts(PUBLISH_BATCH, true).is_empty());
    }

    #[test]
    fn particle_keys_keep_caller_stage_and_off_distinct() {
        let diag = Diagnostics::new();
        diag.reserve_particles();
        let hash = (1u64 << 40) | 0x15b6_ed05;
        assert_eq!(
            diag.observe_particle(hash, 0x25efcdc, 351, FLAG_NAME_HIT | FLAG_CALLER_ALLOWED),
            ParticleObserve::First(351)
        );
        assert_eq!(
            diag.observe_particle(hash, 0x25efcdc, 351, FLAG_NAME_HIT | FLAG_CALLER_ALLOWED),
            ParticleObserve::Repeat
        );
        assert_eq!(
            diag.observe_particle(hash, 0x25efe14, 351, FLAG_NAME_HIT | FLAG_CALLER_ALLOWED),
            ParticleObserve::First(351)
        );
        assert_eq!(
            diag.observe_particle(hash, 0x25efcdc, 3, FLAG_NAME_HIT | FLAG_CALLER_ALLOWED),
            ParticleObserve::First(3)
        );
        assert_eq!(
            diag.observe_particle(
                hash,
                0x25efcdc,
                351,
                FLAG_NAME_HIT | FLAG_CALLER_ALLOWED | FLAG_ROW_OFF
            ),
            ParticleObserve::First(351)
        );
        assert_eq!(
            diag.observe_particle(
                hash & HASH40_MASK,
                0x25efcdc,
                351,
                FLAG_NAME_HIT | FLAG_CALLER_ALLOWED
            ),
            ParticleObserve::Repeat
        );
        assert_eq!(
            diag.observe_particle(1, CALLER_UNREPRESENTABLE, -1, FLAG_CALLER_ALLOWED),
            ParticleObserve::First(-1)
        );
        assert_eq!(
            diag.observe_particle(1, 1, i32::from(i16::MAX) + 1, FLAG_NAME_HIT),
            ParticleObserve::StageUnrepresentable
        );
    }

    #[test]
    fn particle_cap_and_contention_skip_without_blocking() {
        let diag = Diagnostics::new();
        diag.reserve_particles();
        for index in 0..PARTICLE_CAPACITY as u32 {
            assert_eq!(
                diag.observe_particle(u64::from(index), index, 95, FLAG_NAME_HIT),
                ParticleObserve::First(95)
            );
        }
        assert_eq!(
            diag.observe_particle(u64::from(PARTICLE_CAPACITY as u32), 1, 95, FLAG_NAME_HIT),
            ParticleObserve::Full
        );
        assert_eq!(
            diag.observe_particle(0, 0, 95, FLAG_NAME_HIT),
            ParticleObserve::Repeat
        );
        let held = diag.particles.lock().unwrap();
        assert_eq!(
            diag.observe_particle(9, 9, 95, FLAG_STAGE_MATCH),
            ParticleObserve::Contended
        );
        assert!(diag.particles.try_lock().is_err());
        drop(held);
        assert_eq!(
            diag.observe_particle(9, 9, 95, FLAG_STAGE_MATCH),
            ParticleObserve::Full
        );
    }

    #[test]
    fn packed_particle_keeps_negative_stage_flags_and_caller_sentinel_apart() {
        let packed = pack_particle(-1, FLAG_NAME_HIT | FLAG_ROW_OFF, CALLER_UNREPRESENTABLE);
        assert_eq!(packed, 0xffff_0009_ffff_ffff);
        assert_eq!(
            unpack_particle(packed),
            (-1, FLAG_NAME_HIT | FLAG_ROW_OFF, CALLER_UNREPRESENTABLE)
        );
        let off = pack_particle(351, FLAG_ROW_OFF, 0x25efcdc);
        let on = pack_particle(351, 0, 0x25efcdc);
        assert_ne!(off, on);
        assert_eq!(unpack_particle(off).1, FLAG_ROW_OFF);
        assert_eq!(unpack_particle(on).1, 0);
        let hidden = ModelCount {
            stage_id: -1,
            reason: ModelReason::RequestedNativeHidden as u16,
            count: 4,
        };
        assert_eq!(hidden.argument_a(), 0xffff_ffff_0000_0006);
        assert_ne!(hidden.argument_a() & 0xffff, FLAG_NAME_HIT as u64);
    }
}
