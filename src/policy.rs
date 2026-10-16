//! Per-fighter mode table and the per-request decision.
//!
//! The request pipeline is the Deluxe one: `effects_reduced::classify` then
//! `request_hash` then `eff_variant::forward`. What this plugin adds is the
//! mode it is given (the mode of the fighter that owns the request,
//! [`crate::fighters::fighter_for`], or On for a request no row owns), the
//! per-move variant selection and the harness cull list.
use crate::config::{self, CullList, FxMode, FxScope, Settings, CULL_MAX};
use crate::effect_context::Context;
use crate::effects_reduced::{self, FxIntent};
use crate::fighters::{self, Fighter, COUNT};
use crate::moves::{self, Selection};
use crate::{eff_variant, effects};
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

const ANY_ACTIVE: u8 = 1;
const ANY_REDUCED: u8 = 2;

/// Lock-free modes read by game hooks and written by the menu thread.
pub struct ModeTable {
    modes: [AtomicU8; COUNT],
    scope: AtomicU8,
    variants: [AtomicU8; moves::COUNT],
    cull_hashes: [AtomicU64; CULL_MAX],
    cull_fighters: [AtomicU8; CULL_MAX],
    cull_len: AtomicU8,
    /// `ANY_ACTIVE` when a row is not On, `ANY_REDUCED` when a row is Reduced.
    summary: AtomicU8,
    generation: AtomicU64,
}

impl Default for ModeTable {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeTable {
    pub const fn new() -> Self {
        Self {
            modes: [const { AtomicU8::new(FxMode::On as u8) }; COUNT],
            scope: AtomicU8::new(FxScope::Online as u8),
            variants: [const { AtomicU8::new(1) }; moves::COUNT],
            cull_hashes: [const { AtomicU64::new(0) }; CULL_MAX],
            cull_fighters: [const { AtomicU8::new(0) }; CULL_MAX],
            cull_len: AtomicU8::new(0),
            summary: AtomicU8::new(0),
            generation: AtomicU64::new(0),
        }
    }

    pub fn mode(&self, fighter: Fighter) -> FxMode {
        FxMode::from_byte(self.modes[fighter.index()].load(Ordering::Relaxed))
    }

    pub fn scope(&self) -> FxScope {
        FxScope::from_byte(self.scope.load(Ordering::Relaxed))
    }

    /// Bumped on every change; the persistence thread saves when it moves.
    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }

    fn refresh(&self) {
        let mut summary = 0;
        for slot in &self.modes {
            match FxMode::from_byte(slot.load(Ordering::Relaxed)) {
                FxMode::On => {}
                FxMode::Off => summary |= ANY_ACTIVE,
                FxMode::Reduced => summary |= ANY_ACTIVE | ANY_REDUCED,
            }
        }
        self.summary.store(summary, Ordering::Release);
        self.generation.fetch_add(1, Ordering::AcqRel);
    }

    /// Sets one row. Refuses Reduced on a row without a Reduced variant.
    pub fn set(&self, fighter: Fighter, mode: FxMode) -> bool {
        let Some(mode) = config::accepted(mode, fighter.has_reduced()) else {
            return false;
        };
        let previous = self.mode(fighter);
        self.modes[fighter.index()].store(mode as u8, Ordering::Release);
        effects_reduced::observe_mode(previous, mode);
        self.refresh();
        true
    }

    /// "All": every page row takes `mode` (Reduced is On on a row without a
    /// variant). Fighters without a row stay as they are (On).
    pub fn set_all(&self, mode: FxMode) {
        for fighter in fighters::PAGE {
            let value = config::all_value_for(mode, fighter.has_reduced());
            let previous = self.mode(fighter);
            self.modes[fighter.index()].store(value as u8, Ordering::Release);
            effects_reduced::observe_mode(previous, value);
        }
        self.refresh();
    }

    pub fn set_scope(&self, scope: FxScope) {
        self.scope.store(scope as u8, Ordering::Release);
        self.generation.fetch_add(1, Ordering::AcqRel);
    }

    /// Common value of every row, or `None` while they differ.
    pub fn all(&self) -> Option<FxMode> {
        self.settings().all()
    }

