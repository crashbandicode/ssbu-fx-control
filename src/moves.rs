//! Moves, the effect layers they request and their numbered Reduced variants.
//!
//! This is the unit of the 0.2 measurement harness ([`docs/HARNESS.md`]).
//! A move lists every character-prefixed parent request its scripts make at
//! the pinned dump (`PIN`), and 0..N **variants**: each variant is a cull set,
//! a subset of those layers that Reduced replaces with the `null` resource.
//! Variants are numbered from 1 and selected per move through the INI
//! (`variant.<move>=N`, 0 = keep the move original). A variant starts as a
//! `Candidate`; it becomes `Accepted` or `Rejected` only through the console
//! acceptance capture recorded in the ledger, never by appearance.
//!
//! The 0.1.x cull sets were chosen by appearance; where one existed for a
//! move below it is kept as a candidate so the console can rank it against
//! the new ones. Moves that no console capture has shown to drop frames
//! (Palutena's other moves, Sephiroth's other specials, Inkling smashes) have
//! no entry; their Off tables are untouched. Fighters with no move here at
//! all (Kazuya, Ike, Chrom, Roy, Ganondorf) left the plugin in 0.2.0.
//!
//! Layer names here are Hash40 request arguments of `EFFECT*` ACMD calls in
//! WuBoytH/SSBU-Dumped-Scripts at `PIN`. `shared` lists the `sys_*` requests
//! the same scripts make; they are never culled and belong to no fighter.
use crate::effect_context::{self, Context, Profile};
use crate::effects::hash40;
use crate::fighters::Fighter;
use std::sync::OnceLock;

pub const PIN: &str = "59004685238a3f6f93bae905d3a4e079701c40b4";

/// Measurement state of one variant. Only the ledger moves it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// Defined from the scripts, not yet measured on the console.
    Candidate,
    /// Passed the acceptance capture (no second below 59) on the console.
    Accepted,
    /// Measured and failed, kept for the record.
    Rejected,
}

#[derive(Clone, Copy, Debug)]
pub struct Variant {
    /// Layers this variant nulls. Every name must be in the move's `layers`.
    pub cull: &'static [&'static str],
    pub note: &'static str,
    pub status: Status,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    /// Stable id: the INI key suffix and Online Deluxe's `drs_moves` name.
    pub id: &'static str,
    pub label: &'static str,
    /// Row label on the Deluxe page, under the fighter's header.
    pub short: &'static str,
    pub fighter: Fighter,
    /// The policy's move context, when one exists. A move without a profile
    /// is matched by its layer names alone (they are unique to the move).
    pub profile: Option<Profile>,
    /// Character-prefixed parent requests of the move's scripts.
    pub layers: &'static [&'static str],
    /// `sys_*` requests of the same scripts (documentation; never culled).
    pub shared: &'static [&'static str],
    pub variants: &'static [Variant],
    /// Script files read at `PIN`, relative to `smashline/`.
    pub source: &'static str,
}

/// At most this many variants per move (the INI value and the cull mask).
pub const MAX_VARIANTS: usize = 8;

