use std::mem::MaybeUninit;
use std::sync::{Mutex, MutexGuard};

use ssbu_fx_control::acmd::{replacements, Value};
use ssbu_fx_control::config::FxMode;
use ssbu_fx_control::eff_variant::{self, VariantAvailability, PATH_HASH, REDUCED, SOURCE};
use ssbu_fx_control::effect_context::Context;
use ssbu_fx_control::effects::hash40;
use ssbu_fx_control::fighters::Fighter;
use ssbu_fx_control::menu;
use ssbu_fx_control::policy::ModeTable;

fn value(raw: u64) -> Value {
    Value {
        kind: 2,
        metadata: 0,
        raw,
        indirect_return_padding: MaybeUninit::uninit(),
    }
}

fn up_air() -> Context {
    Context {
        category: 0,
        kind: 0x36,
        motion: hash40("attack_air_hi"),
        status: 0x36,
    }
}

fn other_air() -> Context {
    Context {
        motion: hash40("attack_air_f"),
        ..up_air()
    }
}

static GATES: Mutex<()> = Mutex::new(());

struct IsolatedGates<'a> {
    _guard: MutexGuard<'a, ()>,
}

impl IsolatedGates<'_> {
    fn enter() -> Self {
        let guard = GATES
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        eff_variant::reset_for_test();
        Self { _guard: guard }
    }
}

impl Drop for IsolatedGates<'_> {
    fn drop(&mut self) {
        eff_variant::reset_for_test();
    }
}

#[test]
fn up_air_maps_only_after_the_callback_succeeds() {
    let _gates = IsolatedGates::enter();
    assert_eq!(PATH_HASH, 0x0027_070a_c2fb);
    assert_eq!(SOURCE, hash40("palutena_backlight2"));
    assert_eq!(REDUCED, hash40("palutena_backlight2_reduced"));
    let args = [value(SOURCE)];

    assert_eq!(
        replacements(FxMode::Reduced, &args, 1, Some(up_air())),
        [None, None]
    );
    assert_eq!(
        eff_variant::forward(FxMode::On, true, SOURCE, Some(up_air())),
        SOURCE
    );
    assert_ne!(
        replacements(FxMode::Off, &args, 1, Some(up_air()))[0],
        Some(REDUCED)
    );

    eff_variant::set_availability_for_test(VariantAvailability::Ready);
    assert_eq!(
        replacements(FxMode::Reduced, &args, 1, Some(up_air()))[0],
        Some(REDUCED)
    );
    assert_eq!(
        replacements(FxMode::Reduced, &args, 1, Some(other_air()))[0],
        None
    );
    assert_eq!(
        replacements(FxMode::On, &args, 1, Some(up_air())),
        [None, None]
    );
    assert_eq!(
        eff_variant::forward(FxMode::Reduced, false, SOURCE, Some(up_air())),
        SOURCE
    );
    assert_ne!(
        replacements(FxMode::Off, &args, 1, Some(up_air()))[0],
        Some(REDUCED)
    );

    eff_variant::set_availability_for_test(VariantAvailability::Unavailable);
    assert_eq!(
        replacements(FxMode::Reduced, &args, 1, Some(up_air())),
        [None, None]
    );
    // Deluxe's single FX row is replaced by per-fighter rows. The Palutena row
    // keeps showing Reduced whatever the file state (never a silent Off), and
    // the read-only status row reports the file.
    let table = ModeTable::new();
    assert!(table.set(Fighter::Palutena, FxMode::Reduced));
    let palutena = 2 + Fighter::Palutena.index();
    for state in [
        VariantAvailability::Unavailable,
        VariantAvailability::Pending,
        VariantAvailability::Ready,
    ] {
        assert_eq!(
            menu::current(&table, palutena, true, state),
            Some(FxMode::Reduced as usize)
        );
        assert_eq!(
            menu::current(
                &table,
                menu::STATUS_ROW,
                true,
                menu::variant_status(&[state])
            ),
            Some(match state {
                VariantAvailability::Ready => 0,
                VariantAvailability::Pending => 1,
                VariantAvailability::Unavailable => 2,
            })
        );
    }
    assert_eq!(
        menu::variant_status(&[VariantAvailability::Ready, VariantAvailability::Unavailable]),
        VariantAvailability::Unavailable
    );
    eff_variant::reset_for_test();
}

