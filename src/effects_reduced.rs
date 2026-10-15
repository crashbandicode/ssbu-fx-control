//! Reduced FX decisions. This is not Off, and it does not scale requests.
//!
//! Reduced applies, per move, the variant the INI selects (`moves`): a
//! layer in the variant's cull set is nulled, every other layer of the move
//! is kept. A request that is a layer of a move another actor is performing
//! (same name, different move context) is preserved. The decision is by
//! request name and move context only; scale, joint and the other arguments
//! are never changed. The screen overlay of Gigaflare is the one skip.
use crate::{
    config::FxMode,
    effect_context::Context,
    effects::{self, NULL_EFFECT},
    moves::{self, Layer, Selection},
};
use std::sync::atomic::{AtomicU64, Ordering};

/// What one effect request should do. `Suppress` is Off. Reduced variants never
/// mean Off, and they do not change scale, joint, or other arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FxIntent {
    Passthrough,
    Suppress,
    ReducedCull,
    ReducedKeep,
    /// Reduced is selected and the move matches, but scope does not allow it.
    ReducedRestore,
    /// The name belongs to a move table, but this actor/move does not.
    ReducedPreserve,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FxCounters {
    pub suppressed: u64,
    pub reduced_cull: u64,
    pub reduced_keep: u64,
    pub reduced_restore: u64,
    pub reduced_preserve: u64,
    pub screen_skipped: u64,
    pub mode_restored: u64,
}

static SUPPRESSED: AtomicU64 = AtomicU64::new(0);
static REDUCED_CULL: AtomicU64 = AtomicU64::new(0);
static REDUCED_KEEP: AtomicU64 = AtomicU64::new(0);
static REDUCED_RESTORE: AtomicU64 = AtomicU64::new(0);
static REDUCED_PRESERVE: AtomicU64 = AtomicU64::new(0);
static SCREEN_SKIPPED: AtomicU64 = AtomicU64::new(0);
static MODE_RESTORED: AtomicU64 = AtomicU64::new(0);

/// The name is a layer of some move (the context lookup is worth it).
pub fn reduced_named(hash: u64) -> bool {
    moves::named(hash)
}

/// In-scope Off and Reduced request policy. `scope_allows` is false for
/// online-only scope while the match is not online. `selection` is the
/// per-move variant choice; `extra_cull` is the INI bisection list hit.
pub fn classify(
    mode: FxMode,
    scope_allows: bool,
    hash: u64,
    context: Option<Context>,
    selection: &Selection,
    extra_cull: bool,
) -> FxIntent {
    if mode == FxMode::Off {
        if !scope_allows {
            return FxIntent::Passthrough;
        }
        return if effects::rewrite_with_context(FxMode::Off, hash, context) == hash {
            FxIntent::Passthrough
        } else {
            FxIntent::Suppress
        };
    }
    if mode != FxMode::Reduced {
        return FxIntent::Passthrough;
    }
    let layer = moves::layer(hash, context, selection);
    match (layer, scope_allows) {
        (Layer::Cull, true) => FxIntent::ReducedCull,
        (Layer::Keep, true) if extra_cull => FxIntent::ReducedCull,
        (Layer::Keep, true) => FxIntent::ReducedKeep,
        (Layer::Cull | Layer::Keep, false) => FxIntent::ReducedRestore,
        (Layer::None, true) if extra_cull => FxIntent::ReducedCull,
        (Layer::None, false) if extra_cull => FxIntent::ReducedRestore,
        (Layer::None, _) if reduced_named(hash) => FxIntent::ReducedPreserve,
        _ => FxIntent::Passthrough,
    }
}

/// Fullscreen Gigaflare overlay. Peer `ssbu_screen_fx_off` blocks this one name.
/// Skipping the original screen call is the existing Off behavior; Reduced uses
/// the same skip and does not null a different asset.
pub fn classify_screen(mode: FxMode, scope_allows: bool, hash: u64) -> FxIntent {
    if hash != effects::hash40("edge_fire3_screen2") {
        return FxIntent::Passthrough;
    }
    match (mode, scope_allows) {
        (FxMode::Off, true) => FxIntent::Suppress,
        (FxMode::Reduced, true) => FxIntent::ReducedCull,
        (FxMode::Reduced, false) => FxIntent::ReducedRestore,
        _ => FxIntent::Passthrough,
    }
}

pub fn skips_screen(intent: FxIntent) -> bool {
    matches!(intent, FxIntent::Suppress | FxIntent::ReducedCull)
}

/// Hash forwarded to the original request. Screen skips must not use this.
pub fn request_hash(intent: FxIntent, hash: u64) -> u64 {
    match intent {
        FxIntent::Suppress | FxIntent::ReducedCull => NULL_EFFECT,
        _ => hash,
    }
}

