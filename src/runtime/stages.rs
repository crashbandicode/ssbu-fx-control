//! Stage-specific particle requests and model draw-record visibility.
//! 13.0.5 static evidence: docs/STAGES.md. Lifecycle observers keep a bounded
//! resource association; only the final draw-record call changes visibility.
use super::{logging, memory, record, Code, DATA_DIR, TABLE};
use crate::{
    effects,
    stage_diag::{self, ModelCall, ParticleObserve, DIAG},
    stage_models::{Lookup, Registry},
    stages,
};
use skyline::hooks::{getRegionAddress, InlineCtx, Region};
use std::sync::atomic::{AtomicU16, AtomicU64, AtomicUsize, Ordering};

// lib::EffectManager request: x1 is masked to 40 bits at +0x68 and looked up
// before any effect allocation; an unknown handle returns zero at +0x22c.
const REQUEST: usize = 0x3563430;
const ENTRY_WORDS: [u32; 8] = [
    0xd10203ff, 0xa9026ffc, 0xa90367fa, 0xa9045ff8, 0xa90557f6, 0xa9064ff4, 0xa9077bfd, 0x9101c3fd,
];
const MODEL_CALL: usize = 0x35d73e0;
const MODEL_CAPTURE: usize = 0x35e26a0;
const MODEL_CONSTRUCT: usize = 0x35c32d0;
const MODEL_DESTRUCT: usize = 0x35c3ab0;
const MODEL_INITIALIZE: usize = 0x35c7aa0;
const MODEL_COPY: usize = 0x35d0e30;
const MODEL_REPLACE: usize = 0x35f73e0;
const MODEL_VTABLE: usize = 0x5232670;
const RESOURCE_MANAGER: usize = 0x5332f20;
static MODEL_REGISTRY: Registry = Registry::new();
static AVAILABLE: AtomicU16 = AtomicU16::new(0);
static SUPPRESSED: AtomicU64 = AtomicU64::new(0);
static MODELS_SUPPRESSED: AtomicU64 = AtomicU64::new(0);
static TEXT_BASE: AtomicUsize = AtomicUsize::new(0);

extern "C" {
    // Unlike app::stage::get_stage_id, this wrapper checks the singleton and
    // its inactive-stage sentinel, returning -1 when no stage is active.
    #[link_name = "\u{1}_ZN3app14sv_information8stage_idEv"]
    fn current_stage_id() -> i32;
}

pub(super) fn available() -> u16 {
    AVAILABLE.load(Ordering::Acquire)
}

/// Predicate bits only. Does not change the request register.
/// Returns the stage id when this call already read it, so the decision
/// path below does not read it a second time.
unsafe fn note_particle_predicate(
    hash: u64,
    name_index: Option<usize>,
    caller: Option<usize>,
    off: u16,
) -> Option<i32> {
    let caller_allowed = caller.is_some_and(stages::stage_caller);
    if name_index.is_none() && !caller_allowed {
        return None;
    }
    let id = current_stage_id();
    let mut flags = 0u16;
    if name_index.is_some() {
        flags |= stage_diag::FLAG_NAME_HIT;
    }
    if caller_allowed {
        flags |= stage_diag::FLAG_CALLER_ALLOWED;
    }
    if let Some(index) = name_index {
        if stages::ALL[index].game_id == id {
            flags |= stage_diag::FLAG_STAGE_MATCH;
        }
        if off & (1 << index) != 0 {
            flags |= stage_diag::FLAG_ROW_OFF;
        }
    }
    let caller32 = match caller {
        Some(value) => u32::try_from(value).unwrap_or(stage_diag::CALLER_UNREPRESENTABLE),
        None => stage_diag::CALLER_UNREPRESENTABLE,
    };
    if let ParticleObserve::First(stage) = DIAG.observe_particle(hash, caller32, id, flags) {
        record(
            Code::FxStageParticleObserve,
            hash & stage_diag::HASH40_MASK,
            stage_diag::pack_particle(stage, flags, caller32),
        );
    }
    Some(id)
}

