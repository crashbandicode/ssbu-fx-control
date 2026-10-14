//! ssbu-fx-control: per-fighter character effects (On / Off / Reduced) for
//! Smash Ultimate 13.0.5 as one Skyline NRO.
//!
//! Portable policy, tables, settings and diagnostics compile and test on a
//! development host; the Skyline hooks live in `runtime` (switch target only).
pub mod acmd;
pub mod config;
pub mod config_store;
pub mod critical_probe;
pub mod diagnostics;
pub mod eff_cleanup;
pub mod eff_sibling;
pub mod eff_variant;
pub mod effect_context;
pub mod effects;
mod effects_catalog;
pub mod effects_reduced;
pub mod fighters;
pub mod menu;
pub mod persist;
pub mod policy;
pub mod scope;

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

#[cfg(target_os = "switch")]
mod runtime;
