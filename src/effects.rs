//! Initial, explicit character-FX scope. No wildcard or global hit-effect deletion.
use crate::config::FxMode;
use crate::effect_context::{self, Context};

pub const fn hash40(name: &str) -> u64 {
    let bytes = name.as_bytes();
    let mut crc = 0xffff_ffffu32;
    let mut index = 0;
    while index < bytes.len() {
        crc ^= bytes[index] as u32;
        let mut bit = 0;
        while bit < 8 {
            crc = (crc >> 1) ^ if crc & 1 != 0 { 0xedb8_8320 } else { 0 };
            bit += 1;
        }
        index += 1;
    }
    ((bytes.len() as u64) << 32) | (!crc as u64)
}

/// Same named null resource used by the peer mod; not the numeric hash zero.
/// The original request still executes, preserving the engine's request path.
pub const NULL_EFFECT: u64 = hash40("null");

pub const TARGET_NAMES: &[&str] = &[
    // Explosive Flame: peer May-14 source, corroborated by vanilla Explode ACMD.
    "palutena_bomb",
    "palutena_bomb_appear",
    "palutena_bomb_finish",
    // Shadow Flare: vanilla fighter/flare1/flare2 ACMD and local ef_edge.eff.
    "edge_shadowflare_start",
    "edge_shadowflare_hold",
    "edge_shadowflare_aura",
    "edge_shadowflare_beam",
    "edge_shadowflare_beam_hit",
    "edge_shadowflare_bullet",
    "edge_shadowflare_bullet_attack",
    "edge_shadowflare_bullet_hit",
    "edge_shadowflare_bullet_end",
    "P_EdgeFire2Burst",
    "P_EdgeFire2Burst2",
    "P_EdgeFire2Smoke",
    "P_EdgeFire3Burst",
    "P_EdgeFire3Burst2",
    "P_EdgeFire3Smoke",
    "captain_fp_body_l",
    "captain_fp_body_r",
    "captain_fp_flash",
    "captain_fp_hit_l",
    "captain_fp_hit_r",
    "captain_fp_hold",
    "captain_fp_wing_l",
    "captain_fp_wing_r",
    "edge_fire2_burst",
    "edge_fire2_burst2",
    "edge_fire2_smoke",
    "edge_fire3_burst",
    "edge_fire3_burst2",
    "edge_fire3_smoke",
    "master_axe_aura",
    "master_axe_hold",
    "master_axe_hold2",
    "master_axe_hold_end",
    "master_axe_slash_edge",
    "master_axe_slash_particle",
    "master_axeflare1",
    "master_axeflare2",
    "master_axeflare3",
    "master_axeflare_sp1",
    "master_axeflare_sp1_end",
    "master_axeflare_sp2",
    "master_axeflare_sp2_end",
    "master_axeflare_sp3",
    "master_axeflare_sp3_end",
];

const TARGET_HASHES: &[u64] = &[
    hash40("palutena_bomb"),
    hash40("palutena_bomb_appear"),
    hash40("palutena_bomb_finish"),
    hash40("edge_shadowflare_start"),
    hash40("edge_shadowflare_hold"),
    hash40("edge_shadowflare_aura"),
    hash40("edge_shadowflare_beam"),
    hash40("edge_shadowflare_beam_hit"),
    hash40("edge_shadowflare_bullet"),
    hash40("edge_shadowflare_bullet_attack"),
    hash40("edge_shadowflare_bullet_hit"),
    hash40("edge_shadowflare_bullet_end"),
    hash40("P_EdgeFire2Burst"),
    hash40("P_EdgeFire2Burst2"),
    hash40("P_EdgeFire2Smoke"),
    hash40("P_EdgeFire3Burst"),
    hash40("P_EdgeFire3Burst2"),
    hash40("P_EdgeFire3Smoke"),
    hash40("captain_fp_body_l"),
    hash40("captain_fp_body_r"),
    hash40("captain_fp_flash"),
    hash40("captain_fp_hit_l"),
    hash40("captain_fp_hit_r"),
    hash40("captain_fp_hold"),
    hash40("captain_fp_wing_l"),
    hash40("captain_fp_wing_r"),
    hash40("edge_fire2_burst"),
    hash40("edge_fire2_burst2"),
    hash40("edge_fire2_smoke"),
    hash40("edge_fire3_burst"),
    hash40("edge_fire3_burst2"),
    hash40("edge_fire3_smoke"),
    hash40("master_axe_aura"),
    hash40("master_axe_hold"),
    hash40("master_axe_hold2"),
    hash40("master_axe_hold_end"),
    hash40("master_axe_slash_edge"),
    hash40("master_axe_slash_particle"),
    hash40("master_axeflare1"),
    hash40("master_axeflare2"),
    hash40("master_axeflare3"),
    hash40("master_axeflare_sp1"),
    hash40("master_axeflare_sp1_end"),
    hash40("master_axeflare_sp2"),
    hash40("master_axeflare_sp2_end"),
    hash40("master_axeflare_sp3"),
    hash40("master_axeflare_sp3_end"),
];