const FALCON_PUNCH_LAYERS: &[&str] = &[
    "captain_fp_flash",
    "captain_fp_hold",
    "captain_fp_body_l",
    "captain_fp_body_r",
    "captain_fp_wing_l",
    "captain_fp_wing_r",
    "captain_fp_hit_l",
    "captain_fp_hit_r",
];
const PYRA_SIDE_B_LAYERS: &[&str] = &[
    "eflame_sword_open",
    "eflame_blazeend_throw",
    "eflame_blazeend_sword",
    "eflame_blazeend_illusion",
    "eflame_blazeend_trail",
    "eflame_blazeend_trail_last",
    "eflame_blazeend_trail_last_back",
    "eflame_blazeend_trail_last_spin",
    "eflame_sword_close",
    "eflame_blazeend_return",
];
const PYRA_NEUTRAL_B_LAYERS: &[&str] = &[
    "eflame_sword_core_start",
    "eflame_sword_open",
    "eflame_nova_hold_body",
    "eflame_nova_hold_sword",
    "eflame_nova_sword",
    "eflame_nova_trail",
    "eflame_nova_ring1",
    "eflame_nova_explosion",
    "eflame_nova_explosion2",
    "eflame_nova_explosion3",
    "eflame_nova_sword_close",
];
const PYRA_UP_B_LAYERS: &[&str] = &[
    "eflame_promrevolt_flash",
    "eflame_sword_open",
    "eflame_sword_beam_m",
    "eflame_sword_fire",
    "eflame_sword_fire2",
    "eflame_sword_firetrail_end",
    "eflame_promrevolt_sword_stab",
    "eflame_promrevolt_sword_smoke",
    "eflame_promrevolt_sword_fire",
    "eflame_sword_close",
    "eflame_promrevolt_firepillar_ground",
    "eflame_promrevolt_firepillar",
    "eflame_promrevolt_firepillar_impact",
];
const MYTHRA_SPECIALS_LAYERS: &[&str] = &[
    "elight_sword_open",
    "elight_sword_light",
    "elight_sword_light_few",
];
const PALUTENA_EXPLOSIVE_FLAME_LAYERS: &[&str] = &[
    "palutena_bomb",
    "palutena_bomb_appear",
    "palutena_bomb_finish",
    "palutena_wand_light_trace",
    "palutena_wand_light2",
    "palutena_backlight",
];
const NESS_PK_THUNDER_LAYERS: &[&str] = &["ness_pkt_attack", "ness_pkt_bomb", "ness_pkt_bullet"];
const NESS_PK_FIRE_LAYERS: &[&str] = &["ness_pkfi_start", "ness_pkfi_bullet", "ness_pkfi_bomb"];
const LUCAS_PK_THUNDER_LAYERS: &[&str] =
    &["lucas_pkt_attack", "lucas_pkt_bomb", "lucas_pkt_bullet"];
const KIRBY_HAMMER_LAYERS: &[&str] = &[
    "kirby_onigoroshi_light",
    "kirby_onigoroshi_arc",
    "kirby_onigoroshi_max",
    "kirby_onigoroshi_wind",
    "kirby_onigoroshi_body",
    "kirby_onigoroshi_smoke",
];
const BYLETH_SPECIALS_LAYERS: &[&str] = &[
    "master_axeflare1",
    "master_axe_slash",
    "master_axeflare3",
    "master_axe_hold2",
];
const INKLING_ROLLER_LAYERS: &[&str] = &[
    "inkling_roller_splash_walk",
    "inkling_roller_splash_run",
    "inkling_roller_splash_air",
];
const SEPHIROTH_GIGAFLARE_LAYERS: &[&str] = &[
    "edge_fire1_sign",
    "edge_fire1_burst",
    "edge_fire2_burst",
    "edge_fire2_burst2",
    "edge_fire3_burst",
    "edge_fire3_burst2",
];

/// Every layer list, for the compile-time name-to-fighter map.
pub const NAME_TABLES: &[&[&str]] = &[
    FALCON_PUNCH_LAYERS,
    PYRA_SIDE_B_LAYERS,
    PYRA_NEUTRAL_B_LAYERS,
    PYRA_UP_B_LAYERS,
    MYTHRA_SPECIALS_LAYERS,
    PALUTENA_EXPLOSIVE_FLAME_LAYERS,
    NESS_PK_THUNDER_LAYERS,
    NESS_PK_FIRE_LAYERS,
    LUCAS_PK_THUNDER_LAYERS,
    KIRBY_HAMMER_LAYERS,
    BYLETH_SPECIALS_LAYERS,
    INKLING_ROLLER_LAYERS,
    SEPHIROTH_GIGAFLARE_LAYERS,
];

const fn candidate(cull: &'static [&'static str], note: &'static str) -> Variant {
    Variant {
        cull,
        note,
        status: Status::Candidate,
    }
}

