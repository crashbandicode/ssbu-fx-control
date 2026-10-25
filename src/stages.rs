//! Elite Legal Stages' eleven normal stages. This is a separate background-FX
//! policy: fighter All/Reduced never changes these settings.
//!
//! List provenance and the visual-request boundary are recorded in
//! `docs/STAGES.md`. Numeric IDs use skyline-smash bed642e's StageID table;
//! Battlefield/Omega variants of other stages are intentionally not aliases.
use crate::{config::FxMode, effects::hash40};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Stage {
    pub key: &'static str,
    pub label: &'static str,
    pub game_id: i32,
}

pub const COUNT: usize = 11;
pub const ALL: [Stage; COUNT] = [
    Stage {
        key: "battle_field",
        label: "Battlefield",
        game_id: 0,
    },
    Stage {
        key: "battle_field_s",
        label: "Small Battlefield",
        game_id: 347,
    },
    Stage {
        key: "end",
        label: "Final Destination",
        game_id: 3,
    },
    Stage {
        key: "yoshi_cartboard",
        label: "Yoshi's Story",
        game_id: 44,
    },
    Stage {
        key: "fox_lylatcruise",
        label: "Lylat Cruise",
        game_id: 89,
    },
    Stage {
        key: "poke_stadium2",
        label: "Pokemon Stadium 2",
        game_id: 107,
    },
    Stage {
        key: "animal_village",
        label: "Smashville",
        game_id: 95,
    },
    Stage {
        key: "poke_kalos",
        label: "Kalos Pokemon League",
        game_id: 242,
    },
    Stage {
        key: "animal_city",
        label: "Town and City",
        game_id: 257,
    },
    Stage {
        key: "ff_cave",
        label: "Northern Cave",
        game_id: 351,
    },
    Stage {
        key: "trail_castle",
        label: "Hollow Bastion",
        game_id: 361,
    },
];

pub fn by_key(key: &str) -> Option<usize> {
    ALL.iter().position(|stage| stage.key == key)
}

pub fn by_game_id(id: i32) -> Option<usize> {
    ALL.iter().position(|stage| stage.game_id == id)
}

/// Exact stock EFFN header handles, normalized to the lowercase Hash40 used
/// by the stage request records. No prefix matching or emitter/texture names.
/// Classification is static; per-stage runtime/FPS acceptance is still pending.
#[derive(Clone, Copy)]
pub struct Background {
    pub stage: usize,
    pub name: &'static str,
    pub hash: u64,
}

macro_rules! backgrounds {
    ($($stage:literal => [$($name:literal),+ $(,)?]),+ $(,)?) => {
        pub const BACKGROUNDS: &[Background] = &[
            $($(Background { stage: $stage, name: $name, hash: hash40($name) },)+)+
        ];
    };
}

backgrounds! {
    2 => ["stg_end_particle", "stg_end_swarm", "stg_end_elec",
          "stg_end_ringaura1", "stg_end_ringaura2"],
    4 => ["stg_sf_btlship", "stg_sf_space", "stg_sf_space2",
          "stg_sf_space3", "stg_sf_asteroid"],
    8 => ["stg_village2_wind", "stg_village2_shabon"],
    9 => ["stg_cave_life_stream2", "stg_cave_holy_wave", "stg_cave_meteor_wave",
          "stg_cave_holy_line", "stg_cave_life_stream_tentacle"],
    10 => [
        "stg_trail_castle_water_fall_bubble", "stg_trail_castle_bubble",
        "stg_trail_castle_switch_red", "stg_trail_castle_switch_blue",
        "stg_trail_castle_switch_yellow", "stg_trail_castle_trolley",
        "stg_trail_castle_trolley_light", "stg_trail_castle_trolley_m",
        "stg_trail_castle_trolley_s", "stg_trail_castle_divetoheart",
        "stg_trail_castle_divetoheart_2", "stg_trail_castle_divetoheart_3",
        "stg_trail_castle_divetoheart_4", "stg_trail_castle_divetoheart_5",
        "stg_trail_castle_divetoheart_6", "stg_trail_castle_divetoheart_7",
        "stg_trail_castle_stained_glass_line1", "stg_trail_castle_stained_glass_line2",
        "stg_trail_castle_stained_glass_line3", "stg_trail_castle_stained_glass_line4",
        "stg_trail_castle_stained_glass_line5", "stg_trail_castle_stained_glass_line6",
        "stg_trail_castle_stained_glass_line7", "stg_trail_castle_stained_glass",
        "stg_trail_castle_stained_glass_2", "stg_trail_castle_stained_glass_3",
        "stg_trail_castle_stained_glass_4", "stg_trail_castle_stained_glass_5",
        "stg_trail_castle_stained_glass_6", "stg_trail_castle_stained_glass_7",
    ],
}