pub fn rewrite(mode: FxMode, hash: u64) -> u64 {
    if mode == FxMode::Off
        && (TARGET_HASHES.contains(&hash) || crate::effects_catalog::global(hash))
    {
        NULL_EFFECT
    } else {
        hash
    }
}

pub fn needs_context(hash: u64) -> bool {
    crate::effects_catalog::needs_context(hash)
}

pub fn rewrite_with_context(mode: FxMode, hash: u64, context: Option<Context>) -> u64 {
    let global = rewrite(mode, hash);
    if global != hash || mode != FxMode::Off {
        return global;
    }
    if context
        .and_then(effect_context::profile)
        .is_some_and(|p| crate::effects_catalog::scoped(p, hash))
    {
        NULL_EFFECT
    } else {
        hash
    }
}

/// Build sorted lookup tables at compile time; no runtime hashing/allocation.
pub(crate) const fn sorted_hashes<const N: usize>(names: &[&str]) -> [u64; N] {
    assert!(names.len() == N);
    let mut values = [0; N];
    let mut i = 0;
    while i < N {
        let hash = hash40(names[i]);
        let mut j = i;
        while j > 0 && values[j - 1] > hash {
            values[j] = values[j - 1];
            j -= 1;
        }
        values[j] = hash;
        i += 1;
    }
    values
}

pub fn suppress_screen(mode: FxMode, hash: u64) -> bool {
    mode == FxMode::Off && hash == hash40("edge_fire3_screen2")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reduced_rewrite_is_identity_until_the_policy_module() {
        for &hash in TARGET_HASHES
            .iter()
            .chain([0, u64::MAX, hash40("sys_hit_normal")].iter())
        {
            assert_eq!(rewrite(FxMode::On, hash), hash);
            // Off still nulls TARGET_HASHES here. Reduced does not.
            assert_eq!(rewrite(FxMode::Reduced, hash), hash);
            let intent = crate::effects_reduced::classify(
                FxMode::Reduced,
                true,
                hash,
                None,
                &crate::moves::default_selection(),
                false,
            );
            if crate::moves::move_of(hash, None).is_none() {
                assert_eq!(
                    crate::effects_reduced::request_hash(intent, hash),
                    hash,
                    "a move-qualified name stays original until that move matches"
                );
            }
        }
        assert!(!suppress_screen(FxMode::On, hash40("edge_fire3_screen2")));
        assert!(!suppress_screen(
            FxMode::Reduced,
            hash40("edge_fire3_screen2")
        ));
    }
    #[test]
    fn off_is_scoped_and_does_not_erase_general_feedback() {
        assert_eq!(rewrite(FxMode::Off, hash40("captain_fp_flash")), NULL_EFFECT);
        for name in [
            "sys_hit_normal",
            "sys_shield",
            "sys_damage_fire",
            "sys_score_aura",
            "bg_finishhit",
            // Retired 0.1.x fighters: their Off tables are gone.
            "ike_volcano",
            "chrom_volcano_a",
            "roy_eruption",
            "demon_smash_arc",
            "ganon_sword_flare",
        ] {
            assert_eq!(rewrite(FxMode::Off, hash40(name)), hash40(name));
            assert!(!suppress_screen(FxMode::Off, hash40(name)));
        }
        assert!(suppress_screen(FxMode::Off, hash40("edge_fire3_screen2")));
    }
    #[test]
    fn off_leaves_protected_up_air_parent_and_still_nulls_shadow_flare() {
        use crate::effect_context::Context;
        let up_air = Context {
            category: 0,
            kind: 0x36,
            motion: hash40("attack_air_hi"),
            status: 0x36,
        };
        let light = hash40("palutena_backlight2");
        assert!(!crate::effects_catalog::global(light));
        assert!(!needs_context(light));
        assert_eq!(rewrite(FxMode::Off, light), light);
        assert_eq!(
            rewrite_with_context(FxMode::Off, light, Some(up_air)),
            light
        );
        assert_eq!(
            rewrite(FxMode::Off, hash40("edge_shadowflare_aura")),
            NULL_EFFECT
        );
    }
    #[test]
    fn hash_format_matches_crc32_and_name_table() {
        // CRC-32/ISO-HDLC standard check string.
        assert_eq!(hash40("123456789"), 0x09cb_f43926);
        for name in TARGET_NAMES {
            assert!(TARGET_HASHES.contains(&hash40(name)));
        }
        assert_eq!(TARGET_NAMES.len(), TARGET_HASHES.len());
    }
}
