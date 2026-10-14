//! Where "Scope: Online" gets its online signal.
//!
//! * [`OnlineSource::Host`]: Online Deluxe 0.5.0 or later exports
//!   `ssbu_online_deluxe_online_context_v1`. The policy applies while its
//!   `ONLINE` bit is set and the game is not in Training (the same test Deluxe
//!   used for its own FX: `online && !is_training_mode()`).
//! * [`OnlineSource::Own`]: no Deluxe (no `ssbu_online_deluxe_plugin_api_v1`)
//!   and no legacy SsbuSync host. This plugin installs its own copy of the
//!   mark-only online-latency hook (`0x16ccab8`) and clears the mark at every
//!   stage load (`0x25d9288`), so a later offline match is not treated as
//!   online. The latency hook fires about three seconds after an online
//!   Singles/Doubles stage loads, so the first seconds of each online match
//!   are outside the policy.
//! * [`OnlineSource::Unavailable`]: Deluxe (or SsbuSync) is present but does
//!   not export an online context. Installing this plugin's own hook would
//!   collide with that host's hook on the same instruction, so it is not
//!   installed. The policy applies everywhere and the Scope row shows
//!   `Unavailable`.
//! * [`OnlineSource::Undetected`]: no host, but the own hooks were refused
//!   (unexpected instruction words). Scope Online then never applies, the
//!   same conservative choice Deluxe made when its scene hooks failed.
use crate::config::FxScope;
use deluxe_plugin_sdk::abi::ONLINE_CONTEXT_ONLINE;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum OnlineSource {
    Host = 1,
    Own = 2,
    Unavailable = 3,
    Undetected = 4,
}

impl OnlineSource {
    pub const fn from_byte(byte: u8) -> Self {
        match byte {
            1 => Self::Host,
            2 => Self::Own,
            3 => Self::Unavailable,
            _ => Self::Undetected,
        }
    }

    /// The Scope row has a meaningful value.
    pub const fn scope_row_available(self) -> bool {
        !matches!(self, Self::Unavailable)
    }
}

/// Which source to use, from what was found at startup.
pub const fn select(api_present: bool, online_export: bool, legacy_host: bool) -> OnlineSource {
    if online_export {
        OnlineSource::Host
    } else if api_present || legacy_host {
        OnlineSource::Unavailable
    } else {
        OnlineSource::Own
    }
}

/// Whether the policy applies now.
///
/// `host_bits` is `host_online_context()`, `own_marked` the plugin's own
/// latency mark, `training` is `app::smashball::is_training_mode()`.
pub fn allows(
    scope: FxScope,
    source: OnlineSource,
    host_bits: Option<u32>,
    own_marked: bool,
    training: bool,
) -> bool {
    if scope == FxScope::All {
        return true;
    }
    match source {
        OnlineSource::Host => host_bits.unwrap_or(0) & ONLINE_CONTEXT_ONLINE != 0 && !training,
        OnlineSource::Own => own_marked && !training,
        OnlineSource::Unavailable => true,
        OnlineSource::Undetected => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deluxe_plugin_sdk::abi::{
        ONLINE_CONTEXT_MATCH, ONLINE_CONTEXT_READY, ONLINE_CONTEXT_TRAINING,
    };

    #[test]
    fn source_selection() {
        assert_eq!(select(true, true, false), OnlineSource::Host);
        assert_eq!(select(false, false, false), OnlineSource::Own);
        assert_eq!(select(true, false, false), OnlineSource::Unavailable);
        assert_eq!(select(false, false, true), OnlineSource::Unavailable);
        assert!(!OnlineSource::Unavailable.scope_row_available());
        for source in [
            OnlineSource::Host,
            OnlineSource::Own,
            OnlineSource::Unavailable,
            OnlineSource::Undetected,
        ] {
            assert_eq!(OnlineSource::from_byte(source as u8), source);
        }
    }

    #[test]
    fn online_scope_needs_online_and_not_training() {
        let online = ONLINE_CONTEXT_READY | ONLINE_CONTEXT_ONLINE | ONLINE_CONTEXT_MATCH;
        let host = OnlineSource::Host;
        assert!(allows(FxScope::Online, host, Some(online), false, false));
        assert!(!allows(FxScope::Online, host, Some(online), false, true));
        assert!(!allows(
            FxScope::Online,
            host,
            Some(ONLINE_CONTEXT_READY | ONLINE_CONTEXT_MATCH),
            true,
            false
        ));
        assert!(!allows(
            FxScope::Online,
            host,
            Some(ONLINE_CONTEXT_READY | ONLINE_CONTEXT_TRAINING),
            false,
            false
        ));
        assert!(!allows(FxScope::Online, host, None, true, false));
        let own = OnlineSource::Own;
        assert!(allows(FxScope::Online, own, None, true, false));
        assert!(!allows(FxScope::Online, own, None, true, true));
        assert!(!allows(FxScope::Online, own, Some(online), false, false));
        assert!(allows(
            FxScope::Online,
            OnlineSource::Unavailable,
            None,
            false,
            true
        ));
        assert!(!allows(
            FxScope::Online,
            OnlineSource::Undetected,
            None,
            true,
            false
        ));
        for source in [
            OnlineSource::Host,
            OnlineSource::Own,
            OnlineSource::Unavailable,
            OnlineSource::Undetected,
        ] {
            assert!(allows(FxScope::All, source, None, false, true));
        }
    }
}
