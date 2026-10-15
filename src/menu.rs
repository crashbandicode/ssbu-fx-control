//! Rows on the Online Deluxe window (plugin SDK, descriptor version 3).
//!
//! The plugin asks for its own page in Deluxe's window cycle. Rows, in order:
//! `Scope` (Online / All), `All` (On / Off / Reduced), one row per covered
//! fighter (On / Off / Reduced, or On / Off for a fighter without a Reduced
//! variant) and a read-only `EFF variants` status row.
//!
//! "All" has no fourth "Mixed" choice: while the fighter rows differ, its
//! `current` answers `CHOICE_UNAVAILABLE`, which the host shows as
//! `Unavailable`. Moving it to a choice sets every row.
use crate::config::{FxMode, FxScope};
use crate::eff_variant::VariantAvailability;
use crate::fighters::{self, Fighter, COUNT};
use crate::policy::ModeTable;
use deluxe_plugin_sdk::{OptionSpec, PluginSpec, RequestStatus, Tone};

pub const PLUGIN_ID: &str = "fx_control";
pub const PLUGIN_LABEL: &str = "FX Control";
pub const CONFIG_PATH: &str = "sd:/ultimate/ssbu_fx_control/config.ini";

static MODE_CHOICES: [&str; 3] = ["On", "Off", "Reduced"];
static MODE_VALUES: [&str; 3] = ["on", "off", "reduced"];
static MODE_TONES: [Tone; 3] = [Tone::Ok, Tone::Neutral, Tone::Warn];
static ON_OFF_CHOICES: [&str; 2] = ["On", "Off"];
static ON_OFF_VALUES: [&str; 2] = ["on", "off"];
static ON_OFF_TONES: [Tone; 2] = [Tone::Ok, Tone::Neutral];
static SCOPE_CHOICES: [&str; 2] = ["Online", "All"];
static SCOPE_VALUES: [&str; 2] = ["online", "all"];
static STATUS_CHOICES: [&str; 3] = ["Ready", "Pending", "Unavailable"];
static STATUS_TONES: [Tone; 3] = [Tone::Ok, Tone::Warn, Tone::Error];

/// Option index of the first fighter row.
const FIRST_FIGHTER: usize = 2;
/// Option index of the status row.
pub const STATUS_ROW: usize = FIRST_FIGHTER + COUNT;
pub const OPTION_COUNT: usize = STATUS_ROW + 1;

const fn fighter_option(fighter: Fighter) -> OptionSpec {
    let reduced = fighter.has_reduced();
    OptionSpec {
        id: fighter.id(),
        label: fighter.label(),
        choices: if reduced {
            &MODE_CHOICES
        } else {
            &ON_OFF_CHOICES
        },
        config_key: Some(fighter.id()),
        values: Some(if reduced {
            &MODE_VALUES
        } else {
            &ON_OFF_VALUES
        }),
        no_persist: false,
        tones: Some(if reduced { &MODE_TONES } else { &ON_OFF_TONES }),
        read_only: false,
    }
}

const fn build_options() -> [OptionSpec; OPTION_COUNT] {
    let scope = OptionSpec {
        id: "scope",
        label: "Scope",
        choices: &SCOPE_CHOICES,
        config_key: Some("scope"),
        values: Some(&SCOPE_VALUES),
        no_persist: false,
        tones: None,
        read_only: false,
    };
    // The plugin's own thread writes `all=` with every fighter line; the host
    // would write only the one key.
    let all = OptionSpec {
        id: "all",
        label: "All",
        choices: &MODE_CHOICES,
        config_key: Some("all"),
        values: Some(&MODE_VALUES),
        no_persist: true,
        tones: Some(&MODE_TONES),
        read_only: false,
    };
    let status = OptionSpec {
        id: "status",
        label: "EFF variants",
        choices: &STATUS_CHOICES,
        config_key: None,
        values: None,
        no_persist: true,
        tones: Some(&STATUS_TONES),
        read_only: true,
    };
    let mut options = [status; OPTION_COUNT];
    options[0] = scope;
    options[1] = all;
    let mut index = 0;
    while index < COUNT {
        options[FIRST_FIGHTER + index] = fighter_option(fighters::ALL[index]);
        index += 1;
    }
    options
}