pub const PARTICLE_MASK: u16 = {
    let mut mask = 0;
    let mut i = 0;
    while i < BACKGROUNDS.len() {
        mask |= 1 << BACKGROUNDS[i].stage;
        i += 1;
    }
    mask
};

/// Exact `stage/<arc_key>/normal/model/<group>` folder `path.hash40()` values
/// from stock MODL evidence (`MODEL-TARGETS.json` / DirInfo join). Selected
/// scenery groups only; preserved platform folders are not listed here.
#[derive(Clone, Copy, Debug)]
pub struct BackgroundModel {
    pub stage: usize,
    pub path: u64,
    pub description: &'static str,
}

macro_rules! models {
    ($(($stage:literal, $path:literal, $description:literal)),+ $(,)?) => {
        pub const MODELS: &[BackgroundModel] = &[
            $(BackgroundModel { stage: $stage, path: $path, description: $description },)+
        ];
    };
}

models! {
    // Keep the landscape and sky fill: hiding either is not a contrast-safe
    // substitute for suppressing the separate decorative groups.
    (0, 0x0000002b4bb4272c, "Distant birds"),
    (0, 0x0000002b1e0f086b, "Background waterfalls and mist"),
    (0, 0x000000315a16b74f, "Distant sky city"),
    (0, 0x00000033750d0d24, "Sun"),
    (1, 0x0000002d7f2d0809, "Distant birds"),
    (1, 0x0000002d2a96274e, "Background waterfalls and mist"),
    (1, 0x00000033adcc787d, "Distant sky city"),
    (1, 0x0000003593a671ae, "Sun"),
    (3, 0x0000002a64a18d77, "Distant sea and islands"),
    (5, 0x00000033452ece7d, "City searchlights"),
    (6, 0x0000002c57952cc2, "Village background"),
    (6, 0x00000037e5838f27, "Background clock hour hand"),
    (6, 0x00000036df2d6df4, "Background clock minute hand"),
    // Keep Smashville's native sky fill across its time-of-day phases.
    // Suppressing the active sky produces a white/blooming backdrop.
    (6, 0x0000003a1e32cfaf, "Background window lights 0"),
    (6, 0x0000003a2352e61f, "Background window lights 1"),
    (6, 0x0000003a64f29ccf, "Background window lights 2"),
    (6, 0x0000003a5992b57f, "Background window lights 3"),
    (7, 0x0000003e1ac8733d, "Great Hall decorative lights"),
}

pub const MODEL_MASK: u16 = {
    let mut mask = 0;
    let mut i = 0;
    while i < MODELS.len() {
        mask |= 1 << MODELS[i].stage;
        i += 1;
    }
    mask
};

pub const SUPPORTED_MASK: u16 = PARTICLE_MASK | MODEL_MASK;

/// Bounded exact stage/folder-path lookup; no pointers or resource lookups retained.
pub fn background_model(stage_id: i32, path: u64) -> Option<&'static BackgroundModel> {
    MODELS
        .iter()
        .find(|model| ALL[model.stage].game_id == stage_id && model.path == path & 0xff_ffff_ffff)
}

/// Never make a game-hidden object visible. On leaves the native request alone.
pub fn model_visibility(stage_id: i32, path: u64, off: u16, requested: bool) -> bool {
    requested && background_model(stage_id, path).is_none_or(|model| off & (1 << model.stage) == 0)
}

pub const fn supported(index: usize) -> bool {
    index < COUNT && SUPPORTED_MASK & (1 << index) != 0
}

const SORTED: [Background; BACKGROUNDS.len()] = {
    let mut entries = [BACKGROUNDS[0]; BACKGROUNDS.len()];
    let mut i = 0;
    while i < entries.len() {
        entries[i] = BACKGROUNDS[i];
        let mut j = i;
        while j > 0 && entries[j - 1].hash > entries[j].hash {
            let previous = entries[j - 1];
            entries[j - 1] = entries[j];
            entries[j] = previous;
            j -= 1;
        }
        i += 1;
    }
    entries
};

/// Bounded binary search; no allocation, string hashing or engine query here.
pub fn background_stage(hash: u64) -> Option<usize> {
    SORTED
        .binary_search_by_key(&(hash & 0xff_ffff_ffff), |entry| entry.hash)
        .ok()
        .map(|index| SORTED[index].stage)
}