#[skyline::hook(offset = REQUEST, inline)]
unsafe fn ssbu_fx_control_stage_background(ctx: &mut InlineCtx) {
    let off = TABLE.stage_off_mask();
    let trace = super::TRACE.enabled();
    let diag = DIAG.enabled();
    if off & stages::PARTICLE_MASK == 0 && !trace && !diag {
        return;
    }
    let original = ctx.registers[1].x();
    let name_index = stages::background_stage(original);
    let caller = (ctx.registers[30].x() as usize).checked_sub(TEXT_BASE.load(Ordering::Relaxed));
    let observed_stage = if diag {
        note_particle_predicate(original, name_index, caller, off)
    } else {
        None
    };
    let Some(index) = name_index else {
        return;
    };
    let Some(caller) = caller else {
        return;
    };
    if !stages::stage_caller(caller) || (off & (1 << index) == 0 && !trace) {
        return;
    }
    let id = observed_stage.unwrap_or_else(|| current_stage_id());
    if stages::ALL[index].game_id != id {
        return;
    }
    if trace {
        let key = crate::trace::Key {
            hash: original,
            category: 0xfe,
            kind: id,
            status: -1,
        };
        if let Some(ordinal) = super::TRACE.first_seen(key) {
            record(
                Code::FxStageTrace,
                original,
                ((id as u64) << 32) | u64::from(ordinal),
            );
        }
    }
    if !stages::suppress(id, original, off, caller) {
        return;
    }
    ctx.registers[1].set_x(effects::NULL_EFFECT);
    let count = SUPPRESSED.fetch_add(1, Ordering::Relaxed) + 1;
    if count <= 16 || count.is_multiple_of(128) {
        record(
            Code::FxStageSuppressed,
            original,
            ((id as u64) << 32) | (count & 0xffff_ffff),
        );
    }
}

fn latch_stage_diagnostics() {
    let present = std::path::Path::new(DATA_DIR)
        .join(stage_diag::MARKER)
        .exists();
    if present {
        DIAG.reserve_particles();
    }
    DIAG.latch(present);
    record(Code::FxStageDiagnostics, u64::from(present), 0);
    let mut lines = vec![format!("stage_diag enabled={}", u8::from(present))];
    for (code, name) in stage_diag::MODEL_REASONS {
        lines.push(format!("stage_diag_reason {code} {name}"));
    }
    lines.push("stage_diag_flags name-hit=1 caller-allowed=2 stage-match=4 row-off=8".to_owned());
    lines.push(
        "stage_diag_particle cap=512 caller_unrepresentable=0xffffffff first-seen-only".to_owned(),
    );
    lines.push("stage_diag_model_counts observed-calls-not-frames-or-fps".to_owned());
    logging::note_install(&lines);
}

pub(super) fn install() {
    latch_stage_diagnostics();
    let base = unsafe { getRegionAddress(Region::Text) as usize };
    let end = unsafe { getRegionAddress(Region::Rodata) as usize };
    TEXT_BASE.store(base, Ordering::Release);
    install_particles(base, end);
    install_models(base, end);
}

fn install_particles(base: usize, end: usize) {
    let Some(address) = base.checked_add(REQUEST) else {
        return;
    };
    let bytes = 0x23c;
    if address.checked_add(bytes).is_none_or(|stop| stop > end)
        || !unsafe { memory::accessible(address, bytes, false) }
    {
        record(Code::HookRejected, REQUEST as u64, 0);
        return;
    }
    for (index, expected) in ENTRY_WORDS.iter().enumerate() {
        let actual = unsafe { *((address + index * 4) as *const u32) };
        if actual != *expected {
            record(
                Code::HookRejected,
                (REQUEST + index * 4) as u64,
                actual as u64,
            );
            return;
        }
    }
    // Semantic guard in addition to the prologue: saved/masked hash and the
    // no-resource zero-result path. Reject other builds/overlapping patches.
    for (offset, expected) in [
        (0x4c, 0xaa0103fb),
        (0x68, 0x92409f68),
        (0x22c, 0x2a1f03fc),
        (0x238, 0x2a1c03e0),
    ] {
        let actual = unsafe { *((address + offset) as *const u32) };
        if actual != expected {
            record(Code::HookRejected, (REQUEST + offset) as u64, actual as u64);
            return;
        }
    }
    logging::install_checkpoint(Code::HookEntryWord, REQUEST as u64, ENTRY_WORDS[0] as u64);
    record(Code::HookInstalling, REQUEST as u64, address as u64);
    skyline::install_hook!(ssbu_fx_control_stage_background);
    AVAILABLE.fetch_or(stages::PARTICLE_MASK, Ordering::Release);
    record(Code::HookInstalled, REQUEST as u64, address as u64);
}