    pub fn any_active(&self) -> bool {
        self.summary.load(Ordering::Relaxed) & ANY_ACTIVE != 0
    }

    pub fn any_reduced(&self) -> bool {
        self.summary.load(Ordering::Relaxed) & ANY_REDUCED != 0
    }

    /// The Reduced variant selected for every move.
    pub fn selection(&self) -> Selection {
        let mut selection = [0u8; moves::COUNT];
        for (slot, value) in selection.iter_mut().zip(&self.variants) {
            *slot = value.load(Ordering::Relaxed);
        }
        selection
    }

    /// The Reduced variant selected for one move (0 = original); 0 for an
    /// index outside `moves::MOVES`.
    pub fn variant(&self, index: usize) -> u8 {
        self.variants
            .get(index)
            .map_or(0, |value| value.load(Ordering::Relaxed))
    }

    /// Selects variant `number` for move `index` (0 = original). Refuses a
    /// number above the move's variant count.
    pub fn select_variant(&self, index: usize, number: u8) -> bool {
        let Some(mv) = moves::MOVES.get(index) else {
            return false;
        };
        if usize::from(number) > mv.variants.len() {
            return false;
        }
        self.variants[index].store(number, Ordering::Release);
        self.generation.fetch_add(1, Ordering::AcqRel);
        true
    }

    pub fn cull(&self) -> CullList {
        let mut list = CullList {
            len: self.cull_len.load(Ordering::Acquire),
            ..CullList::default()
        };
        for slot in 0..usize::from(list.len) {
            list.hashes[slot] = self.cull_hashes[slot].load(Ordering::Relaxed);
            list.fighters[slot] = self.cull_fighters[slot].load(Ordering::Relaxed);
        }
        list
    }

    /// The fighter whose harness cull line names this request, if any.
    pub fn cull_owner(&self, hash: u64) -> Option<Fighter> {
        let len = usize::from(self.cull_len.load(Ordering::Acquire));
        (0..len)
            .find(|slot| self.cull_hashes[*slot].load(Ordering::Relaxed) == hash)
            .map(|slot| {
                fighters::ALL[usize::from(self.cull_fighters[slot].load(Ordering::Relaxed))]
            })
    }

    pub fn settings(&self) -> Settings {
        Settings {
            scope: self.scope(),
            modes: fighters::ALL.map(|fighter| self.mode(fighter)),
            variants: self.selection(),
            cull: self.cull(),
        }
    }

    pub fn load(&self, settings: &Settings) {
        self.scope.store(settings.scope as u8, Ordering::Release);
        for fighter in fighters::ALL {
            let mode = config::accepted(settings.mode(fighter), fighter.has_reduced())
                .unwrap_or(FxMode::On);
            self.modes[fighter.index()].store(mode as u8, Ordering::Release);
        }
        for (slot, value) in self.variants.iter().zip(settings.variants) {
            slot.store(value, Ordering::Release);
        }
        // Length last, so a reader never sees a slot it was not meant to.
        self.cull_len.store(0, Ordering::Release);
        for (slot, (hash, fighter)) in settings.cull.entries().enumerate() {
            self.cull_hashes[slot].store(hash, Ordering::Relaxed);
            self.cull_fighters[slot].store(fighter as u8, Ordering::Relaxed);
        }
        self.cull_len.store(settings.cull.len, Ordering::Release);
        self.refresh();
    }

    /// Nothing can change: every row is On, or scope blocks a table with no
    /// Reduced row (Reduced still counts its out-of-scope restores, as Deluxe).
    pub fn idle(&self, scope_allows: bool) -> bool {
        !self.any_active() || (!scope_allows && !self.any_reduced())
    }
}

