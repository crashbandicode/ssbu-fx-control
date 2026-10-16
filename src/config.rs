//! Modes, scope, per-move variant selection and the plugin's own INI
//! (`sd:/ultimate/ssbu_fx_control/config.ini`).
//!
//! ```ini
//! scope=online             # online | all
//! all=on                   # on | off | reduced; written only while every row agrees
//! kazuya=on                # one line per covered fighter: on | off | reduced
//! variant.kirby_hammer=2   # Reduced variant per move (0 = original); written when not the default
//! cull=eflame_sword_open   # harness: extra names nulled under Reduced (comma separated)
//! ```
//!
//! Loading applies `all=` first and then every fighter line, whatever the line
//! order, so a fighter line always wins over `all=`. Unknown keys, comments and
//! values that do not parse are ignored on load and preserved on save. A
//! `reduced` value for a fighter without a Reduced variant is ignored (the row
//! stays at its default, On); it never becomes Off. `variant.<move>` takes 0
//! to the move's variant count; anything else keeps the default (1 where the
//! move has a variant). `cull=` names must carry a covered fighter's prefix;
//! the line is read, never rewritten (the menu cannot change it).
use crate::effects::hash40;
use crate::fighters::{self, Fighter, COUNT};
use crate::moves::{self, Selection};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum FxMode {
    #[default]
    On = 0,
    Off = 1,
    Reduced = 2,
}

impl FxMode {
    pub const ALL: [FxMode; 3] = [FxMode::On, FxMode::Off, FxMode::Reduced];

    pub const fn label(self) -> &'static str {
        match self {
            Self::On => "On",
            Self::Off => "Off",
            Self::Reduced => "Reduced",
        }
    }

    pub const fn token(self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Reduced => "reduced",
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|mode| mode.token() == token)
    }

    pub const fn from_byte(byte: u8) -> Self {
        match byte {
            1 => Self::Off,
            2 => Self::Reduced,
            _ => Self::On,
        }
    }
}

/// Where the policy applies. `Online` means online play outside Training.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum FxScope {
    #[default]
    Online = 0,
    All = 1,
}

impl FxScope {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::All => "all",
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        [Self::Online, Self::All]
            .into_iter()
            .find(|scope| scope.token() == token)
    }

    pub const fn from_byte(byte: u8) -> Self {
        if byte == 1 {
            Self::All
        } else {
            Self::Online
        }
    }
}

/// Mode a fighter takes when "All" is set to `mode`: `Reduced` on a fighter
/// without a Reduced variant is On, never Off.
pub const fn all_value_for(mode: FxMode, has_reduced: bool) -> FxMode {
    match mode {
        FxMode::Reduced if !has_reduced => FxMode::On,
        other => other,
    }
}

/// The value the "All" row shows: the one mode every row has (counting On as
/// Reduced on a row without a Reduced variant), or `None` while rows differ.
pub fn common_mode(modes: &[FxMode], has_reduced: &[bool]) -> Option<FxMode> {
    FxMode::ALL.into_iter().find(|candidate| {
        modes
            .iter()
            .zip(has_reduced)
            .all(|(mode, reduced)| *mode == all_value_for(*candidate, *reduced))
    })
}

/// A stored mode that the row offers, else `None` (Reduced without a variant).
pub const fn accepted(mode: FxMode, has_reduced: bool) -> Option<FxMode> {
    match mode {
        FxMode::Reduced if !has_reduced => None,
        other => Some(other),
    }
}

/// Most names the harness `cull=` line accepts.
pub const CULL_MAX: usize = 16;

/// Extra names nulled under Reduced (the bisection list), with the fighter
/// each name's prefix names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CullList {
    pub hashes: [u64; CULL_MAX],
    pub fighters: [u8; CULL_MAX],
    pub len: u8,
}

impl Default for CullList {
    fn default() -> Self {
        Self {
            hashes: [0; CULL_MAX],
            fighters: [0; CULL_MAX],
            len: 0,
        }
    }
}

impl CullList {
    pub fn parse(value: &str) -> Self {
        let mut list = Self::default();
        for name in value
            .split(',')
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            if usize::from(list.len) == CULL_MAX {
                break;
            }
            let Some(fighter) = fighters::name_fighter(name) else {
                continue;
            };
            let slot = usize::from(list.len);
            list.hashes[slot] = hash40(name);
            list.fighters[slot] = fighter as u8;
            list.len += 1;
        }
        list
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn entries(&self) -> impl Iterator<Item = (u64, Fighter)> + '_ {
        self.hashes[..usize::from(self.len)]
            .iter()
            .zip(&self.fighters)
            .map(|(hash, fighter)| (*hash, fighters::ALL[usize::from(*fighter)]))
    }

    pub fn owner(&self, hash: u64) -> Option<Fighter> {
        self.entries()
            .find(|(listed, _)| *listed == hash)
            .map(|(_, fighter)| fighter)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Settings {
    pub scope: FxScope,
    pub modes: [FxMode; COUNT],
    /// Reduced variant per move (`moves::MOVES` order); 0 keeps the move original.
    pub variants: Selection,
    pub cull: CullList,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scope: FxScope::Online,
            modes: [FxMode::On; COUNT],
            variants: moves::default_selection(),
            cull: CullList::default(),
        }
    }
}

