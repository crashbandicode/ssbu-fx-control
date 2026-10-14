//! Which fighter owns an effect request.
//!
//! A request belongs to a fighter in one of two ways:
//!
//! * **Actor context.** When the request carries a context (the policy fetched
//!   the requesting object's category, kind, motion and status), a matching
//!   move profile names the fighter (weapons included: Falcon Punch bird,
//!   PK Thunder, Explosive Flame, Gigaflare burst, Shadow Flare orbs). A
//!   tracked fighter object with no matching move is still that fighter.
//! * **Name.** Otherwise the effect name decides. The map below is built at
//!   compile time from every name the policy can act on without a move
//!   context: the Off tables (`effects::TARGET_NAMES`,
//!   `effects_catalog::LEGACY_NAMES`), the character-wide Reduced tables, the
//!   EFF-sibling parents that need no move and the Gigaflare screen overlay.
//!   Each name is classified by its character prefix; a name that matches no
//!   prefix, or more than one, fails the build.
//!
//! A request that maps to no fighter passes through untouched.
use crate::effect_context::{self, Context, Profile};
use crate::effects::hash40;

/// Covered fighters in menu order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Fighter {
    Kazuya,
    CaptainFalcon,
    Ness,
    Lucas,
    Ike,
    Chrom,
    Roy,
    Palutena,
    Sephiroth,
    Pyra,
    Mythra,
    Byleth,
    Inkling,
    Ganondorf,
}

pub const COUNT: usize = 14;

pub const ALL: [Fighter; COUNT] = [
    Fighter::Kazuya,
    Fighter::CaptainFalcon,
    Fighter::Ness,
    Fighter::Lucas,
    Fighter::Ike,
    Fighter::Chrom,
    Fighter::Roy,
    Fighter::Palutena,
    Fighter::Sephiroth,
    Fighter::Pyra,
    Fighter::Mythra,
    Fighter::Byleth,
    Fighter::Inkling,
    Fighter::Ganondorf,
];

impl Fighter {
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Stable INI key and menu option id.
    pub const fn id(self) -> &'static str {
        match self {
            Fighter::Kazuya => "kazuya",
            Fighter::CaptainFalcon => "captain_falcon",
            Fighter::Ness => "ness",
            Fighter::Lucas => "lucas",
            Fighter::Ike => "ike",
            Fighter::Chrom => "chrom",
            Fighter::Roy => "roy",
            Fighter::Palutena => "palutena",
            Fighter::Sephiroth => "sephiroth",
            Fighter::Pyra => "pyra",
            Fighter::Mythra => "mythra",
            Fighter::Byleth => "byleth",
            Fighter::Inkling => "inkling",
            Fighter::Ganondorf => "ganondorf",
        }
    }

    /// Display label (at most 24 bytes).
    pub const fn label(self) -> &'static str {
        match self {
            Fighter::Kazuya => "Kazuya",
            Fighter::CaptainFalcon => "Captain Falcon",
            Fighter::Ness => "Ness",
            Fighter::Lucas => "Lucas",
            Fighter::Ike => "Ike",
            Fighter::Chrom => "Chrom",
            Fighter::Roy => "Roy",
            Fighter::Palutena => "Palutena",
            Fighter::Sephiroth => "Sephiroth",
            Fighter::Pyra => "Pyra",
            Fighter::Mythra => "Mythra",
            Fighter::Byleth => "Byleth",
            Fighter::Inkling => "Inkling",
            Fighter::Ganondorf => "Ganondorf",
        }
    }

    /// True when the fighter has at least one Reduced keep/cull table entry or
    /// an EFF sibling. A fighter without one is offered On / Off only, and
    /// "All = Reduced" leaves it On: Reduced never silently performs Off.
    /// `tests::reduced_support_matches_the_tables` derives this from the tables.
    pub const fn has_reduced(self) -> bool {
        match self {
            // Pyra, Mythra and Byleth have character-wide tables only (partial:
            // a few sword/axe layers). Every other row has move tables or a
            // sibling.
            Fighter::Kazuya
            | Fighter::CaptainFalcon
            | Fighter::Ness
            | Fighter::Lucas
            | Fighter::Ike
            | Fighter::Chrom
            | Fighter::Roy
            | Fighter::Palutena
            | Fighter::Sephiroth
            | Fighter::Pyra
            | Fighter::Mythra
            | Fighter::Byleth
            | Fighter::Inkling
            | Fighter::Ganondorf => true,
        }
    }

    pub fn from_id(id: &str) -> Option<Fighter> {
        ALL.iter().copied().find(|fighter| fighter.id() == id)
    }
}