/// The context lookup is needed when the owner or the policy depends on the move.
pub fn needs_context(hash: u64) -> bool {
    effects::needs_context(hash)
        || effects_reduced::reduced_named(hash)
        || eff_variant::wants_context(hash)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Decision {
    pub fighter: Option<Fighter>,
    pub mode: FxMode,
    pub intent: FxIntent,
    /// After Off/Reduced culling, before the EFF-sibling swap.
    pub requested: u64,
    /// What the original request receives.
    pub forwarded: u64,
}

pub fn decide(
    table: &ModeTable,
    scope_allows: bool,
    hash: u64,
    context: Option<Context>,
) -> Decision {
    let listed = table.cull_owner(hash);
    let fighter = fighters::fighter_for(hash, context).or(listed);
    let mode = fighter.map_or(FxMode::On, |fighter| table.mode(fighter));
    let selection = table.selection();
    let intent = effects_reduced::classify(
        mode,
        scope_allows,
        hash,
        context,
        &selection,
        listed.is_some(),
    );
    let requested = effects_reduced::request_hash(intent, hash);
    let forwarded = eff_variant::forward(mode, scope_allows, requested, context);
    Decision {
        fighter,
        mode,
        intent,
        requested,
        forwarded,
    }
}

/// Screen overlays (`req_screen`, `req_screen_system`). Only the Gigaflare
/// overlay is covered; its owner is Sephiroth by name.
pub fn decide_screen(table: &ModeTable, scope_allows: bool, hash: u64) -> FxIntent {
    let mode = fighters::unscoped_fighter(hash).map_or(FxMode::On, |fighter| table.mode(fighter));
    effects_reduced::classify_screen(mode, scope_allows, hash)
}

/// Mode handed to the `PROBE_CRITICAL_SCREEN` diagnostic: Off only while every
/// row is Off. Special Zoom backgrounds belong to no fighter.
pub fn critical_mode(table: &ModeTable) -> FxMode {
    if table.all() == Some(FxMode::Off) {
        FxMode::Off
    } else {
        FxMode::On
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::{hash40, NULL_EFFECT};

    fn fighter_ctx(kind: i32, motion: &str, status: i32) -> Context {
        Context {
            category: 0,
            kind,
            motion: hash40(motion),
            status,
        }
    }

    #[test]
    fn all_sets_every_row_and_mixed_rows_show_unavailable() {
        let table = ModeTable::new();
        assert_eq!(table.all(), Some(FxMode::On));
        assert!(table.idle(true));
        table.set_all(FxMode::Off);
        assert!(fighters::PAGE
            .iter()
            .all(|fighter| table.mode(*fighter) == FxMode::Off));
        assert_eq!(table.mode(Fighter::Kazuya), FxMode::On, "no row: untouched");
        assert_eq!(table.all(), Some(FxMode::Off));
        assert!(table.set(Fighter::Ness, FxMode::On));
        assert_eq!(table.all(), None, "rows differ: All shows Unavailable");
        table.set_all(FxMode::Reduced);
        assert_eq!(table.all(), Some(FxMode::Reduced));
        assert_eq!(table.mode(Fighter::Inkling), FxMode::On, "no variant: On");
        assert!(table.any_reduced());
        assert!(!table.idle(false));
        table.set_all(FxMode::On);
        assert_eq!(table.all(), Some(FxMode::On));
        assert!(table.idle(false) && table.idle(true));
        assert!(!table.set(Fighter::Inkling, FxMode::Reduced));
        assert!(!table.set(Fighter::Chrom, FxMode::Reduced), "siblings are off");
    }

    #[test]
    fn generation_moves_on_every_change_and_load_round_trips() {
        let table = ModeTable::new();
        let start = table.generation();
        table.set_scope(FxScope::All);
        assert!(table.generation() > start);
        let mut settings = Settings::default();
        settings.modes[Fighter::Pyra.index()] = FxMode::Reduced;
        settings.modes[Fighter::Roy.index()] = FxMode::Off;
        settings.variants[moves::by_id("pyra_side_b").unwrap()] = 2;
        settings.cull = CullList::parse("eflame_sword_open,kirby_onigoroshi_arc");
        table.load(&settings);
        assert_eq!(table.settings(), settings);
        assert!(!table.idle(true));
        assert!(!table.idle(false), "a Reduced row still counts restores");
        let only_off = ModeTable::new();
        only_off.set(Fighter::Roy, FxMode::Off);
        assert!(only_off.idle(false));
        assert!(!only_off.idle(true));
        let before = table.generation();
        assert!(table.select_variant(moves::by_id("kirby_hammer").unwrap(), 3));
        assert!(!table.select_variant(moves::by_id("kirby_hammer").unwrap(), 4));
        assert!(!table.select_variant(99, 1));
        assert_eq!(table.generation(), before + 1);
    }

    #[test]
    fn each_fighter_row_controls_only_its_own_requests() {
        let table = ModeTable::new();
        table.set(Fighter::Ike, FxMode::Off);
        let ike = fighter_ctx(0x23, "special_n", 0x1dc);
        let chrom = fighter_ctx(0x1b, "special_n", 0x1dc);
        // Ike's global name: Off for Ike.
        let d = decide(&table, true, hash40("ike_volcano"), None);
        assert_eq!(d.fighter, Some(Fighter::Ike));
        assert_eq!(d.forwarded, NULL_EFFECT);
        assert_eq!(
            decide(&table, true, hash40("P_IkeVolcanoAdd"), Some(ike)).forwarded,
            NULL_EFFECT
        );
        // Chrom's neutral-B reuses Ike's pillar; Chrom is On, so it stays.
        let pillar = hash40("ike_firepiller02");
        let d = decide(&table, true, pillar, Some(chrom));
        assert_eq!((d.fighter, d.forwarded), (Some(Fighter::Chrom), pillar));
        // Out of scope nothing changes.
        assert_eq!(
            decide(&table, false, hash40("ike_volcano"), None).forwarded,
            hash40("ike_volcano")
        );
        // Another fighter's name is untouched by Ike's row.
        let trail = hash40("eflame_blazeend_trail");
        assert_eq!(decide(&table, true, trail, None).forwarded, trail);
        table.set(Fighter::Pyra, FxMode::Reduced);
        assert_eq!(
            decide(&table, true, trail, None).intent,
            FxIntent::ReducedCull
        );
        assert_eq!(decide(&table, true, trail, None).forwarded, NULL_EFFECT);
        let open = hash40("eflame_sword_open");
        assert_eq!(
            decide(&table, true, open, None).intent,
            FxIntent::ReducedKeep
        );
        assert_eq!(decide(&table, true, open, None).forwarded, open);
        // Unowned names pass through even with every row Off.
        table.set_all(FxMode::Off);
        for name in [
            "sys_hit_normal",
            "sys_shield",
            "bg_finishhit",
            "mario_fireball",
        ] {
            let d = decide(&table, true, hash40(name), None);
            assert_eq!((d.fighter, d.forwarded), (None, hash40(name)), "{name}");
        }
    }

    #[test]
    fn variant_selection_and_the_cull_list_drive_reduced() {
        let table = ModeTable::new();
        let mut settings = Settings::default();
        settings.modes[Fighter::Kirby.index()] = FxMode::Reduced;
        settings.modes[Fighter::Pyra.index()] = FxMode::Reduced;
        settings.variants[moves::by_id("kirby_hammer").unwrap()] = 0;
        settings.cull = CullList::parse("eflame_attack100,kirby_onigoroshi_arc");
        table.load(&settings);
        let light = hash40("kirby_onigoroshi_light");
        // Variant 0: the move stays original.
        assert_eq!(
            decide(&table, true, light, None).intent,
            FxIntent::ReducedKeep
        );
        assert_eq!(decide(&table, true, light, None).forwarded, light);
        // The cull list nulls a kept layer and an unlisted name of a Reduced fighter.
        let arc = hash40("kirby_onigoroshi_arc");
        assert_eq!(decide(&table, true, arc, None).forwarded, NULL_EFFECT);
        let swing = hash40("eflame_attack100");
        let d = decide(&table, true, swing, None);
        assert_eq!(
            (d.fighter, d.intent),
            (Some(Fighter::Pyra), FxIntent::ReducedCull)
        );
        assert_eq!(
            decide(&table, false, swing, None).intent,
            FxIntent::ReducedRestore
        );
        // A listed name of a fighter that is On is untouched.
        table.set(Fighter::Pyra, FxMode::On);
        assert_eq!(decide(&table, true, swing, None).forwarded, swing);
        // Variant 3 culls the hammer body.
        assert!(table.select_variant(moves::by_id("kirby_hammer").unwrap(), 3));
        assert_eq!(
            decide(&table, true, hash40("kirby_onigoroshi_body"), None).forwarded,
            NULL_EFFECT
        );
    }

    #[test]
    fn shared_flash_follows_the_move_owner() {
        let table = ModeTable::new();
        let flash = hash40("sys_flash");
        let ness = fighter_ctx(0xa, "special_hi", 0x1de);
        let lucas = fighter_ctx(0x28, "special_hi", 0x1de);
        table.set(Fighter::Ness, FxMode::Off);
        assert_eq!(
            decide(&table, true, flash, Some(ness)).forwarded,
            NULL_EFFECT
        );
        assert_eq!(decide(&table, true, flash, Some(lucas)).forwarded, flash);
        assert_eq!(decide(&table, true, flash, None).forwarded, flash);
        let smash = hash40("sys_smash_flash");
        let seph = fighter_ctx(0x59, "attack_s4", 0x2f);
        assert_eq!(decide(&table, true, smash, Some(seph)).forwarded, smash);
        table.set(Fighter::Sephiroth, FxMode::Off);
        assert_eq!(
            decide(&table, true, smash, Some(seph)).forwarded,
            NULL_EFFECT
        );
    }

    #[test]
    fn the_same_inputs_match_the_single_mode_pipeline() {
        // With every row at one mode, the decision equals the global-mode pipeline.
        let selection = moves::default_selection();
        for mode in FxMode::ALL {
            let table = ModeTable::new();
            table.set_all(mode);
            for name in crate::effects::TARGET_NAMES
                .iter()
                .chain(crate::effects_catalog::LEGACY_NAMES)
            {
                let hash = hash40(name);
                let row_mode =
                    fighters::unscoped_fighter(hash).map_or(FxMode::On, |f| table.mode(f));
                for scope in [true, false] {
                    let intent =
                        effects_reduced::classify(row_mode, scope, hash, None, &selection, false);
                    let expected = eff_variant::forward(
                        row_mode,
                        scope,
                        effects_reduced::request_hash(intent, hash),
                        None,
                    );
                    assert_eq!(
                        decide(&table, scope, hash, None).forwarded,
                        expected,
                        "{name}"
                    );
                }
            }
        }
    }

    #[test]
    fn screen_overlay_and_critical_probe_mode() {
        let table = ModeTable::new();
        let screen = hash40("edge_fire3_screen2");
        assert_eq!(decide_screen(&table, true, screen), FxIntent::Passthrough);
        table.set(Fighter::Sephiroth, FxMode::Reduced);
        assert!(effects_reduced::skips_screen(decide_screen(
            &table, true, screen
        )));
        assert_eq!(
            decide_screen(&table, false, screen),
            FxIntent::ReducedRestore
        );
        table.set(Fighter::Sephiroth, FxMode::Off);
        assert_eq!(decide_screen(&table, true, screen), FxIntent::Suppress);
        assert_eq!(
            decide_screen(&table, true, hash40("sys_bg_black")),
            FxIntent::Passthrough
        );
        assert_eq!(critical_mode(&table), FxMode::On);
        table.set_all(FxMode::Off);
        assert_eq!(critical_mode(&table), FxMode::Off);
    }

    #[test]
    fn context_is_requested_for_every_move_or_sibling_dependent_name() {
        for name in crate::effects_catalog::CONTEXT_NAME_LIST {
            assert!(needs_context(hash40(name)), "{name}");
        }
        assert!(needs_context(eff_variant::SOURCE));
        assert!(needs_context(hash40("edge_shadowflare_start")));
        assert!(!needs_context(hash40("sys_hit_normal")));
        // Move layers ask too (shared names resolve by move context).
        assert!(needs_context(hash40("eflame_sword_open")));
        assert!(needs_context(hash40("kirby_onigoroshi_light")));
        assert!(!needs_context(hash40("eflame_attack100")));
    }
}