pub static OPTIONS: [OptionSpec; OPTION_COUNT] = build_options();

pub static SPEC: PluginSpec = PluginSpec {
    id: PLUGIN_ID,
    label: PLUGIN_LABEL,
    options: &OPTIONS,
    config_path: Some(CONFIG_PATH),
    own_page: true,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Row {
    Scope,
    All,
    Fighter(Fighter),
    Status,
}

pub fn row(option: usize) -> Option<Row> {
    match option {
        0 => Some(Row::Scope),
        1 => Some(Row::All),
        STATUS_ROW => Some(Row::Status),
        index if (FIRST_FIGHTER..STATUS_ROW).contains(&index) => {
            Some(Row::Fighter(fighters::ALL[index - FIRST_FIGHTER]))
        }
        _ => None,
    }
}

/// One status value for the four EFF files: any failure is Unavailable, any
/// successful load is Ready, otherwise Pending (no covered file loaded yet).
pub fn variant_status(files: &[VariantAvailability]) -> VariantAvailability {
    if files.contains(&VariantAvailability::Unavailable) {
        VariantAvailability::Unavailable
    } else if files.contains(&VariantAvailability::Ready) {
        VariantAvailability::Ready
    } else {
        VariantAvailability::Pending
    }
}

const fn status_choice(status: VariantAvailability) -> usize {
    match status {
        VariantAvailability::Ready => 0,
        VariantAvailability::Pending => 1,
        VariantAvailability::Unavailable => 2,
    }
}

/// `current` for one option. `None` is `CHOICE_UNAVAILABLE`.
pub fn current(
    table: &ModeTable,
    option: usize,
    scope_row_available: bool,
    status: VariantAvailability,
) -> Option<usize> {
    match row(option)? {
        Row::Scope => scope_row_available.then_some(table.scope() as usize),
        Row::All => table.all().map(|mode| mode as usize),
        Row::Fighter(fighter) => Some(table.mode(fighter) as usize),
        Row::Status => Some(status_choice(status)),
    }
}

/// `request` for one option. Every accepted change applies at once.
pub fn request(
    table: &ModeTable,
    option: usize,
    choice: usize,
    scope_row_available: bool,
) -> RequestStatus {
    let mode = FxMode::ALL.get(choice).copied();
    match row(option) {
        Some(Row::Scope) => {
            if !scope_row_available {
                return RequestStatus::Unavailable;
            }
            match choice {
                0 => table.set_scope(FxScope::Online),
                1 => table.set_scope(FxScope::All),
                _ => return RequestStatus::Rejected,
            }
            RequestStatus::Applied
        }
        Some(Row::All) => match mode {
            Some(mode) => {
                table.set_all(mode);
                RequestStatus::Applied
            }
            None => RequestStatus::Rejected,
        },
        Some(Row::Fighter(fighter)) => match mode {
            Some(mode) if table.set(fighter, mode) => RequestStatus::Applied,
            _ => RequestStatus::Rejected,
        },
        Some(Row::Status) | None => RequestStatus::Rejected,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deluxe_plugin_sdk::{register, PluginHandler, RegisterError};

    struct Handler(&'static ModeTable);

    impl PluginHandler for Handler {
        fn current(&self, option: usize) -> Option<usize> {
            current(self.0, option, true, VariantAvailability::Pending)
        }
        fn request(&self, option: usize, choice: usize) -> RequestStatus {
            request(self.0, option, choice, true)
        }
    }

    #[test]
    fn spec_is_valid_for_the_sdk() {
        static TABLE: ModeTable = ModeTable::new();
        // The SDK validates locally first: Invalid would be returned before
        // the host lookup, which is always absent on a development host.
        assert_eq!(
            register(&SPEC, Handler(&TABLE)),
            Err(RegisterError::HostMissing)
        );
        const { assert!(OPTION_COUNT <= deluxe_plugin_sdk::abi::MAX_OPTIONS) };
        assert_eq!(OPTION_COUNT, 18, "scope, all, 15 fighters, status");
        assert!(SPEC.own_page);
    }

    #[test]
    fn rows_are_scope_all_fighters_then_status() {
        assert_eq!(OPTIONS[0].id, "scope");
        assert_eq!(OPTIONS[1].id, "all");
        assert!(OPTIONS[1].no_persist);
        for (index, fighter) in fighters::ALL.iter().enumerate() {
            let option = &OPTIONS[FIRST_FIGHTER + index];
            assert_eq!(option.id, fighter.id());
            assert_eq!(option.label, fighter.label());
            assert_eq!(option.config_key, Some(fighter.id()));
            let expected = if fighter.has_reduced() { 3 } else { 2 };
            assert_eq!(option.choices.len(), expected);
            assert_eq!(option.tones.unwrap()[0], Tone::Ok);
            assert_eq!(row(FIRST_FIGHTER + index), Some(Row::Fighter(*fighter)));
        }
        let status = &OPTIONS[STATUS_ROW];
        assert!(status.read_only && status.no_persist);
        assert_eq!(row(STATUS_ROW), Some(Row::Status));
        assert_eq!(row(OPTION_COUNT), None);
        // A fighter without a Reduced variant gets On / Off only.
        let narrow = OptionSpec {
            choices: &ON_OFF_CHOICES,
            ..OPTIONS[FIRST_FIGHTER]
        };
        assert_eq!(narrow.choices, ["On", "Off"]);
    }

    #[test]
    fn all_row_sets_every_fighter_and_reports_unavailable_while_mixed() {
        let table = ModeTable::new();
        let status = VariantAvailability::Pending;
        assert_eq!(current(&table, 1, true, status), Some(0));
        assert_eq!(request(&table, 1, 1, true), RequestStatus::Applied);
        for index in 0..COUNT {
            assert_eq!(
                current(&table, FIRST_FIGHTER + index, true, status),
                Some(1)
            );
        }
        assert_eq!(current(&table, 1, true, status), Some(1));
        assert_eq!(
            request(&table, FIRST_FIGHTER + 3, 2, true),
            RequestStatus::Applied
        );
        assert_eq!(current(&table, 1, true, status), None, "mixed");
        assert_eq!(request(&table, 1, 2, true), RequestStatus::Applied);
        assert_eq!(current(&table, 1, true, status), Some(2));
        assert_eq!(request(&table, 1, 3, true), RequestStatus::Rejected);
        assert_eq!(
            request(&table, FIRST_FIGHTER, 9, true),
            RequestStatus::Rejected
        );
    }

    #[test]
    fn scope_and_status_rows() {
        let table = ModeTable::new();
        let ready = VariantAvailability::Ready;
        assert_eq!(current(&table, 0, true, ready), Some(0));
        assert_eq!(request(&table, 0, 1, true), RequestStatus::Applied);
        assert_eq!(table.scope(), FxScope::All);
        assert_eq!(current(&table, 0, false, ready), None);
        assert_eq!(request(&table, 0, 0, false), RequestStatus::Unavailable);
        assert_eq!(table.scope(), FxScope::All);
        assert_eq!(current(&table, STATUS_ROW, true, ready), Some(0));
        assert_eq!(
            current(&table, STATUS_ROW, true, VariantAvailability::Unavailable),
            Some(2)
        );
        assert_eq!(
            request(&table, STATUS_ROW, 0, true),
            RequestStatus::Rejected
        );
    }

    #[test]
    fn variant_status_aggregates_the_four_files() {
        use VariantAvailability::{Pending, Ready, Unavailable};
        assert_eq!(variant_status(&[Pending; 4]), Pending);
        assert_eq!(variant_status(&[Ready, Pending, Pending, Pending]), Ready);
        assert_eq!(
            variant_status(&[Ready, Ready, Ready, Unavailable]),
            Unavailable
        );
        assert_eq!(variant_status(&[Unavailable; 4]), Unavailable);
    }
}