#[test]
fn shadow_flare_maps_only_from_its_own_ready_gate() {
    let _gates = IsolatedGates::enter();
    let bullet = hash40("edge_shadowflare_bullet");
    let reduced = hash40("edge_shadowflare_bullet_reduced");
    let aura = hash40("edge_shadowflare_aura");
    let start = hash40("edge_shadowflare_start");
    let hold = hash40("edge_shadowflare_hold");
    let side = Context {
        category: 0,
        kind: 0x59,
        motion: hash40("special_s"),
        status: 0x1dd,
    };
    let flare = Context {
        category: 1,
        kind: 0x248,
        motion: 0,
        status: 0,
    };
    let args = [value(bullet)];
    eff_variant::set_availability_for_test(VariantAvailability::Unavailable);
    eff_variant::set_edge_availability_for_test(VariantAvailability::Ready);
    assert_eq!(
        replacements(FxMode::Reduced, &args, 1, Some(side))[0],
        Some(reduced)
    );
    assert_eq!(
        replacements(FxMode::Reduced, &[value(bullet)], 1, Some(flare))[0],
        Some(reduced)
    );
    assert_eq!(
        replacements(FxMode::Reduced, &[value(aura)], 1, Some(side))[0],
        None
    );
    assert_eq!(
        replacements(FxMode::Reduced, &[value(start)], 1, Some(side))[0],
        Some(hash40("edge_shadowflare_start_reduced"))
    );
    assert_eq!(
        replacements(FxMode::Reduced, &[value(hold)], 1, Some(side))[0],
        Some(hash40("edge_shadowflare_hold_reduced"))
    );
    assert_eq!(
        replacements(FxMode::On, &[value(start)], 1, Some(side))[0],
        None
    );
    assert_ne!(
        replacements(FxMode::Off, &[value(start)], 1, Some(side))[0],
        Some(hash40("edge_shadowflare_start_reduced"))
    );
    let smash = Context {
        category: 0,
        kind: 0x59,
        motion: hash40("attack_s4"),
        status: 0x2f,
    };
    assert_eq!(
        replacements(FxMode::Reduced, &[value(start)], 1, Some(smash))[0],
        None
    );
    assert_eq!(
        eff_variant::cleanup_alias(start),
        Some(hash40("edge_shadowflare_start_reduced"))
    );
    assert_eq!(
        replacements(FxMode::Reduced, &[value(SOURCE)], 1, Some(up_air()))[0],
        None
    );
    assert_ne!(
        replacements(FxMode::Off, &args, 1, Some(side))[0],
        Some(reduced)
    );
    eff_variant::set_edge_availability_for_test(VariantAvailability::Unavailable);
    eff_variant::set_availability_for_test(VariantAvailability::Ready);
    assert_eq!(replacements(FxMode::Reduced, &args, 1, Some(side))[0], None);
    assert_eq!(
        replacements(FxMode::Reduced, &[value(start)], 1, Some(side))[0],
        None
    );
    assert_eq!(
        eff_variant::cleanup_alias(start),
        Some(hash40("edge_shadowflare_start_reduced"))
    );
    assert_eq!(
        eff_variant::cleanup_alias(hash40("edge_shadowflare_start_reduced")),
        None
    );
    assert_eq!(
        replacements(FxMode::Reduced, &[value(SOURCE)], 1, Some(up_air()))[0],
        Some(REDUCED)
    );
    eff_variant::reset_for_test();
}