/// Fighter of a move profile. `Edge*` is Sephiroth.
pub const fn of_profile(profile: Profile) -> Fighter {
    match profile {
        Profile::KazuyaFSmash => Fighter::Kazuya,
        Profile::FalconPunch => Fighter::CaptainFalcon,
        Profile::NessUpB => Fighter::Ness,
        Profile::LucasUpB => Fighter::Lucas,
        Profile::IkeNeutralB => Fighter::Ike,
        Profile::ChromNeutralB => Fighter::Chrom,
        Profile::RoyNeutralB => Fighter::Roy,
        Profile::PalutenaRapidJab
        | Profile::PalutenaNeutralB
        | Profile::PalutenaUpB
        | Profile::PalutenaSideB
        | Profile::PalutenaUpSmash
        | Profile::PalutenaTilts
        | Profile::PalutenaWingSmash
        | Profile::PalutenaUpAir => Fighter::Palutena,
        Profile::EdgeFSmash
        | Profile::EdgePurpleDash
        | Profile::EdgeNeutralB
        | Profile::EdgeDownB
        | Profile::EdgeSideB => Fighter::Sephiroth,
    }
}

/// Covered fighter of a `FIGHTER_KIND_*` (category 0 only; weapon kinds
/// overlap fighter kinds and are never read here). Constants are the ones
/// `effect_context` already tracks (skyline-smash bed642e).
pub const fn of_fighter_kind(kind: i32) -> Option<Fighter> {
    match kind {
        0x5c => Some(Fighter::Kazuya),
        0xb => Some(Fighter::CaptainFalcon),
        0xa => Some(Fighter::Ness),
        0x28 => Some(Fighter::Lucas),
        0x23 => Some(Fighter::Ike),
        0x1b => Some(Fighter::Chrom),
        0x1a => Some(Fighter::Roy),
        0x36 => Some(Fighter::Palutena),
        0x59 => Some(Fighter::Sephiroth),
        _ => None,
    }
}

/// Character prefixes of effect names. Exactly one must match each mapped name.
const PREFIXES: &[(&str, Fighter)] = &[
    ("demon_", Fighter::Kazuya),
    ("captain_", Fighter::CaptainFalcon),
    ("ness_", Fighter::Ness),
    ("P_Ness", Fighter::Ness),
    ("lucas_", Fighter::Lucas),
    ("P_Lucas", Fighter::Lucas),
    ("ike_", Fighter::Ike),
    ("P_Ike", Fighter::Ike),
    ("chrom_", Fighter::Chrom),
    ("P_Chrom", Fighter::Chrom),
    ("roy_", Fighter::Roy),
    ("P_Roy", Fighter::Roy),
    ("palutena_", Fighter::Palutena),
    ("P_Palutena", Fighter::Palutena),
    ("edge_", Fighter::Sephiroth),
    ("P_Edge", Fighter::Sephiroth),
    ("eflame_", Fighter::Pyra),
    ("P_Eflame", Fighter::Pyra),
    ("elight_", Fighter::Mythra),
    ("P_Elight", Fighter::Mythra),
    ("master_", Fighter::Byleth),
    ("inkling_", Fighter::Inkling),
    ("ganon_", Fighter::Ganondorf),
];

/// Names the policy acts on without a move context that are in neither Off
/// table nor a character-wide Reduced table: EFF-sibling parents resolved
/// without a move (`chrom_mc_1`), the up-air sibling parent (needs a move,
/// listed so a context-less request still names Palutena) and the Gigaflare
/// screen overlay skipped by `req_screen`.
const EXTRA_NAMES: &[&str] = &["chrom_mc_1", "palutena_backlight2", "edge_fire3_screen2"];

const fn starts_with(name: &str, prefix: &str) -> bool {
    let name = name.as_bytes();
    let prefix = prefix.as_bytes();
    if name.len() < prefix.len() {
        return false;
    }
    let mut index = 0;
    while index < prefix.len() {
        if name[index] != prefix[index] {
            return false;
        }
        index += 1;
    }
    true
}

/// The fighter whose prefix starts `name`; `None` for no match or an
/// ambiguous one.
pub const fn name_fighter(name: &str) -> Option<Fighter> {
    let mut found = None;
    let mut index = 0;
    while index < PREFIXES.len() {
        if starts_with(name, PREFIXES[index].0) {
            if found.is_some() {
                return None;
            }
            found = Some(PREFIXES[index].1);
        }
        index += 1;
    }
    found
}