impl Settings {
    pub fn mode(&self, fighter: Fighter) -> FxMode {
        self.modes[fighter.index()]
    }

    /// Common value of every row, or `None` while they differ.
    pub fn all(&self) -> Option<FxMode> {
        common_mode(&self.modes, &reduced_support())
    }
}

/// Reduced support per row, in `fighters::ALL` order.
pub fn reduced_support() -> [bool; COUNT] {
    fighters::ALL.map(Fighter::has_reduced)
}

const VARIANT_PREFIX: &str = "variant.";

fn assignment(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
        return None;
    }
    let (key, value) = line.split_once('=')?;
    let value = value.split('#').next().unwrap_or_default();
    Some((key.trim(), value.trim()))
}

/// Parses the INI. Never fails: anything unusable keeps its default. A line
/// for a retired 0.1.x fighter (`kazuya=off`, `ike=`, `chrom=`, `roy=`,
/// `ganondorf=`) is an unknown key: ignored here, kept verbatim by `render`.
pub fn parse(text: &str) -> Settings {
    let mut settings = Settings::default();
    let support = reduced_support();
    // `all=` first, so fighter lines override it in any order. Last valid wins.
    for (key, value) in text.lines().filter_map(assignment) {
        match key {
            "all" => {
                if let Some(mode) = FxMode::from_token(value) {
                    for (slot, reduced) in settings.modes.iter_mut().zip(support) {
                        *slot = all_value_for(mode, reduced);
                    }
                }
            }
            "scope" => {
                if let Some(scope) = FxScope::from_token(value) {
                    settings.scope = scope;
                }
            }
            "cull" => settings.cull = CullList::parse(value),
            _ => {}
        }
    }
    for (key, value) in text.lines().filter_map(assignment) {
        if let Some(fighter) = Fighter::from_id(key) {
            if let Some(mode) =
                FxMode::from_token(value).and_then(|mode| accepted(mode, fighter.has_reduced()))
            {
                settings.modes[fighter.index()] = mode;
            }
        } else if let Some(id) = key.strip_prefix(VARIANT_PREFIX) {
            if let Some(index) = moves::by_id(id) {
                if let Ok(number) = value.parse::<u8>() {
                    if usize::from(number) <= moves::MOVES[index].variants.len() {
                        settings.variants[index] = number;
                    }
                }
            }
        }
    }
    settings
}

const HEADER: &str = "# ssbu-fx-control. Values: on | off | reduced. scope: online | all.
# The all key is applied first and every fighter line overrides it. It is
# written only while every fighter has the same value.
# variant.<move>=N selects a Reduced variant per move (0 = original); the
# default (variant 1) is not written. cull=<name>,<name> is the measurement
# harness: extra requests nulled while that fighter is Reduced.
";

