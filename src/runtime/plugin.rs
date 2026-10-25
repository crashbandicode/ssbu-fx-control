//! Menu rows on the Online Deluxe window, when Deluxe 0.5.3 or later is loaded
//! (SDK 5.0.0 registers with descriptor version 4 only; older hosts refuse it).
//!
//! Callbacks run on Deluxe's overlay draw thread: they only read and write
//! the lock-free mode table.
use super::{logging, online, record, Code, TABLE};
use crate::menu;
use deluxe_plugin_sdk::{register as sdk_register, PluginHandler, RegisterError, RequestStatus};

struct Handler;

struct StageHandler;

impl PluginHandler for StageHandler {
    fn current(&self, option: usize) -> Option<usize> {
        crate::stage_menu::current(&TABLE, option, super::stages::available())
    }

    fn request(&self, option: usize, choice: usize) -> RequestStatus {
        let status = crate::stage_menu::request(&TABLE, option, choice, super::stages::available());
        if status == RequestStatus::Applied {
            record(Code::StageMenuChanged, option as u64, choice as u64);
        }
        status
    }
}

impl PluginHandler for Handler {
    fn current(&self, option: usize) -> Option<usize> {
        menu::current(&TABLE, option, online::source().scope_row_available())
    }

    fn request(&self, option: usize, choice: usize) -> RequestStatus {
        let status = menu::request(
            &TABLE,
            option,
            choice,
            online::source().scope_row_available(),
        );
        if status == RequestStatus::Applied {
            record(Code::MenuChanged, option as u64, choice as u64);
        }
        status
    }
}

pub(super) fn register() {
    let result = sdk_register(&menu::SPEC, Handler);
    note_registration(Code::PluginRegistered, "FX Control", result);
    let result = sdk_register(&crate::stage_menu::SPEC, StageHandler);
    note_registration(Code::StagesRegistered, "Stages", result);
}

fn note_registration(
    code: Code,
    label: &str,
    result: Result<deluxe_plugin_sdk::Handle, RegisterError>,
) {
    let (a, b) = match result {
        // SDK 5 sends descriptor version 4 only; `b` is that version on success.
        Ok(handle) => (handle.0 as i64 as u64, 4),
        Err(RegisterError::HostMissing) => (-100i64 as u64, 0),
        Err(RegisterError::HostIncompatible) => (-101i64 as u64, 0),
        Err(RegisterError::Null) => (-1i64 as u64, 0),
        Err(RegisterError::Abi) => (-2i64 as u64, 0),
        Err(RegisterError::Invalid) => (-3i64 as u64, 0),
        Err(RegisterError::Full) => (-4i64 as u64, 0),
        Err(RegisterError::Duplicate) => (-5i64 as u64, 0),
        Err(RegisterError::Callbacks) => (-6i64 as u64, 0),
        Err(RegisterError::Unknown(code)) => (code as i64 as u64, 0),
    };
    record(code, a, b);
    logging::install_checkpoint(code, a, b);
    skyline::println!("[ssbu-fx-control] {label} menu registration: {result:?} (descriptor {b})");
}