pub fn accumulate(counters: &mut FxCounters, intent: FxIntent, screen: bool) {
    match intent {
        FxIntent::Passthrough => {}
        FxIntent::Suppress => {
            if screen {
                counters.screen_skipped = counters.screen_skipped.saturating_add(1);
            } else {
                counters.suppressed = counters.suppressed.saturating_add(1);
            }
        }
        FxIntent::ReducedCull => {
            counters.reduced_cull = counters.reduced_cull.saturating_add(1);
            if screen {
                counters.screen_skipped = counters.screen_skipped.saturating_add(1);
            }
        }
        FxIntent::ReducedKeep => counters.reduced_keep = counters.reduced_keep.saturating_add(1),
        FxIntent::ReducedRestore => {
            counters.reduced_restore = counters.reduced_restore.saturating_add(1);
        }
        FxIntent::ReducedPreserve => {
            counters.reduced_preserve = counters.reduced_preserve.saturating_add(1);
        }
    }
}

/// Returns the updated primary counter, or 0 for passthrough. Runtime logging
/// samples that total; it is not a performance measurement.
pub fn observe(intent: FxIntent, screen: bool) -> u64 {
    let mut one = FxCounters::default();
    accumulate(&mut one, intent, screen);
    let mut primary = 0;
    if one.screen_skipped > 0 {
        primary =
            SCREEN_SKIPPED.fetch_add(one.screen_skipped, Ordering::Relaxed) + one.screen_skipped;
    }
    let (slot, amount) = if one.suppressed > 0 {
        (Some(&SUPPRESSED), one.suppressed)
    } else if one.reduced_cull > 0 {
        (Some(&REDUCED_CULL), one.reduced_cull)
    } else if one.reduced_keep > 0 {
        (Some(&REDUCED_KEEP), one.reduced_keep)
    } else if one.reduced_restore > 0 {
        (Some(&REDUCED_RESTORE), one.reduced_restore)
    } else if one.reduced_preserve > 0 {
        (Some(&REDUCED_PRESERVE), one.reduced_preserve)
    } else {
        (None, 0)
    };
    if let Some(slot) = slot {
        primary = slot.fetch_add(amount, Ordering::Relaxed) + amount;
    }
    primary
}

pub fn counters() -> FxCounters {
    FxCounters {
        suppressed: SUPPRESSED.load(Ordering::Relaxed),
        reduced_cull: REDUCED_CULL.load(Ordering::Relaxed),
        reduced_keep: REDUCED_KEEP.load(Ordering::Relaxed),
        reduced_restore: REDUCED_RESTORE.load(Ordering::Relaxed),
        reduced_preserve: REDUCED_PRESERVE.load(Ordering::Relaxed),
        screen_skipped: SCREEN_SKIPPED.load(Ordering::Relaxed),
        mode_restored: MODE_RESTORED.load(Ordering::Relaxed),
    }
}

/// Leaving Reduced for On restores original requests on later creations.
/// Switching to Off is a different policy and is not restoration.
pub fn restores_full_visuals(previous: FxMode, next: FxMode) -> bool {
    previous == FxMode::Reduced && next == FxMode::On
}