#[test]
fn roller_splash_uses_its_own_gate() {
    let _gates = IsolatedGates::enter();
    let splash = hash40("inkling_roller_splash_run");
    let reduced = hash40("inkling_roller_splash_run_reduced");
    eff_variant::set_edge_availability_for_test(VariantAvailability::Ready);
    eff_variant::set_availability_for_test(VariantAvailability::Ready);
    assert_eq!(
        replacements(FxMode::Reduced, &[value(splash)], 1, None)[0],
        None
    );
    eff_variant::set_ink_availability_for_test(VariantAvailability::Ready);
    assert_eq!(
        replacements(FxMode::Reduced, &[value(splash)], 1, None)[0],
        Some(reduced)
    );
    assert_eq!(replacements(FxMode::On, &[value(splash)], 1, None)[0], None);
    // Off is the catalog, never the sibling: the roller parents are Off-table
    // names (effects_catalog `off_nulls_proven_roller_splash_parents_only`).
    // The Deluxe copy of this test still expected `None` here.
    assert_eq!(
        replacements(FxMode::Off, &[value(splash)], 1, None)[0],
        Some(ssbu_fx_control::effects::NULL_EFFECT)
    );
    eff_variant::set_ink_availability_for_test(VariantAvailability::Unavailable);
    assert_eq!(
        replacements(FxMode::Reduced, &[value(SOURCE)], 1, Some(up_air()))[0],
        Some(REDUCED)
    );
    eff_variant::reset_for_test();
}

#[test]
fn chrom_life_gate_does_not_clear_edge_or_become_off() {
    let _gates = IsolatedGates::enter();
    let chrom = hash40("chrom_mc_1");
    let reduced = hash40("chrom_mc_1_reduced");
    let bullet = hash40("edge_shadowflare_bullet");
    let bullet_reduced = hash40("edge_shadowflare_bullet_reduced");
    let side = Context {
        category: 0,
        kind: 0x59,
        motion: hash40("special_s"),
        status: 0x1dd,
    };
    eff_variant::set_edge_availability_for_test(VariantAvailability::Ready);
    eff_variant::set_chrom_availability_for_test(VariantAvailability::Unavailable);
    assert_eq!(
        replacements(FxMode::Reduced, &[value(chrom)], 1, None)[0],
        None
    );
    assert_eq!(
        replacements(FxMode::Reduced, &[value(bullet)], 1, Some(side))[0],
        Some(bullet_reduced)
    );
    assert_eq!(eff_variant::edge_availability(), VariantAvailability::Ready);
    eff_variant::set_chrom_availability_for_test(VariantAvailability::Ready);
    assert_eq!(
        replacements(FxMode::Reduced, &[value(chrom)], 1, None)[0],
        Some(reduced)
    );
    assert_ne!(
        replacements(FxMode::Off, &[value(chrom)], 1, None)[0],
        Some(reduced)
    );
    assert_eq!(replacements(FxMode::On, &[value(chrom)], 1, None)[0], None);
    eff_variant::reset_for_test();
}

#[test]
fn siblings_follow_only_their_own_fighter_row() {
    use ssbu_fx_control::policy::decide;
    let _gates = IsolatedGates::enter();
    eff_variant::set_availability_for_test(VariantAvailability::Ready);
    eff_variant::set_ink_availability_for_test(VariantAvailability::Ready);
    let table = ModeTable::new();
    let splash = hash40("inkling_roller_splash_walk");
    // Palutena Reduced: the up-air sibling; Inkling still On.
    assert!(table.set(Fighter::Palutena, FxMode::Reduced));
    assert_eq!(
        decide(&table, true, SOURCE, Some(up_air())).forwarded,
        REDUCED
    );
    assert_eq!(
        decide(&table, true, SOURCE, Some(other_air())).forwarded,
        SOURCE
    );
    assert_eq!(decide(&table, true, splash, None).forwarded, splash);
    // Inkling Reduced too: its roller sibling. Out of scope: originals.
    assert!(table.set(Fighter::Inkling, FxMode::Reduced));
    assert_eq!(
        decide(&table, true, splash, None).forwarded,
        hash40("inkling_roller_splash_walk_reduced")
    );
    assert_eq!(decide(&table, false, splash, None).forwarded, splash);
    // Palutena Off is the Off catalog, never the sibling.
    assert!(table.set(Fighter::Palutena, FxMode::Off));
    assert_ne!(
        decide(&table, true, SOURCE, Some(up_air())).forwarded,
        REDUCED
    );
    eff_variant::reset_for_test();
}
