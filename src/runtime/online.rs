//! The online signal behind "Scope: Online" (see `crate::scope`).
//!
//! With Online Deluxe 0.5.0 or later the plugin reads Deluxe's exported scene
//! context and installs no scene hook of its own. Without Deluxe it installs
//! two mark-only inline hooks, both from Online Deluxe 0.5.0
//! (`src/runtime/hooks.rs`, `src/runtime/online.rs`), source-verified for 13.0.5:
//!
//! * `0x16ccab8` online-latency setup: sets the mark. Nothing is written to
//!   the game; Deluxe's input-buffer mutation is not ported.
//! * `0x25d9288` stage setup: clears the mark, so an offline match after an
//!   online one is offline again. The stage id is not read.
//!
//! Neither hook is installed while Deluxe (any version) or a legacy SsbuSync
//! host is loaded: two inline hooks on one instruction collide.
use super::{logging, memory, record, Code};
use crate::config::FxScope;
use crate::scope::{self, OnlineSource};
use deluxe_plugin_sdk::abi;
use skyline::hooks::{getRegionAddress, InlineCtx, Region};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

extern "C" {
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    fn is_training_mode() -> bool;
}

const ONLINE_LATENCY: usize = 0x16ccab8;
const STAGE_SETUP: usize = 0x25d9288;
/// Words at `STAGE_SETUP`, `+4` and `+8` on 13.0.5 (Deluxe's guard).
const STAGE_WORDS: [u32; 3] = [0xaa0003f8, 0xd63f0120, 0xf940e316];

static SOURCE: AtomicU8 = AtomicU8::new(OnlineSource::Undetected as u8);
static MARK: AtomicBool = AtomicBool::new(false);

fn symbol_present(name: &[u8]) -> bool {
    let mut address = 0usize;
    unsafe {
        skyline::nn::ro::LookupSymbol(&mut address, name.as_ptr());
    }
    address != 0
}

/// Chooses the online source. Skyline loads every plugin NRO before any
/// `main`, so Deluxe's exports resolve here in any load order.
pub(super) fn detect() -> OnlineSource {
    let api_present = symbol_present(abi::API_SYMBOL_C);
    let online_export = deluxe_plugin_sdk::host_online_context().is_some();
    let legacy_host = symbol_present(b"ssbusync_status\0")
        || symbol_present(b"ssbusync_restrict_mark_arena_mode\0");
    if legacy_host {
        record(Code::Compatibility, 1, 0);
    }
    let source = scope::select(api_present, online_export, legacy_host);
    if source == OnlineSource::Unavailable {
        record(Code::Compatibility, 3, u64::from(api_present));
        skyline::println!(
            "[ssbu-fx-control] host without an online-context export; Scope unavailable, Online scope never applies (set scope=all to filter)"
        );
    }
    SOURCE.store(source as u8, Ordering::Release);
    source
}

pub(super) fn source() -> OnlineSource {
    OnlineSource::from_byte(SOURCE.load(Ordering::Relaxed))
}

/// Whether the policy applies now. Training is only asked when the online
/// part already holds.
pub(super) fn allows(scope_setting: FxScope) -> bool {
    if scope_setting == FxScope::All {
        return true;
    }
    let source = source();
    let bits = match source {
        OnlineSource::Host => deluxe_plugin_sdk::host_online_context(),
        _ => None,
    };
    let marked = MARK.load(Ordering::Relaxed);
    let online_hint = match source {
        OnlineSource::Host => bits.unwrap_or(0) & abi::ONLINE_CONTEXT_ONLINE != 0,
        OnlineSource::Own => marked,
        OnlineSource::Unavailable | OnlineSource::Undetected => false,
    };
    let training = online_hint && unsafe { is_training_mode() };
    scope::allows(scope_setting, source, bits, marked, training)
}

// This detects common trampolines, NOT an original-instruction signature.
unsafe fn preflight(offset: usize) -> bool {
    let base = getRegionAddress(Region::Text) as usize;
    let end = getRegionAddress(Region::Rodata) as usize;
    let Some(address) = base.checked_add(offset) else {
        return false;
    };
    if address % 4 != 0 || address.checked_add(20).is_none_or(|v| v > end) {
        record(Code::HookRejected, offset as u64, 1);
        return false;
    }
    if !memory::accessible(address, 4, false) {
        record(Code::HookRejected, offset as u64, 2);
        return false;
    }
    let word = *(address as *const u32);
    record(Code::HookEntryWord, offset as u64, word as u64);
    let branch = word & 0x7c00_0000 == 0x1400_0000;
    let indirect = word & 0xffff_fc1f == 0xd61f_0000;
    let literal = word & 0xff00_0000 == 0x5800_0000;
    let call = word & 0xfc00_0000 == 0x9400_0000;
    if (branch && !call) || indirect || literal || word == 0 {
        record(Code::HookRejected, offset as u64, word as u64);
        return false;
    }
    true
}

unsafe fn expect(offset: usize, expected: u32) -> bool {
    let address = getRegionAddress(Region::Text) as usize + offset;
    if !memory::accessible(address, 4, false) {
        record(Code::HookRejected, offset as u64, 0);
        return false;
    }
    let actual = *(address as *const u32);
    record(Code::HookEntryWord, offset as u64, actual as u64);
    logging::install_checkpoint(Code::HookEntryWord, offset as u64, actual as u64);
    if actual != expected {
        record(Code::HookRejected, offset as u64, actual as u64);
        return false;
    }
    true
}

#[skyline::hook(offset = ONLINE_LATENCY, inline)]
unsafe fn ssbu_fx_control_online_latency(_: &InlineCtx) {
    if !MARK.swap(true, Ordering::AcqRel) {
        record(Code::FxOnlineContext, OnlineSource::Own as u64, 1);
    }
}

#[skyline::hook(offset = STAGE_SETUP, inline)]
unsafe fn ssbu_fx_control_stage_setup(_: &InlineCtx) {
    if MARK.swap(false, Ordering::AcqRel) {
        record(Code::FxOnlineContext, OnlineSource::Own as u64, 2);
    }
}

/// Installs the own hooks when no host owns those sites. Both or neither:
/// a mark that is never cleared would leak into offline matches.
pub(super) fn install(source: OnlineSource) {
    if source != OnlineSource::Own {
        return;
    }
    let ready = unsafe {
        preflight(ONLINE_LATENCY)
            && STAGE_WORDS
                .iter()
                .enumerate()
                .all(|(index, word)| expect(STAGE_SETUP + index * 4, *word))
    };
    if !ready {
        SOURCE.store(OnlineSource::Undetected as u8, Ordering::Release);
        record(Code::Compatibility, 4, STAGE_SETUP as u64);
        logging::install_checkpoint(Code::Compatibility, 4, STAGE_SETUP as u64);
        return;
    }
    record(Code::HookInstalling, ONLINE_LATENCY as u64, 0);
    record(Code::HookInstalling, STAGE_SETUP as u64, 0);
    skyline::install_hooks!(ssbu_fx_control_online_latency, ssbu_fx_control_stage_setup);
    record(Code::HookInstalled, ONLINE_LATENCY as u64, 0);
    record(Code::HookInstalled, STAGE_SETUP as u64, 0);
}