pub fn observe_mode(previous: FxMode, next: FxMode) {
    if restores_full_visuals(previous, next) {
        MODE_RESTORED.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::hash40;

    fn edge_fighter() -> Context {
        Context {
            category: 0,
            kind: 0x59,
            motion: hash40("special_n_shoot"),
            status: 0x1e5,
        }
    }

    fn edge_weapon() -> Context {
        Context {
            category: 1,
            kind: 0x246,
            motion: 0,
            status: 4,
        }
    }

    fn selection() -> Selection {
        moves::default_selection()
    }

    #[test]
    fn gigaflare_variant_one_culls_the_named_layers_and_keeps_burst2() {
        let cull = hash40("edge_fire3_burst");
        let keep = hash40("edge_fire3_burst2");
        let sign = hash40("edge_fire1_sign");
        let sel = selection();
        // Without a Gigaflare actor the layer is another move's: preserved.
        assert_eq!(
            classify(FxMode::Reduced, true, cull, None, &sel, false),
            FxIntent::ReducedPreserve
        );
        assert_eq!(request_hash(FxIntent::ReducedPreserve, cull), cull);
        for context in [Some(edge_fighter()), Some(edge_weapon())] {
            assert_eq!(
                classify(FxMode::Reduced, true, cull, context, &sel, false),
                FxIntent::ReducedCull
            );
            assert_eq!(request_hash(FxIntent::ReducedCull, cull), NULL_EFFECT);
            assert_eq!(
                classify(FxMode::Reduced, true, keep, context, &sel, false),
                FxIntent::ReducedKeep
            );
            assert_eq!(request_hash(FxIntent::ReducedKeep, keep), keep);
            assert_eq!(
                classify(FxMode::Reduced, true, sign, context, &sel, false),
                FxIntent::ReducedCull
            );
        }
        assert_eq!(
            classify(
                FxMode::Reduced,
                true,
                hash40("edge_fire3_smoke"),
                Some(edge_fighter()),
                &sel,
                false
            ),
            FxIntent::Passthrough
        );
        assert_eq!(
            classify(FxMode::Off, true, cull, None, &sel, false),
            FxIntent::Suppress
        );
        assert_eq!(
            classify(FxMode::On, true, cull, Some(edge_fighter()), &sel, false),
            FxIntent::Passthrough
        );
        assert_eq!(
            classify(
                FxMode::Reduced,
                false,
                keep,
                Some(edge_fighter()),
                &sel,
                false
            ),
            FxIntent::ReducedRestore
        );
        assert_eq!(request_hash(FxIntent::ReducedRestore, keep), keep);
    }

    #[test]
    fn variant_zero_keeps_every_layer_and_higher_variants_cull_more() {
        let mut sel = selection();
        let kirby = moves::by_id("kirby_hammer").unwrap();
        let light = hash40("kirby_onigoroshi_light");
        let body = hash40("kirby_onigoroshi_body");
        assert_eq!(
            classify(FxMode::Reduced, true, light, None, &sel, false),
            FxIntent::ReducedCull
        );
        assert_eq!(
            classify(FxMode::Reduced, true, body, None, &sel, false),
            FxIntent::ReducedKeep
        );
        sel[kirby] = 0;
        assert_eq!(
            classify(FxMode::Reduced, true, light, None, &sel, false),
            FxIntent::ReducedKeep
        );
        sel[kirby] = 3;
        assert_eq!(
            classify(FxMode::Reduced, true, body, None, &sel, false),
            FxIntent::ReducedCull
        );
        // Out of scope the matched layers are restored, whatever the variant.
        assert_eq!(
            classify(FxMode::Reduced, false, body, None, &sel, false),
            FxIntent::ReducedRestore
        );
    }

    #[test]
    fn the_extra_cull_list_nulls_any_name_under_reduced_in_scope() {
        let sel = selection();
        // Not in any Off table or move: Off leaves it alone, the list nulls it.
        let unlisted = hash40("kirby_test_layer");
        assert_eq!(
            classify(FxMode::Reduced, true, unlisted, None, &sel, false),
            FxIntent::Passthrough
        );
        assert_eq!(
            classify(FxMode::Reduced, true, unlisted, None, &sel, true),
            FxIntent::ReducedCull
        );
        assert_eq!(
            classify(FxMode::Reduced, false, unlisted, None, &sel, true),
            FxIntent::ReducedRestore
        );
        // A kept layer named in the list is culled too (bisecting a variant).
        let arc = hash40("kirby_onigoroshi_arc");
        assert_eq!(
            classify(FxMode::Reduced, true, arc, None, &sel, true),
            FxIntent::ReducedCull
        );
        // On and Off ignore the list.
        assert_eq!(
            classify(FxMode::On, true, unlisted, None, &sel, true),
            FxIntent::Passthrough
        );
        assert_eq!(
            classify(FxMode::Off, true, unlisted, None, &sel, true),
            FxIntent::Passthrough
        );
    }

    #[test]
    fn screen_overlay_and_counters() {
        let screen = hash40("edge_fire3_screen2");
        assert_eq!(
            classify_screen(FxMode::Reduced, true, screen),
            FxIntent::ReducedCull
        );
        assert!(skips_screen(classify_screen(FxMode::Off, true, screen)));
        assert_eq!(
            classify_screen(FxMode::Reduced, false, screen),
            FxIntent::ReducedRestore
        );
        assert_eq!(
            classify_screen(FxMode::Reduced, true, hash40("sys_bg_black")),
            FxIntent::Passthrough
        );
        let mut counters = FxCounters::default();
        accumulate(&mut counters, FxIntent::ReducedCull, true);
        accumulate(&mut counters, FxIntent::Suppress, false);
        accumulate(&mut counters, FxIntent::ReducedKeep, false);
        accumulate(&mut counters, FxIntent::ReducedPreserve, false);
        accumulate(&mut counters, FxIntent::ReducedRestore, false);
        accumulate(&mut counters, FxIntent::Passthrough, false);
        assert_eq!(
            counters,
            FxCounters {
                suppressed: 1,
                reduced_cull: 1,
                reduced_keep: 1,
                reduced_restore: 1,
                reduced_preserve: 1,
                screen_skipped: 1,
                mode_restored: 0,
            }
        );
        assert!(restores_full_visuals(FxMode::Reduced, FxMode::On));
        assert!(!restores_full_visuals(FxMode::Reduced, FxMode::Off));
        assert!(!restores_full_visuals(FxMode::Off, FxMode::On));
    }
}