const SOURCES: &[&[&str]] = &[
    crate::effects::TARGET_NAMES,
    crate::effects_catalog::LEGACY_NAMES,
    EXTRA_NAMES,
];

const fn total(groups: &[&[&str]], more: &[&[&str]]) -> usize {
    let mut count = 0;
    let mut index = 0;
    while index < groups.len() {
        count += groups[index].len();
        index += 1;
    }
    index = 0;
    while index < more.len() {
        count += more[index].len();
        index += 1;
    }
    count
}

const MAP_LEN: usize = total(SOURCES, crate::effects_reduced::GLOBAL_NAME_TABLES);

const fn insert(map: &mut [(u64, u8)], filled: usize, name: &str) {
    let fighter = match name_fighter(name) {
        Some(fighter) => fighter,
        None => panic!("an FX name maps to no fighter or to several"),
    };
    let entry = (hash40(name), fighter as u8);
    let mut slot = filled;
    while slot > 0 && map[slot - 1].0 > entry.0 {
        map[slot] = map[slot - 1];
        slot -= 1;
    }
    map[slot] = entry;
}

const fn build() -> [(u64, u8); MAP_LEN] {
    let mut map = [(0u64, 0u8); MAP_LEN];
    let mut filled = 0;
    let levels: [&[&[&str]]; 2] = [SOURCES, crate::effects_reduced::GLOBAL_NAME_TABLES];
    let mut level = 0;
    while level < levels.len() {
        let groups = levels[level];
        let mut group = 0;
        while group < groups.len() {
            let names = groups[group];
            let mut index = 0;
            while index < names.len() {
                insert(&mut map, filled, names[index]);
                filled += 1;
                index += 1;
            }
            group += 1;
        }
        level += 1;
    }
    map
}

/// Sorted by hash. A name listed in two tables appears twice with the same fighter.
const MAP: [(u64, u8); MAP_LEN] = build();

/// Fighter that owns a context-free effect name, or `None`.
pub fn unscoped_fighter(hash: u64) -> Option<Fighter> {
    let slot = MAP.binary_search_by(|entry| entry.0.cmp(&hash)).ok()?;
    ALL.get(MAP[slot].1 as usize).copied()
}

