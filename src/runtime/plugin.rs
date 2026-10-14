//! Menu rows on the Online Deluxe window, when Deluxe 0.5.0 or later is loaded.
//!
//! Callbacks run on Deluxe's overlay draw thread: they only read and write
//! the lock-free mode table and the EFF availability atomics.
use super::{logging, online, record, Code, TABLE};
use crate::eff_variant;
use crate::menu;
use deluxe_plugin_sdk::{
    negotiated_descriptor_version, register as sdk_register, PluginHandler, RegisterError,
    RequestStatus,
};

struct Handler;

fn variant_status() -> eff_variant::VariantAvailability {
    menu::variant_status(&[
        eff_variant::availability(),
        eff_variant::edge_availability(),
        eff_variant::ink_availability(),
        eff_variant::chrom_availability(),
    ])
}

impl PluginHandler for Handler {
    fn current(&self, option: usize) -> Option<usize> {
        menu::current(
            &TABLE,
            option,
            online::source().scope_row_available(),
            variant_status(),
        )
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
    let (a, b) = match result {
        Ok(handle) => (
            handle.0 as i64 as u64,
            u64::from(negotiated_descriptor_version(handle).unwrap_or(0)),
        ),
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
    record(Code::PluginRegistered, a, b);
    logging::install_checkpoint(Code::PluginRegistered, a, b);
    skyline::println!("[ssbu-fx-control] menu registration: {result:?} (descriptor {b})");
}