/// Ledger order (`MOVE-LEDGER.md`, 2026-10-15). Append only: the index is the
/// INI/selection slot.
pub const MOVES: &[Move] = &[
    Move {
        id: "falcon_punch",
        label: "Falcon Punch",
        short: "Falcon Punch",
        fighter: Fighter::CaptainFalcon,
        profile: Some(Profile::FalconPunch),
        layers: FALCON_PUNCH_LAYERS,
        shared: &[],
        variants: &[
            candidate(
                &["captain_fp_wing_l", "captain_fp_wing_r"],
                "0.1.x: wings off, body and hold kept",
            ),
            candidate(
                &[
                    "captain_fp_wing_l",
                    "captain_fp_wing_r",
                    "captain_fp_hold",
                    "captain_fp_flash",
                ],
                "wings, wind-up hold and flash off; body and hit sparks kept",
            ),
        ],
        source: "lua2cpp_captain/captain/SpecialN.txt, captain_falconpunch/SpecialN.txt",
    },
    Move {
        id: "pyra_side_b",
        label: "Pyra Blazing End",
        short: "Blazing End",
        fighter: Fighter::Pyra,
        profile: None,
        layers: PYRA_SIDE_B_LAYERS,
        shared: &["sys_smash_flash_s", "sys_atk_smoke", "sys_item_get", "sys_down_smoke"],
        variants: &[
            candidate(
                &[
                    "eflame_blazeend_trail",
                    "eflame_blazeend_trail_last",
                    "eflame_blazeend_trail_last_back",
                    "eflame_blazeend_trail_last_spin",
                    "eflame_blazeend_illusion",
                ],
                "spin trails and flight illusion off; sword flame, throw and return kept",
            ),
            candidate(
                &[
                    "eflame_blazeend_trail",
                    "eflame_blazeend_trail_last",
                    "eflame_blazeend_trail_last_back",
                    "eflame_blazeend_trail_last_spin",
                    "eflame_blazeend_illusion",
                    "eflame_blazeend_sword",
                ],
                "variant 1 plus the sword's own flame body; throw, close and return kept",
            ),
            candidate(
                &[
                    "eflame_blazeend_trail",
                    "eflame_blazeend_trail_last",
                    "eflame_blazeend_trail_last_back",
                    "eflame_blazeend_trail_last_spin",
                    "eflame_blazeend_illusion",
                    "eflame_blazeend_sword",
                    "eflame_sword_close",
                ],
                "every Blazing End layer off except the throw burst and the return",
            ),
        ],
        source: "lua2cpp_eflame/eflame/SpecialS.txt SpecialAirS.txt SpecialSFlick.txt SpecialAirSFlick.txt SpecialSCatch.txt SpecialAirSCatch.txt; eflame_esword/FlyL.txt Rotate.txt Back.txt Hit.txt (Hop, Reflected, FlyFlick not read)",
    },
    Move {
        id: "pyra_neutral_b",
        label: "Pyra Flame Nova",
        short: "Flame Nova",
        fighter: Fighter::Pyra,
        profile: None,
        layers: PYRA_NEUTRAL_B_LAYERS,
        shared: &["sys_run_smoke", "sys_down_smoke"],
        variants: &[
            candidate(
                &["eflame_nova_trail", "eflame_nova_hold_body"],
                "charge body aura and spin trail off",
            ),
            candidate(
                &[
                    "eflame_nova_trail",
                    "eflame_nova_hold_body",
                    "eflame_nova_hold_sword",
                    "eflame_nova_ring1",
                ],
                "variant 1 plus the hold sword glow and the first ring",
            ),
            candidate(
                &[
                    "eflame_nova_trail",
                    "eflame_nova_hold_body",
                    "eflame_nova_hold_sword",
                    "eflame_nova_ring1",
                    "eflame_sword_core_start",
                    "eflame_nova_sword_close",
                ],
                "only the spinning sword and the explosions kept",
            ),
        ],
        source: "lua2cpp_eflame/eflame/SpecialNStart.txt SpecialNHold.txt SpecialN1.txt SpecialN4.txt (N2, N3 and the Common files not read: ring2/ring3 names unconfirmed)",
    },
    Move {
        id: "pyra_up_b",
        label: "Pyra Prominence Revolt",
        short: "Prominence Revolt",
        fighter: Fighter::Pyra,
        profile: None,
        layers: PYRA_UP_B_LAYERS,
        shared: &["sys_down_smoke", "sys_atk_smoke"],
        variants: &[
            candidate(
                &[
                    "eflame_sword_fire2",
                    "eflame_sword_firetrail_end",
                    "eflame_promrevolt_sword_smoke",
                    "eflame_promrevolt_firepillar_ground",
                ],
                "secondary sword fire, trail end, smoke and the pillar's ground layer off",
            ),
            candidate(
                &[
                    "eflame_sword_fire2",
                    "eflame_sword_firetrail_end",
                    "eflame_promrevolt_sword_smoke",
                    "eflame_promrevolt_firepillar_ground",
                    "eflame_sword_fire",
                    "eflame_promrevolt_sword_fire",
                    "eflame_promrevolt_firepillar_impact",
                ],
                "variant 1 plus every sword fire and the pillar impact; stab, flash, beam and pillar kept",
            ),
        ],
        source: "lua2cpp_eflame/eflame/SpecialHiStart.txt SpecialHi.txt; eflame_firepillar/SpecialHi.txt",
    },
    Move {
        id: "mythra_specials",
        label: "Mythra specials",
        short: "Specials",
        fighter: Fighter::Mythra,
        profile: None,
        layers: MYTHRA_SPECIALS_LAYERS,
        shared: &[],
        variants: &[candidate(
            &["elight_sword_light", "elight_sword_light_few"],
            "0.1.x character-wide: sword light layers off (the specials' own layers are not extracted yet)",
        )],
        source: "lua2cpp_elight sword scripts (0.1.x extraction); SpecialN/S/Hi not read",
    },
    Move {
        id: "palutena_explosive_flame",
        label: "Palutena Explosive Flame",
        short: "Explosive Flame",
        fighter: Fighter::Palutena,
        profile: Some(Profile::PalutenaSideB),
        layers: PALUTENA_EXPLOSIVE_FLAME_LAYERS,
        shared: &[],
        variants: &[candidate(
            &[
                "palutena_bomb_appear",
                "palutena_wand_light_trace",
                "palutena_wand_light2",
                "palutena_backlight",
            ],
            "0.1.x: appear flash, wand lights and backlight off; bomb and finish kept",
        )],
        source: "lua2cpp_palutena/palutena/SpecialS.txt; palutena_explosiveflame/Explode.txt",
    },
    Move {
        id: "ness_pk_thunder",
        label: "Ness PK Thunder",
        short: "PK Thunder",
        fighter: Fighter::Ness,
        profile: Some(Profile::NessUpB),
        layers: NESS_PK_THUNDER_LAYERS,
        shared: &["sys_flash"],
        variants: &[candidate(
            &["ness_pkt_bomb"],
            "0.1.x: launch burst off; attack and projectile kept",
        )],
        source: "lua2cpp_ness/ness/SpecialAirHi.txt; ness_pkthunder/Move.txt",
    },
    Move {
        id: "ness_pk_fire",
        label: "Ness PK Fire",
        short: "PK Fire",
        fighter: Fighter::Ness,
        profile: None,
        layers: NESS_PK_FIRE_LAYERS,
        shared: &["sys_h_smoke_a"],
        variants: &[
            candidate(&["ness_pkfi_start"], "hand flash off; bolt and pillar kept"),
            candidate(
                &["ness_pkfi_start", "ness_pkfi_bomb"],
                "hand flash and the fire pillar off; only the bolt kept (pillar hitbox then has no visual)",
            ),
        ],
        source: "lua2cpp_ness/ness/SpecialS.txt SpecialAirS.txt; ness_pkfire/Shoot.txt Pillar.txt",
    },
    Move {
        id: "lucas_pk_thunder",
        label: "Lucas PK Thunder",
        short: "PK Thunder",
        fighter: Fighter::Lucas,
        profile: Some(Profile::LucasUpB),
        layers: LUCAS_PK_THUNDER_LAYERS,
        shared: &["sys_flash"],
        variants: &[candidate(
            &["lucas_pkt_bomb"],
            "0.1.x: launch burst off; attack and projectile kept",
        )],
        source: "lua2cpp_lucas/lucas/SpecialAirHi.txt; lucas_pkthunder/Move.txt",
    },
    Move {
        id: "kirby_hammer",
        label: "Kirby Hammer",
        short: "Hammer",
        fighter: Fighter::Kirby,
        profile: None,
        layers: KIRBY_HAMMER_LAYERS,
        shared: &[
            "sys_flash",
            "sys_smash_flash",
            "sys_whirlwind_l",
            "sys_whirlwind_r",
            "sys_atk_smoke",
            "sys_run_smoke",
            "sys_dash_smoke",
        ],
        variants: &[
            candidate(
                &["kirby_onigoroshi_light"],
                "charge glow on the hammer off (held for the whole charge); swing layers kept",
            ),
            candidate(
                &[
                    "kirby_onigoroshi_light",
                    "kirby_onigoroshi_smoke",
                    "kirby_onigoroshi_wind",
                ],
                "variant 1 plus swing smoke and the aerial wind",
            ),
            candidate(
                &[
                    "kirby_onigoroshi_light",
                    "kirby_onigoroshi_smoke",
                    "kirby_onigoroshi_wind",
                    "kirby_onigoroshi_body",
                ],
                "only the swing arc and the max-charge burst kept",
            ),
        ],
        source: "lua2cpp_kirby/kirby/SpecialS.txt SpecialSStart.txt SpecialSHold*.txt SpecialSMax.txt SpecialSS.txt SpecialAirS.txt SpecialAirSMax.txt; kirby_hammer/SpecialS*.txt SpecialAirS*.txt",
    },
    Move {
        id: "byleth_specials",
        label: "Byleth specials",
        short: "Specials",
        fighter: Fighter::Byleth,
        profile: None,
        layers: BYLETH_SPECIALS_LAYERS,
        shared: &[],
        variants: &[candidate(
            &["master_axeflare3", "master_axe_hold2"],
            "0.1.x character-wide: second axe flare and second hold layer off",
        )],
        source: "lua2cpp_master axe scripts (0.1.x extraction); SpecialN/S/Hi/Lw not read",
    },
    Move {
        id: "inkling_roller",
        label: "Inkling Roller",
        short: "Roller",
        fighter: Fighter::Inkling,
        profile: None,
        layers: INKLING_ROLLER_LAYERS,
        shared: &[],
        // No request-cull variant: the 0.1.x Reduced was an EFF sibling
        // (archived with the feature in 0.2.0). The row offers On / Off.
        variants: &[],
        source: "inkling roller splash parents (0.1.x runtime check, Deluxe 24e714e7)",
    },
    Move {
        id: "sephiroth_gigaflare",
        label: "Sephiroth Gigaflare",
        short: "Gigaflare",
        fighter: Fighter::Sephiroth,
        profile: Some(Profile::EdgeNeutralB),
        layers: SEPHIROTH_GIGAFLARE_LAYERS,
        shared: &[],
        variants: &[candidate(
            &["edge_fire3_burst", "edge_fire2_burst", "edge_fire1_sign"],
            "0.1.x: first burst layer and the small sign off; burst2 layers kept, screen overlay skipped",
        )],
        source: "edge_fire/BurstL.txt BurstM.txt BurstS.txt",
    },
];