/// Fighter whose mode decides this request, or `None` to pass it through.
pub fn fighter_for(hash: u64, context: Option<Context>) -> Option<Fighter> {
    if let Some(context) = context {
        if let Some(profile) = effect_context::profile(context) {
            return Some(of_profile(profile));
        }
        if context.category == 0 {
            if let Some(fighter) = of_fighter_kind(context.kind) {
                return Some(fighter);
            }
        }
    }
    unscoped_fighter(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect_context::Profile;
    use crate::{effects, effects_catalog, effects_reduced};
    use std::collections::BTreeSet;

    const PROFILES: [Profile; 20] = [
        Profile::KazuyaFSmash,
        Profile::FalconPunch,
        Profile::NessUpB,
        Profile::LucasUpB,
        Profile::IkeNeutralB,
        Profile::ChromNeutralB,
        Profile::RoyNeutralB,
        Profile::PalutenaRapidJab,
        Profile::PalutenaNeutralB,
        Profile::PalutenaUpB,
        Profile::PalutenaSideB,
        Profile::PalutenaUpSmash,
        Profile::PalutenaTilts,
        Profile::PalutenaWingSmash,
        Profile::PalutenaUpAir,
        Profile::EdgeFSmash,
        Profile::EdgePurpleDash,
        Profile::EdgeNeutralB,
        Profile::EdgeDownB,
        Profile::EdgeSideB,
    ];

    /// Names inside a fighter's move table that carry another fighter's prefix
    /// or none. They are attributed through the requesting actor's move
    /// context, never through the name, so the owner follows the move.
    const SHARED_NAMES: &[&str] = &[
        // Ness and Lucas up-B; Sephiroth smashes.
        "sys_flash",
        "sys_smash_flash",
        "sys_smash_flash_s",
        // Ike's pillar layers reused by Chrom and Roy neutral-B.
        "ike_firepiller02",
        "ike_fireground00",
        "ike_firepiller00",
        // Sephiroth emitter-style parents without a character prefix.
        "aura1",
        "aura2",
        "burn1",
        "circle1",
        "fire1B",
        "fire1_bk",
        "fireBlack",
        "flare1",
        "flare2",
        "flare3",
        "impact2",
        "impact_flash1",
        "impactring1",
        "impactring1b",
        "impactring2",
        "impactring2_b",
        "light1_bk",
        "lightning1",
        "lightning2",
        "smoke1",
        "smoke2",
        "sparks2",
        "sphere1",
        "Flare1",
        "FlashLine1",
        "flareblack1",
        "light",
        "light3",
        "line1",
        "line1_1",
        "line2",
        "line2_1",
        "line_rainbow1",
        "fire1",
        "fire2",
        "impact_star_b1",
        "impact_star_b2",
        "impact_star_f1",
        "impact_star_f2",
        "debris1",
        "fire3",
        "flash1",
        "flash2",
        "ring1",
        "sparkline1",
    ];

    fn sample(profile: Profile) -> Context {
        effect_context::rule_samples()
            .find(|(p, _)| *p == profile)
            .map(|(_, context)| context)
            .expect("every profile has a rule")
    }

    fn prefix_matches(name: &str) -> Vec<Fighter> {
        PREFIXES
            .iter()
            .filter(|(prefix, _)| name.starts_with(prefix))
            .map(|(_, fighter)| *fighter)
            .collect()
    }

    #[test]
    fn every_off_table_name_maps_to_exactly_one_fighter() {
        let mut seen = 0;
        for name in effects::TARGET_NAMES
            .iter()
            .chain(effects_catalog::LEGACY_NAMES)
        {
            let matches = prefix_matches(name);
            assert_eq!(matches.len(), 1, "{name} -> {matches:?}");
            assert_eq!(unscoped_fighter(hash40(name)), Some(matches[0]), "{name}");
            assert_eq!(fighter_for(hash40(name), None), Some(matches[0]), "{name}");
            seen += 1;
        }
        assert_eq!(
            seen,
            effects::TARGET_NAMES.len() + effects_catalog::LEGACY_NAMES.len()
        );
    }

    #[test]
    fn character_wide_reduced_names_and_extras_map() {
        for names in effects_reduced::GLOBAL_NAME_TABLES
            .iter()
            .chain([EXTRA_NAMES].iter())
        {
            for name in names.iter() {
                assert_eq!(prefix_matches(name).len(), 1, "{name}");
                assert!(unscoped_fighter(hash40(name)).is_some(), "{name}");
            }
        }
        for (name, fighter) in [
            ("eflame_sword_firetrail", Fighter::Pyra),
            ("P_EflameSwordAura", Fighter::Pyra),
            ("elight_sword_light", Fighter::Mythra),
            ("P_ElightBusterHold", Fighter::Mythra),
            ("master_axeflare3", Fighter::Byleth),
            ("inkling_slosher_trace", Fighter::Inkling),
            ("inkling_roller_splash_run", Fighter::Inkling),
            ("ganon_majinken_flash", Fighter::Ganondorf),
            ("ganon_sword_trace", Fighter::Ganondorf),
            ("chrom_mc_1", Fighter::Chrom),
            ("palutena_backlight2", Fighter::Palutena),
            ("edge_fire3_screen2", Fighter::Sephiroth),
            ("edge_shadowflare_bullet", Fighter::Sephiroth),
            ("captain_fp_flash", Fighter::CaptainFalcon),
            ("roy_erupution_hold", Fighter::Roy),
            ("P_IkeVolcanoMax", Fighter::Ike),
            ("ike_firepiller02", Fighter::Ike),
        ] {
            assert_eq!(unscoped_fighter(hash40(name)), Some(fighter), "{name}");
        }
        // General feedback and other fighters' names pass through.
        for name in [
            "sys_hit_normal",
            "sys_flash",
            "sys_smash_flash",
            "flare1",
            "inkling_blaster_typo_unlisted",
            "mario_fireball",
            "null",
        ] {
            assert_eq!(unscoped_fighter(hash40(name)), None, "{name}");
            assert_eq!(fighter_for(hash40(name), None), None, "{name}");
        }
    }

    #[test]
    fn move_table_names_follow_the_move_owner() {
        let shared: BTreeSet<&str> = SHARED_NAMES.iter().copied().collect();
        let mut used = BTreeSet::new();
        for profile in PROFILES {
            let owner = of_profile(profile);
            let context = sample(profile);
            let mut names: Vec<&str> = effects_catalog::profile_names(profile).to_vec();
            for (p, keep, cull) in effects_reduced::PROFILE_NAME_TABLES {
                if *p == profile {
                    names.extend(keep.iter().chain(cull.iter()));
                }
            }
            for name in names {
                let by_name = name_fighter(name);
                if by_name != Some(owner) {
                    assert!(
                        shared.contains(name),
                        "{name} in {profile:?} is not {owner:?}'s and not listed as shared"
                    );
                    used.insert(name);
                }
                assert_eq!(
                    fighter_for(hash40(name), Some(context)),
                    Some(owner),
                    "{name} {profile:?}"
                );
            }
        }
        let unused: Vec<_> = shared.difference(&used).collect();
        assert!(unused.is_empty(), "stale shared names {unused:?}");
    }

    #[test]
    fn rule_kinds_and_weapons_resolve_to_the_profile_owner() {
        for (profile, context) in effect_context::rule_samples() {
            assert_eq!(of_fighter_kind(context.kind), Some(of_profile(profile)));
            // Same fighter while idle: no move matches, the kind still does.
            let idle = Context {
                motion: hash40("wait"),
                status: 0,
                ..context
            };
            assert_eq!(
                fighter_for(hash40("sys_hit_normal"), Some(idle)),
                Some(of_profile(profile))
            );
        }
        let weapon = |kind: i32, status: i32| Context {
            category: 1,
            kind,
            motion: 0,
            status,
        };
        for (kind, status, fighter) in [
            (0x58, 0, Fighter::CaptainFalcon),
            (0x61, 0, Fighter::Ness),
            (0x163, 0, Fighter::Lucas),
            (0x110, 0, Fighter::Palutena),
            (0x111, 0, Fighter::Palutena),
            (0x112, 0, Fighter::Palutena),
            (0x246, 4, Fighter::Sephiroth),
            (0x247, 0, Fighter::Sephiroth),
            (0x248, 0, Fighter::Sephiroth),
            (0x249, 0, Fighter::Sephiroth),
        ] {
            assert_eq!(
                fighter_for(hash40("flare1"), Some(weapon(kind, status))),
                Some(fighter),
                "{kind:#x}"
            );
        }
        // A weapon kind is never read as a fighter kind (0x5c is Kazuya only as
        // a fighter); a Gigaflare fly status has no profile and falls back to
        // the name.
        assert_eq!(fighter_for(hash40("flare1"), Some(weapon(0x5c, 0))), None);
        assert_eq!(
            fighter_for(hash40("edge_fire3_burst"), Some(weapon(0x246, 0))),
            Some(Fighter::Sephiroth)
        );
        // An untracked object category is ignored; the name decides.
        assert_eq!(
            fighter_for(
                hash40("ganon_sword_flare"),
                Some(Context {
                    category: 2,
                    kind: 0x36,
                    motion: 0,
                    status: 0
                })
            ),
            Some(Fighter::Ganondorf)
        );
        // Chrom's neutral-B uses Ike's pillar: Chrom's row decides it.
        assert_eq!(
            fighter_for(
                hash40("ike_firepiller02"),
                Some(sample(Profile::ChromNeutralB))
            ),
            Some(Fighter::Chrom)
        );
    }

    #[test]
    fn reduced_support_matches_the_tables() {
        let mut derived = [false; COUNT];
        for (profile, keep, cull) in effects_reduced::PROFILE_NAME_TABLES {
            if !keep.is_empty() || !cull.is_empty() {
                derived[of_profile(*profile).index()] = true;
            }
        }
        for names in effects_reduced::GLOBAL_NAME_TABLES {
            for name in names.iter() {
                derived[name_fighter(name).unwrap().index()] = true;
            }
        }
        // EFF siblings: Palutena up air, Shadow Flare, roller splash, Chrom side B.
        for name in [
            "palutena_backlight2",
            "edge_shadowflare_bullet",
            "inkling_roller_splash_walk",
            "chrom_mc_1",
        ] {
            assert!(crate::eff_variant::cleanup_alias(hash40(name)).is_some());
            derived[name_fighter(name).unwrap().index()] = true;
        }
        for fighter in ALL {
            assert_eq!(
                fighter.has_reduced(),
                derived[fighter.index()],
                "{fighter:?}"
            );
        }
    }

    #[test]
    fn ids_labels_and_order_are_stable() {
        for (index, fighter) in ALL.iter().enumerate() {
            assert_eq!(fighter.index(), index);
            assert!(fighter.label().len() <= 24);
            assert!(!fighter.id().is_empty() && fighter.id().len() <= 32);
            assert!(fighter
                .id()
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b == b'_'));
            assert_eq!(Fighter::from_id(fighter.id()), Some(*fighter));
        }
        assert_eq!(Fighter::from_id("all"), None);
        assert_eq!(Fighter::from_id("scope"), None);
        let map_sorted = MAP.windows(2).all(|pair| pair[0].0 <= pair[1].0);
        assert!(map_sorted);
    }
}
