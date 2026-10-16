//! Skyline entry point and game hooks (switch target only).
//!
//! Startup order: install log, version check, INI load, online-source
//! detection, hooks (unless the `DISABLE` marker exists), diagnostics thread,
//! then menu registration with Online Deluxe when it is installed.
mod acmd;
mod effect_context;
mod effects;
mod logging;
mod memory;
mod online;
mod plugin;

use crate::{
    diagnostics::{Code, Recorder},
    policy::ModeTable,
};

pub(super) static RECORDER: Recorder = Recorder::new();
pub(super) static TABLE: ModeTable = ModeTable::new();
/// `TRACE_EFFECTS` harness trace (code 110), latched at install.
pub(super) static TRACE: crate::trace::Trace = crate::trace::Trace::new();
pub(super) const DATA_DIR: &str = "sd:/ultimate/ssbu_fx_control";
/// Marker file under [`DATA_DIR`] that skips every hook.
const DISABLE_MARKER: &str = "DISABLE";
/// Marker file under [`DATA_DIR`] that turns on the request trace.
pub(super) const TRACE_MARKER: &str = "TRACE_EFFECTS";

pub(super) fn record(code: Code, a: u64, b: u64) {
    let tick = unsafe { skyline::nn::os::GetSystemTick() as u64 };
    RECORDER.record(tick, code, a, b);
    // Startup-only checkpoints reach disk before/after each patch, so an
    // install crash is diagnosable before the background sink first wakes.
    if matches!(
        code,
        Code::HookInstalling | Code::HookInstalled | Code::HookRejected
    ) {
        logging::install_checkpoint(code, a, b);
    }
}

/// Whether the per-fighter policy applies to a request right now.
pub(super) fn scope_allows() -> bool {
    online::allows(TABLE.scope())
}

fn display_version() -> String {
    let mut version = skyline::nn::oe::DisplayVersion { name: [0; 16] };
    unsafe {
        skyline::nn::oe::GetDisplayVersion(&mut version);
    }
    let end = version
        .name
        .iter()
        .position(|v| *v == 0)
        .unwrap_or(version.name.len());
    String::from_utf8_lossy(&version.name[..end]).into_owned()
}

#[skyline::main(name = "ssbu-fx-control")]
pub fn main() {
    logging::prepare();
    logging::startup_phase("version_query_begin");
    let version = display_version();
    logging::startup_phase("version_query_complete");
    let source = online::detect();
    logging::start(&version, source);
    if version != "13.0.5" {
        record(Code::UnsupportedVersion, 0, 0);
        logging::startup_phase("unsupported_version_hooks_skipped");
        skyline::println!("[ssbu-fx-control] Unsupported Smash version {version}; hooks skipped");
    } else if std::path::Path::new(DATA_DIR).join(DISABLE_MARKER).exists() {
        record(Code::Compatibility, 2, 0);
        logging::startup_phase("disable_marker_hooks_skipped");
        skyline::println!("[ssbu-fx-control] DISABLE marker present; hooks skipped");
    } else {
        logging::startup_phase("hooks_begin");
        effects::install();
        online::install(source);
        logging::startup_phase("hooks_complete");
        // Rows only when the hooks behind them exist.
        plugin::register();
    }
    logging::startup_phase("main_complete");
}