#[skyline::hook(offset = MODEL_CALL, inline)]
unsafe fn ssbu_fx_control_stage_model(ctx: &mut InlineCtx) {
    let off = TABLE.stage_off_mask();
    let trace = super::TRACE.enabled();
    let diag = DIAG.enabled();
    if off & stages::MODEL_MASK == 0 && !trace && !diag {
        return;
    }
    let renderer = ctx.registers[19].x() as usize;
    let lookup = MODEL_REGISTRY.lookup(renderer);
    let path = match lookup {
        Lookup::Matched(path) => path,
        other => {
            if diag {
                DIAG.tally(
                    current_stage_id(),
                    ModelCall {
                        lookup: other,
                        vtable_ok: false,
                        target: false,
                        requested: false,
                        leave_visible: false,
                    },
                );
            }
            return;
        }
    };
    // x19 is a live renderer already being used by the pinned native consumer.
    // No mapping queries, resource traversal or retained native references here.
    if *(renderer as *const usize) != TEXT_BASE.load(Ordering::Relaxed) + MODEL_VTABLE {
        if diag {
            DIAG.tally(
                current_stage_id(),
                ModelCall {
                    lookup,
                    vtable_ok: false,
                    target: false,
                    requested: false,
                    leave_visible: false,
                },
            );
        }
        return;
    }
    let id = current_stage_id();
    let Some(model) = stages::background_model(id, path) else {
        if diag {
            DIAG.tally(
                id,
                ModelCall {
                    lookup,
                    vtable_ok: true,
                    target: false,
                    requested: false,
                    leave_visible: false,
                },
            );
        }
        return;
    };
    if diag {
        let requested = ctx.registers[1].w() & 1 != 0;
        let leave_visible = !requested || stages::model_visibility(id, path, off, requested);
        DIAG.tally(
            id,
            ModelCall {
                lookup,
                vtable_ok: true,
                target: true,
                requested,
                leave_visible,
            },
        );
    }
    if off & (1 << model.stage) == 0 && !trace {
        return;
    }
    if trace {
        let key = crate::trace::Key {
            hash: path,
            category: 0xfd,
            kind: id,
            status: -1,
        };
        if let Some(ordinal) = super::TRACE.first_seen(key) {
            record(
                Code::FxStageModelTrace,
                path,
                ((id as u64) << 32) | u64::from(ordinal),
            );
        }
    }
    let requested = ctx.registers[1].w() & 1 != 0;
    if !requested || stages::model_visibility(id, path, off, requested) {
        return;
    }
    // Requested +4b0 and effective +4b1 have already been left entirely native.
    // Only the argument to the per-renderer draw-record update becomes false.
    ctx.registers[1].set_x(0);
    let count = MODELS_SUPPRESSED.fetch_add(1, Ordering::Relaxed) + 1;
    if count <= 16 || count.is_multiple_of(128) {
        record(
            Code::FxStageModelSuppressed,
            path,
            ((id as u64) << 32) | (count & 0xffff_ffff),
        );
    }
}

fn registry_result(healthy: bool) {
    if !healthy
        && AVAILABLE.fetch_and(!stages::MODEL_MASK, Ordering::AcqRel) & stages::MODEL_MASK != 0
    {
        record(Code::HookRejected, MODEL_CALL as u64, 0x524547);
    }
}

unsafe fn invalidate_model(ctx: &InlineCtx) {
    registry_result(MODEL_REGISTRY.invalidate(ctx.registers[0].x() as usize));
}

#[skyline::hook(offset = MODEL_CONSTRUCT, inline)]
unsafe fn ssbu_fx_control_stage_model_construct(ctx: &mut InlineCtx) {
    invalidate_model(ctx);
}

#[skyline::hook(offset = MODEL_DESTRUCT, inline)]
unsafe fn ssbu_fx_control_stage_model_destruct(ctx: &mut InlineCtx) {
    invalidate_model(ctx);
}

#[skyline::hook(offset = MODEL_INITIALIZE, inline)]
unsafe fn ssbu_fx_control_stage_model_initialize(ctx: &mut InlineCtx) {
    invalidate_model(ctx);
}

#[skyline::hook(offset = MODEL_COPY, inline)]
unsafe fn ssbu_fx_control_stage_model_copy(ctx: &mut InlineCtx) {
    invalidate_model(ctx);
}

