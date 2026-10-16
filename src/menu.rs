//! Rows on the Online Deluxe window (plugin SDK, descriptor version 4).
//!
//! The plugin asks for its own page in Deluxe's window cycle. Rows, in order:
//! `Scope` (Online / All), `All` (On / Off / Reduced), one row per ledger
//! fighter (`fighters::PAGE`: On / Off / Reduced, or On / Off for a fighter
//! without a Reduced variant), then one group per fighter with variants: a
//! header row with the fighter's name (not selectable) and, under it, one
//! value row per move showing the selected variant (`Original`,
//! `Variant 1 .. N`; the `variant.<move>` value), which changes in place.
//! Fighters with no ledger move (Kazuya, Ike, Chrom, Roy, Ganondorf) have no
//! row since 0.2.0, and the 0.1.x `EFF variants` status row is gone with
//! the sibling feature.
//!
//! "All" has no fourth "Mixed" choice: while the fighter rows differ, its
//! `current` answers `CHOICE_UNAVAILABLE`, which the host shows as
//! `Unavailable`. Moving it to a choice sets every row.
//!
//! The variant rows have no host config key: the plugin's own saver writes
//! `variant.<move>=N` from the table (only when not the default), as it does
//! for `all=`. Header rows need a Deluxe host with descriptor version 4
//! (0.5.3); the SDK does not downgrade a spec that carries them.
use crate::config::{FxMode, FxScope};
use crate::fighters::{Fighter, PAGE, PAGE_COUNT};
use crate::moves;
use crate::policy::ModeTable;
use deluxe_plugin_sdk::{OptionSpec, PluginSpec, RequestStatus, RowLayout, Tone};

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
/// Choices of a variant row; a move with `n` variants shows the first `n + 1`.
/// Extend both arrays (and `PAGE_VARIANTS`) if a move ever gets a fourth.
static VARIANT_CHOICES: [&str; 4] = ["Original", "Variant 1", "Variant 2", "Variant 3"];
static VARIANT_TONES: [Tone; 4] = [Tone::Neutral, Tone::Warn, Tone::Warn, Tone::Warn];
const PAGE_VARIANTS: usize = VARIANT_CHOICES.len() - 1;

/// Option index of the first fighter row.
const FIRST_FIGHTER: usize = 2;
/// Option index of the first group row (a fighter header).
const FIRST_GROUP: usize = FIRST_FIGHTER + PAGE_COUNT;
/// Group rows: one header per fighter with variants plus one value row per
/// move with variants.
pub const GROUP_ROWS: usize = group_slots().0;
pub const OPTION_COUNT: usize = FIRST_GROUP + GROUP_ROWS;

/// What one group row stands for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Slot {
    /// The fighter's header (not selectable).
    Header(Fighter),
    /// The variant row of `moves::MOVES[index]`.
    Move(usize),
}

const MAX_GROUP_ROWS: usize = PAGE_COUNT + moves::COUNT;

/// Group rows in page order: fighters in `fighters::PAGE` order, each header
/// followed by that fighter's moves in `moves::MOVES` order. The count comes
/// first so one pass serves both constants.
const fn group_slots() -> (usize, [Slot; MAX_GROUP_ROWS]) {
    let mut out = [Slot::Move(0); MAX_GROUP_ROWS];
    let mut rows = 0;
    let mut fighter = 0;
    while fighter < PAGE_COUNT {
        let owner = PAGE[fighter];
        let mut first = true;
        let mut index = 0;
        while index < moves::COUNT {
            let mv = &moves::MOVES[index];
            if mv.fighter as usize == owner as usize && !mv.variants.is_empty() {
                assert!(
                    mv.variants.len() <= PAGE_VARIANTS,
                    "VARIANT_CHOICES needs one entry per variant plus Original"
                );
                if first {
                    out[rows] = Slot::Header(owner);
                    rows += 1;
                    first = false;
                }
                out[rows] = Slot::Move(index);
                rows += 1;
            }
            index += 1;
        }
        fighter += 1;
    }
    (rows, out)
}

const GROUP_SLOTS: [Slot; MAX_GROUP_ROWS] = group_slots().1;