/// Renders `settings` over `existing`: known keys are rewritten in place (the
/// first line of each; later duplicates are dropped), missing keys are
/// appended in menu order, and every other line is kept as it was.
pub fn render(existing: &str, settings: &Settings) -> String {
    let all = settings.all();
    let defaults = moves::default_selection();
    let value_for = |key: &str| -> Option<Option<String>> {
        match key {
            "scope" => Some(Some(settings.scope.token().to_owned())),
            "all" => Some(all.map(|mode| mode.token().to_owned())),
            _ => {
                if let Some(fighter) = Fighter::from_id(key) {
                    return Some(Some(settings.mode(fighter).token().to_owned()));
                }
                let index = key.strip_prefix(VARIANT_PREFIX).and_then(moves::by_id)?;
                let number = settings.variants[index];
                Some((number != defaults[index]).then(|| number.to_string()))
            }
        }
    };
    let mut written: Vec<String> = Vec::new();
    let mut out = String::new();
    if existing.trim().is_empty() {
        out.push_str(HEADER);
    }
    for line in existing.lines() {
        let Some((key, _)) = assignment(line) else {
            out.push_str(line);
            out.push('\n');
            continue;
        };
        match value_for(key) {
            None => {
                out.push_str(line);
                out.push('\n');
            }
            Some(value) => {
                if written.iter().any(|done| done == key) {
                    continue;
                }
                written.push(key.to_owned());
                if let Some(value) = value {
                    out.push_str(&format!("{key}={value}\n"));
                }
            }
        }
    }
    let variant_keys: Vec<String> = moves::MOVES
        .iter()
        .map(|mv| format!("{VARIANT_PREFIX}{}", mv.id))
        .collect();
    let order = ["scope", "all"]
        .into_iter()
        .chain(fighters::ALL.iter().map(|fighter| fighter.id()))
        .chain(variant_keys.iter().map(String::as_str));
    for key in order {
        if written.iter().any(|done| done == key) {
            continue;
        }
        if let Some(Some(value)) = value_for(key) {
            out.push_str(&format!("{key}={value}\n"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_everything_on_and_online_scope() {
        let settings = parse("");
        assert_eq!(settings, Settings::default());
        assert_eq!(settings.scope, FxScope::Online);
        assert!(settings.modes.iter().all(|mode| *mode == FxMode::On));
        assert_eq!(settings.all(), Some(FxMode::On));
        assert_eq!(settings.variants, moves::default_selection());
        assert!(settings.cull.is_empty());
    }

    #[test]
    fn round_trip_through_a_fresh_file() {
        let mut settings = Settings {
            scope: FxScope::All,
            ..Settings::default()
        };
        settings.modes[Fighter::Inkling.index()] = FxMode::Off;
        settings.modes[Fighter::Pyra.index()] = FxMode::Reduced;
        let kirby = moves::by_id("kirby_hammer").unwrap();
        settings.variants[kirby] = 3;
        let text = render("", &settings);
        assert!(text.starts_with("# ssbu-fx-control"));
        assert!(text.contains("scope=all\n"));
        assert!(text.contains("inkling=off\n"));
        assert!(text.contains("pyra=reduced\n"));
        assert!(!text.contains("kazuya="), "no line without a row:\n{text}");
        assert!(text.contains("variant.kirby_hammer=3\n"));
        assert!(
            !text.contains("variant.pyra_side_b="),
            "default variants are not written:\n{text}"
        );
        assert!(
            !text.contains("all="),
            "mixed rows write no all= line:\n{text}"
        );
        assert_eq!(parse(&text), settings);
        // Re-rendering the rendered file is a fixed point.
        assert_eq!(render(&text, &settings), text);
    }

    #[test]
    fn all_applies_first_and_fighter_lines_override_in_any_order() {
        let text = "ness=on\nall=off\nscope=all\n";
        let settings = parse(text);
        assert_eq!(settings.scope, FxScope::All);
        assert_eq!(settings.mode(Fighter::Ness), FxMode::On);
        assert_eq!(settings.mode(Fighter::Kirby), FxMode::Off);
        assert_eq!(settings.all(), None);
        let uniform = parse("all=reduced\n");
        assert_eq!(uniform.all(), Some(FxMode::Reduced));
        assert_eq!(uniform.mode(Fighter::Inkling), FxMode::On, "no variant: On");
        assert_eq!(uniform.mode(Fighter::Kirby), FxMode::Reduced);
        let text = render("", &uniform);
        assert!(text.contains("all=reduced\n"));
        assert_eq!(parse(&text), uniform);
    }

    #[test]
    fn unknown_keys_comments_and_bad_values_survive() {
        let existing = "# my notes\nfuture_key=7\npyra=maybe\n\n; other\nscope = online # trailing\npyra=off\nall=sideways\nkazuya=off\n";
        let settings = parse(existing);
        assert_eq!(settings.mode(Fighter::Pyra), FxMode::Off);
        assert_eq!(settings.scope, FxScope::Online);
        // A 0.1.x line for a retired fighter is an unknown key: ignored and kept.
        assert_eq!(settings, parse(existing.replace("kazuya=off\n", "").as_str()));
        let out = render(existing, &settings);
        assert!(out.starts_with("# my notes\nfuture_key=7\n"));
        assert!(out.contains("\n; other\n"));
        assert!(out.contains("kazuya=off\n"));
        assert_eq!(out.matches("kazuya=").count(), 1);
        // First pyra line is rewritten in place; the later duplicate is dropped.
        assert_eq!(out.matches("pyra=").count(), 1);
        assert!(out.contains("pyra=off\n"));
        assert!(!out.contains("maybe"));
        // Mixed rows: the stale all= line is removed rather than left wrong.
        assert!(!out.contains("all="));
        for fighter in fighters::ALL {
            assert!(out.contains(&format!("{}=", fighter.id())), "{fighter:?}");
        }
        assert_eq!(parse(&out), settings);
    }

    #[test]
    fn variant_lines_select_per_move_and_bad_numbers_keep_the_default() {
        let kirby = moves::by_id("kirby_hammer").unwrap();
        let pyra = moves::by_id("pyra_side_b").unwrap();
        let settings = parse(
            "variant.kirby_hammer=0\nvariant.pyra_side_b=3\nvariant.pyra_up_b=9\nvariant.not_a_move=1\nvariant.ness_pk_fire=x\n",
        );
        assert_eq!(settings.variants[kirby], 0);
        assert_eq!(settings.variants[pyra], 3);
        assert_eq!(settings.variants[moves::by_id("pyra_up_b").unwrap()], 1);
        assert_eq!(settings.variants[moves::by_id("ness_pk_fire").unwrap()], 1);
        // A variant line back at the default is dropped on rewrite; a
        // non-default one is rewritten in place.
        let mut back = settings;
        back.variants[pyra] = 1;
        let out = render("variant.pyra_side_b=3\nvariant.kirby_hammer=0\n", &back);
        assert!(
            out.starts_with("variant.kirby_hammer=0\nscope=online\n"),
            "{out}"
        );
        assert!(!out.contains("variant.pyra_side_b"), "{out}");
        assert_eq!(out.matches("variant.").count(), 1);
        assert_eq!(parse(&out), back);
    }

    #[test]
    fn cull_line_lists_prefixed_names_and_is_preserved_verbatim() {
        let text = "cull= eflame_sword_open ,kirby_onigoroshi_arc, sys_flash, nobody_knows,\n";
        let settings = parse(text);
        assert_eq!(settings.cull.len, 2);
        assert_eq!(
            settings.cull.owner(hash40("eflame_sword_open")),
            Some(Fighter::Pyra)
        );
        assert_eq!(
            settings.cull.owner(hash40("kirby_onigoroshi_arc")),
            Some(Fighter::Kirby)
        );
        assert_eq!(settings.cull.owner(hash40("sys_flash")), None);
        let out = render(text, &settings);
        assert!(out.starts_with(text), "{out}");
        assert_eq!(parse(&out).cull, settings.cull);
        let many: Vec<String> = (0..CULL_MAX + 4).map(|n| format!("kirby_x{n}")).collect();
        let full = CullList::parse(&many.join(","));
        assert_eq!(usize::from(full.len), CULL_MAX);
    }

    #[test]
    fn reduced_without_a_variant_is_never_off() {
        // Synthetic capabilities: the generic rule, independent of today's rows.
        assert_eq!(accepted(FxMode::Reduced, false), None);
        assert_eq!(accepted(FxMode::Reduced, true), Some(FxMode::Reduced));
        assert_eq!(accepted(FxMode::Off, false), Some(FxMode::Off));
        assert_eq!(all_value_for(FxMode::Reduced, false), FxMode::On);
        assert_eq!(all_value_for(FxMode::Reduced, true), FxMode::Reduced);
        assert_eq!(all_value_for(FxMode::Off, false), FxMode::Off);
        let caps = [true, false, true];
        let modes = [FxMode::Reduced, FxMode::On, FxMode::Reduced];
        assert_eq!(common_mode(&modes, &caps), Some(FxMode::Reduced));
        let modes = [FxMode::Reduced, FxMode::Off, FxMode::Reduced];
        assert_eq!(common_mode(&modes, &caps), None);
        let modes = [FxMode::On, FxMode::On, FxMode::On];
        assert_eq!(common_mode(&modes, &caps), Some(FxMode::On));
    }

    #[test]
    fn shipped_inis_parse_and_are_fixed_points() {
        let default = include_str!("../config/default.ini");
        assert_eq!(parse(default), Settings::default());
        assert_eq!(render(default, &parse(default)), default);
        let eden = include_str!("../config/eden-test.ini");
        let settings = parse(eden);
        assert_eq!(settings.scope, FxScope::All);
        assert_eq!(settings.all(), Some(FxMode::On));
        assert_eq!(render(eden, &settings), eden);
    }

    #[test]
    fn tokens_round_trip() {
        for mode in FxMode::ALL {
            assert_eq!(FxMode::from_token(mode.token()), Some(mode));
            assert_eq!(FxMode::from_byte(mode as u8), mode);
        }
        for scope in [FxScope::Online, FxScope::All] {
            assert_eq!(FxScope::from_token(scope.token()), Some(scope));
            assert_eq!(FxScope::from_byte(scope as u8), scope);
        }
        assert_eq!(FxMode::from_token("On"), None);
    }
}