/// Return addresses of the seven statically verified stage-effect request sites.
/// A shared-manager request from a fighter/item/other caller is left alone.
pub fn stage_caller(return_offset: usize) -> bool {
    matches!(
        return_offset,
        0x25efcdc | 0x25efe14 | 0x25effa4 | 0x25f0f1c | 0x2626a70 | 0x2627e90 | 0x2627f44
    )
}

pub fn suppress(stage_id: i32, hash: u64, off: u16, return_offset: usize) -> bool {
    stage_caller(return_offset)
        && background_stage(hash)
            .is_some_and(|index| ALL[index].game_id == stage_id && off & (1 << index) != 0)
}

/// Stage backgrounds offer On/Off, never an unmeasured Reduced mode.
pub fn accepted(mode: FxMode) -> Option<FxMode> {
    (mode != FxMode::Reduced).then_some(mode)
}

pub fn common_mode(modes: &[FxMode; COUNT]) -> Option<FxMode> {
    [FxMode::On, FxMode::Off].into_iter().find(|mode| {
        modes
            .iter()
            .enumerate()
            .all(|(index, value)| !supported(index) || value == mode)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn models_cover_the_other_six_without_aliasing_stages_or_forcing_visibility() {
        assert_eq!(PARTICLE_MASK & MODEL_MASK, 0);
        assert_eq!(SUPPORTED_MASK, (1 << COUNT) - 1);
        assert_eq!(MODELS.len(), 18);
        for (i, model) in MODELS.iter().enumerate() {
            assert!(model.stage < COUNT && model.path <= 0xff_ffff_ffff);
            assert!(!model.description.is_empty());
            assert!(!MODELS[..i]
                .iter()
                .any(|previous| { previous.stage == model.stage && previous.path == model.path }));
            let id = ALL[model.stage].game_id;
            assert!(model_visibility(id, model.path, 0, true));
            assert!(!model_visibility(id, model.path, 0, false));
            assert!(!model_visibility(id, model.path, 1 << model.stage, true));
            assert!(!model_visibility(id, model.path, 1 << model.stage, false));
            assert!(model_visibility(id, model.path, !(1 << model.stage), true));
            for wrong_id in [-1, 1, 2, 999] {
                assert!(model_visibility(wrong_id, model.path, u16::MAX, true));
            }
        }
        // Full folder paths differ between Battlefield and Small Battlefield.
        let bf_birds = 0x0000002b4bb4272c;
        let sbf_birds = 0x0000002d7f2d0809;
        assert!(!model_visibility(0, bf_birds, 1, true));
        assert!(!model_visibility(347, sbf_birds, 1 << 1, true));
        assert!(model_visibility(347, sbf_birds, 1 << 0, true));
        assert!(model_visibility(0, sbf_birds, 1, true));
    }

    #[test]
    fn model_policy_preserves_platforms_transitions_and_unknown_paths() {
        for (id, paths) in [
            (0, &[0x312c350aa1, 0x343fce69c3, 0x3369cf1008][..]),
            (347, &[0x33dbefc593, 0x363245a881, 0x358f646c82][..]),
            (
                44,
                &[0x34713c2b39, 0x2d46527583, 0x306a3b4e23, 0x37124d526c][..],
            ),
            (
                107,
                &[
                    0x33bbb23ea7,
                    0x3364621744,
                    0x2d487925d2,
                    0x35a8750554,
                    0x350f61f2d1,
                ][..],
            ),
            (95, &[0x3553584044, 0x331f7da877, 0x3689d076fc][..]),
            (
                242,
                &[0x3cc3754d2b, 0x43c1051b1a, 0x384374f592, 0x3235e708b8][..],
            ),
        ] {
            for &path in paths.iter().chain(&[0, 0xff_ffff_ffff]) {
                assert!(background_model(id, path).is_none());
                assert!(model_visibility(id, path, u16::MAX, true));
                assert!(!model_visibility(id, path, u16::MAX, false));
            }
        }
    }

    #[test]
    fn model_policy_matches_full_folder_path_not_duplicate_leaf_name() {
        // battlefield-resources/stage_model_paths.tsv: same name_hash, different parents.
        let bf_normal_birds = 0x0000002b4bb4272c;
        // Leaf hash alone and another normal stage's full path must not match.
        assert!(background_model(0, bf_normal_birds).is_some());
        assert!(background_model(0, 0x0cefa68d5f).is_none());
        assert!(background_model(0, 0x2d7f2d0809).is_none());
        assert!(!model_visibility(0, bf_normal_birds, 1, true));
        assert!(model_visibility(0, 0x2d7f2d0809, 1, true));
    }

    #[test]
    fn stage_off_keeps_contrast_backdrops_without_forcing_hidden_models_visible() {
        for (id, path) in [
            (0, 0x0000002c917d5282),
            (0, 0x0000002de76dc4dc),
            (347, 0x0000002eda4d1fea),
            (347, 0x0000002fa44f1dfb),
            (44, 0x0000002a2301f7a7),
            (107, 0x00000031d3665d85),
            (95, 0x0000002edb810e01),
            (95, 0x0000002e9c2174d1),
            (95, 0x0000002ea1415d61),
            (95, 0x0000002fd06ff93b),
            (242, 0x000000345b4e2c16),
        ] {
            assert!(background_model(id, path).is_none());
            assert!(model_visibility(id, path, u16::MAX, true));
            assert!(!model_visibility(id, path, u16::MAX, false));
        }
    }

    #[test]
    fn keys_and_ids_are_unique_and_forms_are_not_aliases() {
        for (index, stage) in ALL.iter().enumerate() {
            assert_eq!(by_key(stage.key), Some(index));
            assert_eq!(by_game_id(stage.game_id), Some(index));
            assert!(!ALL[..index]
                .iter()
                .any(|other| other.key == stage.key || other.game_id == stage.game_id));
        }
        for other in [-1, 108, 109, 283, 308, 310, 311, 354] {
            assert_eq!(by_game_id(other), None);
        }
        assert_eq!(by_key("Battlefield"), None);
        assert_eq!(accepted(FxMode::Reduced), None);
        assert_eq!(common_mode(&[FxMode::On; COUNT]), Some(FxMode::On));
        let mut modes = [FxMode::On; COUNT];
        modes[2] = FxMode::Off;
        assert_eq!(common_mode(&modes), None);
    }

    #[test]
    fn only_exact_backgrounds_on_their_stage_and_stage_callers_are_suppressed() {
        assert_eq!(BACKGROUNDS.len(), 47);
        assert_eq!(PARTICLE_MASK.count_ones(), 5);
        for (n, entry) in BACKGROUNDS.iter().enumerate() {
            assert_eq!(entry.hash, hash40(entry.name));
            assert!(!BACKGROUNDS[..n]
                .iter()
                .any(|other| other.hash == entry.hash));
            assert_eq!(background_stage(entry.hash), Some(entry.stage));
            for (index, stage) in ALL.iter().enumerate() {
                for caller in [
                    0x25efcdc, 0x25efe14, 0x25effa4, 0x25f0f1c, 0x2626a70, 0x2627e90, 0x2627f44,
                ] {
                    assert_eq!(
                        suppress(stage.game_id, entry.hash, SUPPORTED_MASK, caller),
                        index == entry.stage
                    );
                    assert!(!suppress(stage.game_id, entry.hash, 0, caller));
                    assert!(!suppress(
                        stage.game_id,
                        entry.hash,
                        SUPPORTED_MASK & !(1 << index),
                        caller
                    ));
                }
            }
            assert!(!suppress(-1, entry.hash, SUPPORTED_MASK, 0x25efcdc));
            for rejected in [
                0x15c353c, 0x31c2f04, 0x25f0f18, 0x25f0f20, 0x25f0bc0, 0x25f0dd4, 0x2626a6c,
                0x2626a74, 0x2627e8c, 0x2627e94, 0x2627f40, 0x2627f48,
            ] {
                assert!(!stage_caller(rejected));
                assert!(!suppress(
                    ALL[entry.stage].game_id,
                    entry.hash,
                    SUPPORTED_MASK,
                    rejected,
                ));
            }
        }
        for name in [
            "stg_cartboard_smoke",
            "stg_stadium_kemuri",
            "stg_kalos_firepillar_l",
            "stg_kalos_thunder",
            "stg_kalos_hold",
            "sys_hit_normal",
            "edge_fire3_burst",
            "stg_end_particle_extra",
            "P_EndParticle",
            "ef_cmn_cloud00",
            "STG_END_PARTICLE",
            "null",
        ] {
            assert_eq!(background_stage(hash40(name)), None, "{name}");
        }
        assert!(!supported(usize::MAX));
        assert_eq!(background_stage(0), None);
    }
}