pub const COUNT: usize = MOVES.len();

/// Variant selection per move: 0 keeps the move original, `n` applies
/// `variants[n - 1]`. The default selects variant 1 where one exists.
pub type Selection = [u8; COUNT];

pub fn default_selection() -> Selection {
    let mut selection = [0u8; COUNT];
    for (slot, mv) in selection.iter_mut().zip(MOVES) {
        *slot = u8::from(!mv.variants.is_empty());
    }
    selection
}

pub fn by_id(id: &str) -> Option<usize> {
    MOVES.iter().position(|mv| mv.id == id)
}

/// Whether a fighter has at least one move with a variant.
pub fn fighter_has_variant(fighter: Fighter) -> bool {
    MOVES
        .iter()
        .any(|mv| mv.fighter == fighter && !mv.variants.is_empty())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Layer {
    /// Not a layer of any move.
    None,
    /// A layer the selected variant keeps (or the move is at variant 0).
    Keep,
    /// A layer the selected variant nulls.
    Cull,
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    hash: u64,
    mv: u8,
    /// Bit `n - 1` set when variant `n` culls this layer.
    mask: u8,
}

fn table() -> &'static [Entry] {
    static TABLE: OnceLock<Vec<Entry>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut entries = Vec::new();
        for (index, mv) in MOVES.iter().enumerate() {
            // A move without a variant has nothing to decide: its layers pass
            // through (the roller's Reduced is the EFF sibling).
            if mv.variants.is_empty() {
                continue;
            }
            for name in mv.layers {
                let hash = hash40(name);
                let mut mask = 0u8;
                for (variant, spec) in mv.variants.iter().enumerate() {
                    if spec.cull.contains(name) {
                        mask |= 1 << variant;
                    }
                }
                entries.push(Entry {
                    hash,
                    mv: index as u8,
                    mask,
                });
            }
        }
        entries.sort_by_key(|entry| entry.hash);
        entries
    })
}