#[skyline::hook(offset = MODEL_REPLACE, inline)]
unsafe fn ssbu_fx_control_stage_model_replace(ctx: &mut InlineCtx) {
    invalidate_model(ctx);
}

unsafe fn native_field<T: Copy>(object: usize, offset: usize) -> Option<T> {
    let address = object.checked_add(offset)?;
    if !memory::accessible(address, core::mem::size_of::<T>(), false) {
        return None;
    }
    Some((address as *const T).read_unaligned())
}

/// Mirrors the bounded SearchLookup parent resolution in 0x35c26e8..0x35c2750.
/// This input is a search-path index, NOT a LoadedArc DirInfo index. The optional
/// path-list-indices mapping must be applied before reading the 0x20-byte entry.
unsafe fn model_resource_parent(index: u32) -> Option<u64> {
    let mut index = index as usize;
    if index == 0xff_ffff {
        return None;
    }
    let manager = native_field::<usize>(TEXT_BASE.load(Ordering::Relaxed), RESOURCE_MANAGER)?;
    let tables = native_field::<usize>(manager, 0x78)?;
    let search = native_field::<usize>(tables, 8)?;
    let header = native_field::<usize>(search, 8)?;
    if index >= native_field::<u32>(header, 4)? as usize {
        return None;
    }
    let indices = native_field::<usize>(search, 0x28)?;
    if indices != 0 {
        index = native_field::<u32>(indices, index.checked_mul(4)?)? as usize;
    }
    if index == 0xff_ffff || index >= native_field::<u32>(header, 8)? as usize {
        return None;
    }
    let paths = native_field::<usize>(search, 0x30)?;
    let entry = native_field::<[u64; 4]>(paths, index.checked_mul(0x20)?)?;
    if entry[2] & 0xff_ffff_ffff != effects::hash40("model.numdlb")
        || entry[3] & 0xff_ffff_ffff != effects::hash40("numdlb")
    {
        return None;
    }
    Some(entry[1] & 0xff_ffff_ffff)
}

#[skyline::hook(offset = MODEL_CAPTURE, inline)]
unsafe fn ssbu_fx_control_stage_model_capture(ctx: &mut InlineCtx) {
    if !MODEL_REGISTRY.healthy() {
        return;
    }
    // The primary constructor's initial binding has returned. x22 is the live
    // renderer and w23 the search index native validated, both callee-preserved.
    let renderer = ctx.registers[22].x() as usize;
    let base = TEXT_BASE.load(Ordering::Relaxed);
    if native_field::<usize>(renderer, 0) != Some(base + MODEL_VTABLE)
        || native_field::<usize>(base + MODEL_VTABLE, 0x48) != Some(base + MODEL_INITIALIZE)
    {
        return;
    }
    let Some(path) = model_resource_parent(ctx.registers[23].w()) else {
        return;
    };
    if stages::MODELS.iter().any(|model| model.path == path) {
        registry_result(MODEL_REGISTRY.observe(renderer, path));
    }
}

fn guard_words(base: usize, end: usize, offset: usize, words: &[u32]) -> bool {
    let Some(address) = base.checked_add(offset) else {
        return false;
    };
    let bytes = words.len() * 4;
    if address.checked_add(bytes).is_none_or(|stop| stop > end)
        || !unsafe { memory::accessible(address, bytes, false) }
    {
        record(Code::HookRejected, offset as u64, 0);
        return false;
    }
    for (index, expected) in words.iter().enumerate() {
        let actual = unsafe { *((address + index * 4) as *const u32) };
        if actual != *expected {
            record(
                Code::HookRejected,
                (offset + index * 4) as u64,
                actual as u64,
            );
            return false;
        }
    }
    true
}