/// Option id of a fighter's header row.
const fn header_id(fighter: Fighter) -> &'static str {
    match fighter {
        Fighter::Kazuya => "group_kazuya",
        Fighter::CaptainFalcon => "group_captain_falcon",
        Fighter::Ness => "group_ness",
        Fighter::Lucas => "group_lucas",
        Fighter::Ike => "group_ike",
        Fighter::Chrom => "group_chrom",
        Fighter::Roy => "group_roy",
        Fighter::Palutena => "group_palutena",
        Fighter::Sephiroth => "group_sephiroth",
        Fighter::Pyra => "group_pyra",
        Fighter::Mythra => "group_mythra",
        Fighter::Byleth => "group_byleth",
        Fighter::Inkling => "group_inkling",
        Fighter::Ganondorf => "group_ganondorf",
        Fighter::Kirby => "group_kirby",
    }
}

const fn group_option(slot: Slot) -> OptionSpec {
    match slot {
        Slot::Header(fighter) => OptionSpec::header(header_id(fighter), fighter.label()),
        Slot::Move(index) => {
            let mv = &moves::MOVES[index];
            let shown = mv.variants.len() + 1;
            let choices: &'static [&'static str] = &VARIANT_CHOICES;
            let tones: &'static [Tone] = &VARIANT_TONES;
            OptionSpec {
                id: mv.id,
                label: mv.short,
                choices: choices.split_at(shown).0,
                config_key: None,
                values: None,
                no_persist: true,
                tones: Some(tones.split_at(shown).0),
                read_only: false,
                // A value row: the choice changes in place. The inline strip
                // (`Original  Variant 1 ..`) scrolled off the pane on the console.
                layout: RowLayout::Value,
            }
        }
    }
}

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
        layout: RowLayout::Value,
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
        layout: RowLayout::Value,
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
        layout: RowLayout::Value,
    };
    let mut options = [scope; OPTION_COUNT];
    options[1] = all;
    let mut index = 0;
    while index < PAGE_COUNT {
        options[FIRST_FIGHTER + index] = fighter_option(PAGE[index]);
        index += 1;
    }
    let mut row = 0;
    while row < GROUP_ROWS {
        options[FIRST_GROUP + row] = group_option(GROUP_SLOTS[row]);
        row += 1;
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
    /// A fighter's group header: label only, never selected.
    Header(Fighter),
    /// The Reduced variant of one move (index into `moves::MOVES`).
    Variant(usize),
}

pub fn row(option: usize) -> Option<Row> {
    match option {
        0 => Some(Row::Scope),
        1 => Some(Row::All),
        index if (FIRST_FIGHTER..FIRST_GROUP).contains(&index) => {
            Some(Row::Fighter(PAGE[index - FIRST_FIGHTER]))
        }
        index if (FIRST_GROUP..OPTION_COUNT).contains(&index) => {
            Some(match GROUP_SLOTS[index - FIRST_GROUP] {
                Slot::Header(fighter) => Row::Header(fighter),
                Slot::Move(move_index) => Row::Variant(move_index),
            })
        }
        _ => None,
    }
}

/// The option index of a move's variant row, if the move has variants.
pub fn variant_row(move_index: usize) -> Option<usize> {
    GROUP_SLOTS[..GROUP_ROWS]
        .iter()
        .position(|slot| *slot == Slot::Move(move_index))
        .map(|row| FIRST_GROUP + row)
}

/// The option index of a fighter's header row, if the fighter has a move
/// with variants.
pub fn header_row(fighter: Fighter) -> Option<usize> {
    GROUP_SLOTS[..GROUP_ROWS]
        .iter()
        .position(|slot| *slot == Slot::Header(fighter))
        .map(|row| FIRST_GROUP + row)
}