fn entries(hash: u64) -> &'static [Entry] {
    let table = table();
    let start = table.partition_point(|entry| entry.hash < hash);
    let end = table.partition_point(|entry| entry.hash <= hash);
    &table[start..end]
}

/// Whether any move lists this request.
pub fn named(hash: u64) -> bool {
    !entries(hash).is_empty()
}

/// The move that owns a request. A move with a profile claims a request only
/// from an actor in that move (the 0.1.x rule: a Gigaflare layer requested
/// outside Gigaflare is preserved). A move without a profile claims its
/// layers by name; when a name is shared, the actor's profile decides first.
pub fn move_of(hash: u64, context: Option<Context>) -> Option<usize> {
    let hits = entries(hash);
    let profile = context.and_then(effect_context::profile);
    if let Some(profile) = profile {
        if let Some(hit) = hits
            .iter()
            .find(|hit| MOVES[hit.mv as usize].profile == Some(profile))
        {
            return Some(hit.mv as usize);
        }
    }
    hits.iter()
        .find(|hit| MOVES[hit.mv as usize].profile.is_none())
        .map(|hit| hit.mv as usize)
}

/// Role of a request under Reduced with `selection`.
pub fn layer(hash: u64, context: Option<Context>, selection: &Selection) -> Layer {
    let Some(mv) = move_of(hash, context) else {
        return Layer::None;
    };
    let Some(entry) = entries(hash).iter().find(|entry| entry.mv as usize == mv) else {
        return Layer::None;
    };
    match selection[mv] {
        0 => Layer::Keep,
        n if usize::from(n) <= MOVES[mv].variants.len() && entry.mask & (1 << (n - 1)) != 0 => {
            Layer::Cull
        }
        _ => Layer::Keep,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fighters;

    #[test]
    fn every_variant_culls_only_its_own_layers_and_ids_are_unique() {
        let mut ids = std::collections::BTreeSet::new();
        for mv in MOVES {
            assert!(ids.insert(mv.id), "duplicate move id {}", mv.id);
            assert!(mv.variants.len() <= MAX_VARIANTS, "{}", mv.id);
            assert!(!mv.layers.is_empty(), "{}", mv.id);
            for (number, variant) in mv.variants.iter().enumerate() {
                assert!(!variant.cull.is_empty(), "{} variant {}", mv.id, number + 1);
                for name in variant.cull {
                    assert!(
                        mv.layers.contains(name),
                        "{} variant {} culls {name}, not a layer",
                        mv.id,
                        number + 1
                    );
                    assert!(name_fighter_of(name) == Some(mv.fighter), "{name}");
                }
                assert!(
                    variant.cull.len() < mv.layers.len(),
                    "{} variant {} would null every layer: that is Off",
                    mv.id,
                    number + 1
                );
            }
            for name in mv.layers {
                assert_eq!(
                    name_fighter_of(name),
                    Some(mv.fighter),
                    "{name} in {}",
                    mv.id
                );
            }
            for name in mv.shared {
                assert!(
                    name.starts_with("sys_"),
                    "{name} in {} is not shared",
                    mv.id
                );
            }
            if let Some(profile) = mv.profile {
                assert_eq!(fighters::of_profile(profile), mv.fighter, "{}", mv.id);
            }
        }
        assert!(NAME_TABLES.len() == COUNT);
    }

    fn name_fighter_of(name: &str) -> Option<Fighter> {
        fighters::name_fighter(name)
    }

    #[test]
    fn default_selection_is_variant_one_where_a_variant_exists() {
        let selection = default_selection();
        for (slot, mv) in selection.iter().zip(MOVES) {
            assert_eq!(*slot, u8::from(!mv.variants.is_empty()), "{}", mv.id);
        }
        assert_eq!(by_id("kirby_hammer"), Some(9));
        assert_eq!(by_id("not_a_move"), None);
        assert!(fighter_has_variant(Fighter::Kirby));
        assert!(!fighter_has_variant(Fighter::Inkling));
        // A variant-less move decides nothing: its layers are not named.
        assert!(!named(hash40("inkling_roller_splash_run")));
        assert_eq!(
            layer(hash40("inkling_roller_splash_run"), None, &selection),
            Layer::None
        );
    }

    #[test]
    fn a_profiled_move_claims_only_its_own_actor() {
        let burst = hash40("edge_fire3_burst");
        let sel = default_selection();
        let gigaflare = Context {
            category: 0,
            kind: 0x59,
            motion: hash40("special_n_shoot"),
            status: 0x1e5,
        };
        let weapon = Context {
            category: 1,
            kind: 0x246,
            motion: 0,
            status: 4,
        };
        let palutena = Context {
            category: 0,
            kind: 0x36,
            motion: hash40("special_s"),
            status: 0x1dd,
        };
        assert_eq!(
            move_of(burst, Some(gigaflare)),
            by_id("sephiroth_gigaflare")
        );
        assert_eq!(move_of(burst, Some(weapon)), by_id("sephiroth_gigaflare"));
        assert_eq!(layer(burst, Some(weapon), &sel), Layer::Cull);
        // No context, or another move's actor: preserved (named, no layer).
        assert_eq!(move_of(burst, None), None);
        assert_eq!(move_of(burst, Some(palutena)), None);
        assert_eq!(layer(burst, None, &sel), Layer::None);
        assert!(named(burst));
    }

    #[test]
    fn layers_follow_the_selected_variant() {
        let mut selection = default_selection();
        let light = hash40("kirby_onigoroshi_light");
        let arc = hash40("kirby_onigoroshi_arc");
        let body = hash40("kirby_onigoroshi_body");
        assert_eq!(layer(light, None, &selection), Layer::Cull);
        assert_eq!(layer(arc, None, &selection), Layer::Keep);
        assert_eq!(layer(body, None, &selection), Layer::Keep);
        let kirby = by_id("kirby_hammer").unwrap();
        selection[kirby] = 3;
        assert_eq!(layer(body, None, &selection), Layer::Cull);
        assert_eq!(layer(arc, None, &selection), Layer::Keep);
        selection[kirby] = 0;
        assert_eq!(layer(light, None, &selection), Layer::Keep);
        // A number above the move's variants keeps everything.
        selection[kirby] = 7;
        assert_eq!(layer(light, None, &selection), Layer::Keep);
        assert_eq!(layer(hash40("sys_flash"), None, &selection), Layer::None);
        assert!(named(light) && !named(hash40("sys_flash")));
    }

    #[test]
    fn shared_pyra_names_resolve_by_context_then_by_the_unqualified_move() {
        let open = hash40("eflame_sword_open");
        // Listed by three Pyra moves, none with a profile: the first wins and
        // every variant keeps it.
        assert_eq!(move_of(open, None), by_id("pyra_side_b"));
        assert_eq!(layer(open, None, &default_selection()), Layer::Keep);
        let trace = hash40("palutena_wand_light_trace");
        let side = Context {
            category: 0,
            kind: 0x36,
            motion: hash40("special_s"),
            status: 0x1dd,
        };
        assert_eq!(
            move_of(trace, Some(side)),
            by_id("palutena_explosive_flame")
        );
        assert_eq!(layer(trace, Some(side), &default_selection()), Layer::Cull);
    }
}