fn install_models(base: usize, end: usize) {
    // All observers are required: missing a lifetime/rebind boundary would make
    // an address association unsafe. Pin complete maximum inline patch spans,
    // constructor identity flow, search-parent lookup and draw-only argument.
    for (offset, words) in [
        (
            MODEL_CONSTRUCT,
            &[0xd10203ff, 0xf9002bf5, 0xa9064ff4, 0xa9077bfd, 0x9101c3fd][..],
        ),
        (
            MODEL_DESTRUCT,
            &[0xd10283ff, 0xa9046ffc, 0xa90567fa, 0xa9065ff8, 0xa90757f6][..],
        ),
        (
            MODEL_INITIALIZE,
            &[
                0xa9ba6ffc, 0xa90167fa, 0xa9025ff8, 0xa90357f6, 0xa9044ff4, 0xa9057bfd, 0x910143fd,
                0xd11043ff, 0xb9401028,
            ][..],
        ),
        (
            MODEL_COPY,
            &[
                0xa9ba6ffc, 0xa90167fa, 0xa9025ff8, 0xa90357f6, 0xa9044ff4, 0xa9057bfd, 0x910143fd,
                0xd10743ff, 0xa9402428,
            ][..],
        ),
        (
            MODEL_REPLACE,
            &[0xd10503ff, 0xa90e6ffc, 0xa90f67fa, 0xa9105ff8, 0xa91157f6][..],
        ),
        (
            0x35e0e70,
            &[
                0xaa0103f8, 0x2a0203e1, 0x2a0303f6, 0x2a0203f5, 0xaa0003f3, 0x97ff8913,
            ][..],
        ),
        (0x35e0f04, &[0xf90017f3][..]),
        (0x35e1174, &[0xf9000ff8][..]),
        (
            0x35e265c,
            &[
                0xf9400ff8, 0xb9400317, 0xd0006a21, 0x9121bc21, 0x2a1703e0, 0x97ff7fe0, 0x32005fe8,
                0x6b08001f, 0x540012c0,
            ][..],
        ),
        (
            0x35e2680,
            &[
                0xf94017f6, 0xf94246c8, 0xf9400101, 0xf94002c8, 0xf9402508, 0xaa1603e0, 0xaa1803e2,
                0xd63f0100, 0xf94246c8, 0xf9400108, 0xb9401108, 0x71001d1f, 0x54001288, 0x320003e9,
            ][..],
        ),
        (
            0x35c26e8,
            &[
                0xf9479108, 0xf9403d08, 0x32005fea, 0x6b0a001f, 0x540002c0, 0xf940050a, 0xf940054b,
                0xb940056c, 0x6b00019f, 0x54000229, 0xf940154c, 0xb400004c, 0xb8605980, 0x32005fec,
                0x6b0c001f, 0x54000160, 0xb940096b, 0x6b00017f, 0x54000109, 0xf940194a, 0x2a0003eb,
                0xab0b154a, 0x54000080, 0xf940054a, 0x92409d4c, 0x14000002, 0xaa1f03ec,
            ][..],
        ),
        (
            0x35d70f0,
            &[
                0x3952c268, 0x395e0e69, 0x7100011f, 0x1a9f07f4, 0x34000109, 0xb9404a69, 0x7100011f,
                0x52a0ecaa, 0x1a9f07e8, 0x6a29015f, 0x1a9f07e9, 0x0a090114, 0xf9424275, 0x3912c674,
            ][..],
        ),
        (
            0x35d73d8,
            &[
                0xf9424260, 0x2a1403e1, 0x94008ddc, 0xf9422268, 0x39c00508, 0x71000d1f, 0x5400036b,
            ][..],
        ),
        (
            0x35fab50,
            &[
                0xf81c0ff7, 0xa90157f6, 0xa9024ff4, 0xa9037bfd, 0x9100c3fd, 0xb9401808, 0xf9400009,
                0x321c0bea, 0x9b0a2509, 0xa9405528, 0x2a0103f4, 0xaa0003f3,
            ][..],
        ),
    ] {
        if !guard_words(base, end, offset, words) {
            return;
        }
    }
    logging::install_checkpoint(Code::HookEntryWord, MODEL_CALL as u64, 0x94008ddc);
    record(
        Code::HookInstalling,
        MODEL_CALL as u64,
        (base + MODEL_CALL) as u64,
    );
    skyline::install_hooks!(
        ssbu_fx_control_stage_model_construct,
        ssbu_fx_control_stage_model_destruct,
        ssbu_fx_control_stage_model_initialize,
        ssbu_fx_control_stage_model_copy,
        ssbu_fx_control_stage_model_replace,
        ssbu_fx_control_stage_model_capture,
        ssbu_fx_control_stage_model
    );
    AVAILABLE.fetch_or(stages::MODEL_MASK, Ordering::Release);
    record(
        Code::HookInstalled,
        MODEL_CALL as u64,
        (base + MODEL_CALL) as u64,
    );
}