/// `current` for one option. `None` is `CHOICE_UNAVAILABLE`.
pub fn current(table: &ModeTable, option: usize, scope_row_available: bool) -> Option<usize> {
    match row(option)? {
        Row::Scope => scope_row_available.then_some(table.scope() as usize),
        Row::All => table.all().map(|mode| mode as usize),
        Row::Fighter(fighter) => Some(table.mode(fighter) as usize),
        Row::Header(_) => None,
        Row::Variant(index) => Some(usize::from(table.variant(index))),
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
        Some(Row::Variant(index)) => match u8::try_from(choice) {
            Ok(number) if table.select_variant(index, number) => RequestStatus::Applied,
            _ => RequestStatus::Rejected,
        },
        Some(Row::Header(_)) | None => RequestStatus::Rejected,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fighters;
    use deluxe_plugin_sdk::{register, PluginHandler, RegisterError};

    struct Handler(&'static ModeTable);

    impl PluginHandler for Handler {
        fn current(&self, option: usize) -> Option<usize> {
            current(self.0, option, true)
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
        assert_eq!(
            GROUP_ROWS,
            9 + 12,
            "nine fighters with variants, every ledger move but Inkling Roller"
        );
        assert_eq!(
            OPTION_COUNT, 33,
            "scope, all, 10 ledger fighters, 9 headers + 12 variant rows"
        );
        assert!(SPEC.own_page);
        assert!(OPTIONS.iter().any(OptionSpec::needs_version_4));
        // Only the headers need version 4 now; every other row is a value row.
        for spec in OPTIONS.iter() {
            assert_ne!(spec.layout, RowLayout::Inline, "{}", spec.id);
        }
    }

    #[test]
    fn variant_rows_follow_the_fighters_and_select_per_move() {
        let table = ModeTable::new();
        let kirby = moves::by_id("kirby_hammer").unwrap();
        let roller = moves::by_id("inkling_roller").unwrap();
        assert_eq!(variant_row(roller), None, "no variants, no row");
        assert_eq!(header_row(Fighter::Inkling), None, "no variants, no header");
        assert_eq!(header_row(Fighter::Kazuya), None);
        // Headers: one per fighter with variants, label only, never
        // selectable, directly before that fighter's first move row.
        let mut headers = 0;
        for fighter in fighters::ALL {
            let Some(option) = header_row(fighter) else {
                assert!(!moves::fighter_has_variant(fighter));
                continue;
            };
            headers += 1;
            assert!(moves::fighter_has_variant(fighter));
            assert_eq!(row(option), Some(Row::Header(fighter)));
            let spec = &OPTIONS[option];
            assert_eq!(spec.layout, RowLayout::Header);
            assert_eq!(spec.label, fighter.label());
            assert!(spec.choices.is_empty() && spec.read_only && spec.no_persist);
            assert_eq!(current(&table, option, true), None);
            assert_eq!(request(&table, option, 0, true), RequestStatus::Rejected);
            let first_move = moves::MOVES
                .iter()
                .position(|mv| mv.fighter == fighter && !mv.variants.is_empty())
                .unwrap();
            assert_eq!(variant_row(first_move), Some(option + 1));
        }
        assert_eq!(headers, 9);
        // Pyra's three moves sit under one header.
        let pyra = header_row(Fighter::Pyra).unwrap();
        assert!(matches!(row(pyra + 1), Some(Row::Variant(_))));
        assert!(matches!(row(pyra + 2), Some(Row::Variant(_))));
        assert!(matches!(row(pyra + 3), Some(Row::Variant(_))));
        assert!(matches!(row(pyra + 4), Some(Row::Header(Fighter::Mythra))));
        let mut seen = 0;
        for (index, mv) in moves::MOVES.iter().enumerate() {
            let Some(option) = variant_row(index) else {
                assert!(mv.variants.is_empty());
                continue;
            };
            seen += 1;
            assert!((FIRST_GROUP..OPTION_COUNT).contains(&option));
            assert_eq!(row(option), Some(Row::Variant(index)));
            // The move sits after its fighter's header.
            assert!(header_row(mv.fighter).unwrap() < option);
            let spec = &OPTIONS[option];
            assert_eq!(spec.layout, RowLayout::Value, "changes in place");
            assert_eq!(spec.id, mv.id);
            assert_eq!(spec.label, mv.short);
            assert!(spec.label.len() <= deluxe_plugin_sdk::abi::MAX_LABEL_LEN);
            assert_eq!(spec.choices.len(), mv.variants.len() + 1);
            assert_eq!(spec.choices[0], "Original");
            assert_eq!(spec.tones.unwrap().len(), spec.choices.len());
            assert!(spec.no_persist && spec.config_key.is_none() && !spec.read_only);
            // Default: variant 1.
            assert_eq!(current(&table, option, true), Some(1));
            // Every listed choice applies; one past the end is refused.
            for choice in 0..spec.choices.len() {
                assert_eq!(
                    request(&table, option, choice, true),
                    RequestStatus::Applied
                );
                assert_eq!(current(&table, option, true), Some(choice));
            }
            assert_eq!(
                request(&table, option, spec.choices.len(), true),
                RequestStatus::Rejected
            );
        }
        assert_eq!(seen + headers, GROUP_ROWS);
        let option = variant_row(kirby).unwrap();
        assert_eq!(OPTIONS[option].choices.len(), 4);
        assert_eq!(request(&table, option, 0, true), RequestStatus::Applied);
        assert_eq!(table.variant(kirby), 0);
        assert_eq!(table.selection()[kirby], 0);
        assert_eq!(request(&table, option, 300, true), RequestStatus::Rejected);
    }

    #[test]
    fn rows_are_scope_all_page_fighters_then_groups() {
        assert_eq!(OPTIONS[0].id, "scope");
        assert_eq!(OPTIONS[1].id, "all");
        assert!(OPTIONS[1].no_persist);
        for (index, fighter) in PAGE.iter().enumerate() {
            let option = &OPTIONS[FIRST_FIGHTER + index];
            assert_eq!(option.id, fighter.id());
            assert_eq!(option.label, fighter.label());
            assert_eq!(option.config_key, Some(fighter.id()));
            let expected = if fighter.has_reduced() { 3 } else { 2 };
            assert_eq!(option.choices.len(), expected);
            assert_eq!(option.tones.unwrap()[0], Tone::Ok);
            assert_eq!(row(FIRST_FIGHTER + index), Some(Row::Fighter(*fighter)));
        }
        // No row for a fighter without a ledger move; Inkling is On / Off.
        for fighter in fighters::ALL {
            let has_row = OPTIONS.iter().any(|option| option.id == fighter.id());
            assert_eq!(has_row, fighter.on_page(), "{fighter:?}");
        }
        assert!(!OPTIONS.iter().any(|option| option.id == "kazuya"));
        assert!(!OPTIONS.iter().any(|option| option.id == "status"));
        let inkling = OPTIONS
            .iter()
            .find(|option| option.id == "inkling")
            .unwrap();
        assert_eq!(inkling.choices, ["On", "Off"]);
        assert_eq!(row(OPTION_COUNT), None);
        assert_eq!(row(FIRST_GROUP), Some(Row::Header(Fighter::CaptainFalcon)));
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
        assert_eq!(current(&table, 1, true), Some(0));
        assert_eq!(request(&table, 1, 1, true), RequestStatus::Applied);
        for index in 0..PAGE_COUNT {
            assert_eq!(current(&table, FIRST_FIGHTER + index, true), Some(1));
        }
        assert_eq!(current(&table, 1, true), Some(1));
        // Off-page fighters are untouched by All.
        assert_eq!(table.mode(Fighter::Kazuya), FxMode::On);
        assert_eq!(
            request(&table, FIRST_FIGHTER + 3, 2, true),
            RequestStatus::Applied
        );
        assert_eq!(current(&table, 1, true), None, "mixed");
        assert_eq!(request(&table, 1, 2, true), RequestStatus::Applied);
        assert_eq!(current(&table, 1, true), Some(2));
        assert_eq!(request(&table, 1, 3, true), RequestStatus::Rejected);
        assert_eq!(
            request(&table, FIRST_FIGHTER, 9, true),
            RequestStatus::Rejected
        );
        // Inkling (On / Off) refuses Reduced and is On under All = Reduced.
        let inkling = FIRST_FIGHTER + PAGE.iter().position(|f| *f == Fighter::Inkling).unwrap();
        assert_eq!(current(&table, inkling, true), Some(0));
        assert_eq!(request(&table, inkling, 2, true), RequestStatus::Rejected);
    }

    #[test]
    fn scope_row() {
        let table = ModeTable::new();
        assert_eq!(current(&table, 0, true), Some(0));
        assert_eq!(request(&table, 0, 1, true), RequestStatus::Applied);
        assert_eq!(table.scope(), FxScope::All);
        assert_eq!(current(&table, 0, false), None);
        assert_eq!(request(&table, 0, 0, false), RequestStatus::Unavailable);
        assert_eq!(table.scope(), FxScope::All);
    }
}
