# Stage background targets — review inventory

This is the complete list of what a stage row set to **Off** asks the game not
to draw, alongside reviewed groups now preserved for contrast. This is an
unreleased candidate: **sampled visibility is not full runtime acceptance**.
Nothing here claims a measured FPS benefit or complete background
removal; unlisted scenery, stage simulation, playable geometry, hazards and
animation stay native. Runtime architecture, hooks and trace events are in
[`STAGES.md`](STAGES.md). This file records targets and policy; runtime acceptance
is tracked separately from source/static review.

## Modes and matching

- Stage rows are **On/Off only**, default On. Reduced is a fighter-only mode;
  a stage `reduced` value is ignored. INI: `stages=on|off`, then
  `stage.<key>=on|off` (keys below).
- **Particles:** an exact lowercase Hash40 effect name, requested from one of
  the stage-effect callers on that name's exact normal-stage ID. No prefixes,
  wildcards, textures, `sys_*` or fighter effects.
- **Models:** the runtime policy key is `folder_hash`, the Hash40 of the group's
  full DirInfo path `stage/<arc_key>/normal/model/<group>` (no trailing slash),
  on the exact normal-stage ID. `group_hash` is the leaf name hash only: it is
  provenance, never matched alone (Battlefield and Small Battlefield share leaf
  hashes but have different folder paths). Where a literal folder name is not
  hash-verified, only the full-path Hash40 is given; no names are guessed.
- Model policy selects the **whole group**. The mesh and material
  labels listed are the group's membership from its MODL descriptor, for
  review only: meshes are not switched individually and materials are not
  edited. Which draw passes actually disappear remains a runtime-review question.

## Provenance

| Item | Value |
| --- | --- |
| Repository HEAD at generation | `0c0d7e62c9d07b16b71323356c6cf16f86c97ea9` (stage tables in uncommitted working tree) |
| `src/stages.rs` SHA256 at generation | `6160cf1b9f3f775db6f0d8587ad31bc7562e9dc1e97642851a54df29ca96360e` |
| Game data | `data.arc`, 17600869448 bytes, first-256-byte SHA256 `335f4d53d92a3448516946e6ba6bdf1fa15a9e7f0050cc8e7657c9c99ca44403` |
| Game version of hook evidence | 13.0.5 (see `STAGES.md`) |
| Particle names | `BACKGROUNDS` in `src/stages.rs`; EFFN header census by smash-arc 0.6.0, dir-listing + rust-zstd, offline |
| Model groups | `MODELS` in `src/stages.rs`; external `MODEL-TARGETS.json` (complete MODL entries parsed with ssbh_lib 0.19.0) |
| Folder identity | external `DIRINFO-IDENTITIES.json`: 51 entries joined to 33804 DirInfo rows, 0 missing, 0 name/parent mismatches |
| Stage list and IDs | Elite Legal Stages settings; skyline-smash `bed642e` `StageID` (see `STAGES.md`) |
| External catalog SHA256 | `MODEL-TARGETS.json` `c051e3ff721c56f4ef4e0998e35cda10efd33a4d9850df5239cf5da99341addf`; `DIRINFO-IDENTITIES.json` `0f1c1d2aed3b1029f5ffa7f656c8e53154bc9bf9d2dd323360aa7247a18af1dc`; `RESOURCE-CENSUS.json` `1842d99f3634db7a3a6d25d4a49184031e160f4401e8a7deea18857f90bf7e8f` |

External files live under `~/.local/share/ssbu-mod-backups/plugin-api-tangent-20261012/`
(`stage-background-1021/` and `stage-background-remaining-1021/`). Descriptor
SHA256 values identify stock files kept outside the repository; no asset bytes
or descriptor copies are committed.

The original generation hash above identifies the29-group catalog, not today's
policy. The eleven preserved entries M4/M5/M10/M11/M14/M15/M20/M21/M22/M23/M28 remain documented
with their original stable IDs for provenance; they are absent from `MODELS`.
Runtime samples and limitations are summarized in [`STAGES.md`](STAGES.md).

## Summary — current targets

| Stage | Key (`stage.<key>`) | ID | Particle names | Model groups |
| --- | --- | ---: | ---: | ---: |
| Battlefield | `battle_field` | 0 | 0 | 4 |
| Small Battlefield | `battle_field_s` | 347 | 0 | 4 |
| Final Destination | `end` | 3 | 5 | 0 |
| Yoshi's Story | `yoshi_cartboard` | 44 | 0 | 1 |
| Lylat Cruise | `fox_lylatcruise` | 89 | 5 | 0 |
| Pokemon Stadium 2 | `poke_stadium2` | 107 | 0 | 1 |
| Smashville | `animal_village` | 95 | 0 | 7 |
| Kalos Pokemon League | `poke_kalos` | 242 | 0 | 1 |
| Town and City | `animal_city` | 257 | 2 | 0 |
| Northern Cave | `ff_cave` | 351 | 5 | 0 |
| Hollow Bastion | `trail_castle` | 361 | 30 | 0 |
| **Total** | | | **47** | **18** |

## Particle targets

Hash40 is of the lowercase name; the stock EFFN labels are the same names in
uppercase.

### Final Destination — `stage.end`, ID 3, 5 names

| # | Name | Hash40 | Status | On/Off/On | Geometry/cues | Recreation | Trace | FPS |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `stg_end_particle` | `0x10a56df8df` | pending | | | | | |
| 2 | `stg_end_swarm` | `0x0d60c7b337` | pending | | | | | |
| 3 | `stg_end_elec` | `0x0c138a33f0` | pending | | | | | |
| 4 | `stg_end_ringaura1` | `0x117aecf04a` | pending | | | | | |
| 5 | `stg_end_ringaura2` | `0x11e3e5a1f0` | pending | | | | | |

### Lylat Cruise — `stage.fox_lylatcruise`, ID 89, 5 names

| # | Name | Hash40 | Status | On/Off/On | Geometry/cues | Recreation | Trace | FPS |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `stg_sf_btlship` | `0x0e3f59ce9e` | pending | | | | | |
| 2 | `stg_sf_space` | `0x0c201b461d` | pending | | | | | |
| 3 | `stg_sf_space2` | `0x0d79f3c992` | pending | | | | | |
| 4 | `stg_sf_space3` | `0x0d0ef4f904` | pending | | | | | |
| 5 | `stg_sf_asteroid` | `0x0f647114a9` | pending | | | | | |

### Town and City — `stage.animal_city`, ID 257, 2 names

| # | Name | Hash40 | Status | On/Off/On | Geometry/cues | Recreation | Trace | FPS |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `stg_village2_wind` | `0x11d8cf91de` | pending | | | | | |
| 2 | `stg_village2_shabon` | `0x136b1abd06` | pending | | | | | |

### Northern Cave — `stage.ff_cave`, ID 351, 5 names

| # | Name | Hash40 | Status | On/Off/On | Geometry/cues | Recreation | Trace | FPS |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `stg_cave_life_stream2` | `0x15b6ed0569` | pending | | | | | |
| 2 | `stg_cave_holy_wave` | `0x12039a96b8` | pending | | | | | |
| 3 | `stg_cave_meteor_wave` | `0x1433674d3a` | pending | | | | | |
| 4 | `stg_cave_holy_line` | `0x12088a8fc7` | pending | | | | | |
| 5 | `stg_cave_life_stream_tentacle` | `0x1d926c582b` | pending | | | | | |

### Hollow Bastion — `stage.trail_castle`, ID 361, 30 names

| # | Name | Hash40 | Status | On/Off/On | Geometry/cues | Recreation | Trace | FPS |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `stg_trail_castle_water_fall_bubble` | `0x22ab0ad125` | pending | | | | | |
| 2 | `stg_trail_castle_bubble` | `0x1718483e5e` | pending | | | | | |
| 3 | `stg_trail_castle_switch_red` | `0x1b2fd9aab3` | pending | | | | | |
| 4 | `stg_trail_castle_switch_blue` | `0x1cb18c0ec6` | pending | | | | | |
| 5 | `stg_trail_castle_switch_yellow` | `0x1e5104def8` | pending | | | | | |
| 6 | `stg_trail_castle_trolley` | `0x1876543816` | pending | | | | | |
| 7 | `stg_trail_castle_trolley_light` | `0x1eb1ef2ebd` | pending | | | | | |
| 8 | `stg_trail_castle_trolley_m` | `0x1a7b63359c` | pending | | | | | |
| 9 | `stg_trail_castle_trolley_s` | `0x1a816c08ff` | pending | | | | | |
| 10 | `stg_trail_castle_divetoheart` | `0x1cd0cd3971` | pending | | | | | |
| 11 | `stg_trail_castle_divetoheart_2` | `0x1edd0c1296` | pending | | | | | |
| 12 | `stg_trail_castle_divetoheart_3` | `0x1eaa0b2200` | pending | | | | | |
| 13 | `stg_trail_castle_divetoheart_4` | `0x1e346fb7a3` | pending | | | | | |
| 14 | `stg_trail_castle_divetoheart_5` | `0x1e43688735` | pending | | | | | |
| 15 | `stg_trail_castle_divetoheart_6` | `0x1eda61d68f` | pending | | | | | |
| 16 | `stg_trail_castle_divetoheart_7` | `0x1ead66e619` | pending | | | | | |
| 17 | `stg_trail_castle_stained_glass_line1` | `0x24b9385733` | pending | | | | | |
| 18 | `stg_trail_castle_stained_glass_line2` | `0x2420310689` | pending | | | | | |
| 19 | `stg_trail_castle_stained_glass_line3` | `0x245736361f` | pending | | | | | |
| 20 | `stg_trail_castle_stained_glass_line4` | `0x24c952a3bc` | pending | | | | | |
| 21 | `stg_trail_castle_stained_glass_line5` | `0x24be55932a` | pending | | | | | |
| 22 | `stg_trail_castle_stained_glass_line6` | `0x24275cc290` | pending | | | | | |
| 23 | `stg_trail_castle_stained_glass_line7` | `0x24505bf206` | pending | | | | | |
| 24 | `stg_trail_castle_stained_glass` | `0x1eccd05b67` | pending | | | | | |
| 25 | `stg_trail_castle_stained_glass_2` | `0x206228bb28` | pending | | | | | |
| 26 | `stg_trail_castle_stained_glass_3` | `0x20152f8bbe` | pending | | | | | |
| 27 | `stg_trail_castle_stained_glass_4` | `0x208b4b1e1d` | pending | | | | | |
| 28 | `stg_trail_castle_stained_glass_5` | `0x20fc4c2e8b` | pending | | | | | |
| 29 | `stg_trail_castle_stained_glass_6` | `0x2065457f31` | pending | | | | | |
| 30 | `stg_trail_castle_stained_glass_7` | `0x2012424fa7` | pending | | | | | |

## Reviewed model groups — 22 targets, 7 preserved

"Sampled subset" means the stage's selected groups were switched together: the
geometry sample was clear and each listed path produced events115/116. It does not
establish complete disappearance of each group in isolation or every draw pass.

| # | Stage | Key | Description | `folder_hash` (runtime) | Status | On/Off/On | Geometry/cues | Recreation | Trace | FPS |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| M1 | Battlefield | `stage.battle_field` | Distant birds | `0x0000002b4bb4272c` | sampled subset | isolation2-bf | clear sample | reload | 115/116 | unmeasured |
| M2 | Battlefield | `stage.battle_field` | Background waterfalls and mist | `0x0000002b1e0f086b` | sampled subset | isolation2-bf | clear sample | reload | 115/116 | unmeasured |
| M3 | Battlefield | `stage.battle_field` | Distant sky city | `0x000000315a16b74f` | sampled subset | isolation2-bf | clear sample | reload | 115/116 | unmeasured |
| M4 | Battlefield | `stage.battle_field` | Background landscape and waterfalls | `0x0000002c917d5282` | preserved | isolation1-bf | Off glare | | | |
| M5 | Battlefield | `stage.battle_field` | Distant sky dome | `0x0000002de76dc4dc` | preserved | baseline/restart1 | contrast backdrop | | | |
| M6 | Battlefield | `stage.battle_field` | Sun | `0x00000033750d0d24` | sampled subset | isolation2-bf | clear sample | reload | 115/116 | unmeasured |
| M7 | Small Battlefield | `stage.battle_field_s` | Distant birds | `0x0000002d7f2d0809` | sampled subset | isolation2-sbf | clear sample | reload | 115/116 | unmeasured |
| M8 | Small Battlefield | `stage.battle_field_s` | Background waterfalls and mist | `0x0000002d2a96274e` | sampled subset | isolation2-sbf | clear sample | reload | 115/116 | unmeasured |
| M9 | Small Battlefield | `stage.battle_field_s` | Distant sky city | `0x00000033adcc787d` | sampled subset | isolation2-sbf | clear sample | reload | 115/116 | unmeasured |
| M10 | Small Battlefield | `stage.battle_field_s` | Background landscape and waterfalls | `0x0000002eda4d1fea` | preserved | isolation1-sbf | Off glare | | | |
| M11 | Small Battlefield | `stage.battle_field_s` | Distant sky dome | `0x0000002fa44f1dfb` | preserved | baseline/restart1 | contrast backdrop | | | |
| M12 | Small Battlefield | `stage.battle_field_s` | Sun | `0x0000003593a671ae` | sampled subset | isolation2-sbf | clear sample | reload | 115/116 | unmeasured |
| M13 | Yoshi's Story | `stage.yoshi_cartboard` | Distant sea and islands | `0x0000002a64a18d77` | sampled | isolation1-yoshi | clear sample | reload | 115/116 | unmeasured |
| M14 | Yoshi's Story | `stage.yoshi_cartboard` | Sky and clouds | `0x0000002a2301f7a7` | preserved | restart1-yoshi | Off whiteout | | | |
| M15 | Pokemon Stadium 2 | `stage.poke_stadium2` | Night sky and distant city | `0x00000031d3665d85` | preserved | restart1/isolation1-ps2 | contrast backdrop | | | |
| M16 | Pokemon Stadium 2 | `stage.poke_stadium2` | City searchlights | `0x00000033452ece7d` | sampled | isolation1-ps2 | clear sample | reload | 115/116 | unmeasured |
| M17 | Smashville | `stage.animal_village` | Village background | `0x0000002c57952cc2` | pending | | | | | |
| M18 | Smashville | `stage.animal_village` | Background clock hour hand | `0x00000037e5838f27` | pending | | | | | |
| M19 | Smashville | `stage.animal_village` | Background clock minute hand | `0x00000036df2d6df4` | pending | | | | | |
| M20 | Smashville | `stage.animal_village` | Sky phase 0 | `0x0000002edb810e01` | preserved | phase pending | native sky fill | | | |
| M21 | Smashville | `stage.animal_village` | Sky phase 1 | `0x0000002e9c2174d1` | preserved | Surface sky3974 | whiteout corrected in sampled phase | | | |
| M22 | Smashville | `stage.animal_village` | Sky phase 2 | `0x0000002ea1415d61` | preserved | phase pending | native sky fill | | | |
| M23 | Smashville | `stage.animal_village` | Night sky and stars | `0x0000002fd06ff93b` | preserved | phase pending | native sky fill | | | |
| M24 | Smashville | `stage.animal_village` | Background window lights 0 | `0x0000003a1e32cfaf` | pending | | | | | |
| M25 | Smashville | `stage.animal_village` | Background window lights 1 | `0x0000003a2352e61f` | pending | | | | | |
| M26 | Smashville | `stage.animal_village` | Background window lights 2 | `0x0000003a64f29ccf` | pending | | | | | |
| M27 | Smashville | `stage.animal_village` | Background window lights 3 | `0x0000003a5992b57f` | pending | | | | | |
| M28 | Kalos Pokemon League | `stage.poke_kalos` | Great Hall background | `0x000000345b4e2c16` | preserved | restart1/isolation1-kalos | contrast backdrop | | | |
| M29 | Kalos Pokemon League | `stage.poke_kalos` | Great Hall decorative lights | `0x0000003e1ac8733d` | sampled | isolation1-kalos | clear Great Hall sample | reload | 115/116 | unmeasured |

### Model group details

#### M1 Battlefield — Distant birds

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x0000002b4bb4272c` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0cefa68d5f`
- Descriptor: `0x000000381b5a18c9`, SHA256 `2efd4c7723c7383c4ba8b50ec24016086d409defca6617f8a7f90574758b7a9f`
- DirInfo identity: verified
- Meshes (28): `__still_Shape1`, `__still_Shape2`, `__tokiAShape`, `__tokiBShape`, `__tokiCShape`, `__tokiDShape`, `__tokiShape28`, `__toki_a0Shape1`, `__toki_a0Shape3`, `__toki_a0Shape4`, `__toki_a0Shape5`, `__toki_a0Shape6`, `__toki_aShape14`, `__toki_aShape15`, `__toki_aShape16`, `__toki_aShape17`, `__toki_c0Shape6`, `__toki_cShape14`, `__toki_cShape15`, `__toki_cShape21`, `__toki_cShape22`, `__toki_cShape24`, `__w_toki_b0Shape2`, `__w_toki_b0Shape3`, `__w_toki_b0Shape4`, `__w_toki_b0Shape7`, `__w_toki_b0Shape8`, `__ww_toki_a0Shape6`
- Materials (5): `__ShaderfxShader87_bird_shiftAnim`, `__ShaderfxShader93`, `__ShaderfxShader94`, `__ShaderfxShader95`, `__ShaderfxShader_birds_stay`

#### M2 Battlefield — Background waterfalls and mist

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x0000002b1e0f086b` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0cba1da218`
- Descriptor: `0x0000003892967f78`, SHA256 `86ee88a08bc384d366654728cb00c61a49b174f06b98b39454ab1b3c2797468b`
- DirInfo identity: verified
- Meshes (7): `__BG_fallsShape`, `__fall_LShape`, `__fall_RShape`, `__moya_CShape`, `__moya_farShape`, `__moya_mid_CShape`, `__moya_mid_RShape`
- Materials (3): `__ShaderfxShader101_hamon`, `__ShaderfxShader_fall_04ab`, `__ShaderfxShader_moya__01`

#### M3 Battlefield — Distant sky city

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x000000315a16b74f` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x12c95625bf`
- Descriptor: `0x0000003ea5c1c51a`, SHA256 `e67d2f9c8fcc2d6f5614deda0efff5b0e604eedef11b987e1092bebe8090cbf3`
- DirInfo identity: verified
- Meshes (4): `__skycity_01Shape`, `__skycity_02Shape`, `__skycity_0Shape3`, `__skycity_0Shape4`
- Materials (1): `__ShaderfxShader_A_Punch_All`

#### M4 Battlefield — Background landscape and waterfalls

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x0000002c917d5282` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0d58d560ae`
- Descriptor: `0x00000039dc96810a`, SHA256 `cba77e4ee327cb9294a7f57b53abfd5c1b698e75f9cb9b55e1321004f1f61f5a`
- DirInfo identity: verified
- Meshes (10): `__BG_groundShape`, `__LKShape0`, `__LKShape1`, `__LKShape2`, `__LShape`, `__R1Shape`, `__RKShape`, `__RKShape0`, `__RKShape2`, `__RShape2`
- Materials (4): `M3_ShaderfxShader_water04`, `__M2_ShaderfxShader_fall_05`, `__ShaderfxShader_A_Punch_All`, `__ShaderfxShader_fall_03top_ab`

#### M5 Battlefield — Distant sky dome

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x0000002de76dc4dc` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0ed57c000d`
- Descriptor: `0x0000003af9b9ac23`, SHA256 `3d9da8cfc6ace26ed277f35dbf019ef80e39f12aefeefeed55d42c66280bacea`
- DirInfo identity: verified
- Meshes (1): `__bg_far_skydome_2texShape`
- Materials (1): `__Z_bg_Sky_2uv_alp_haderfxShader150`

#### M6 Battlefield — Sun

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x00000033750d0d24` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x147fb08db1`
- Descriptor: `0x00000040e1a30ab0`, SHA256 `f72cc18f9a9a87a6cc5de345c7a245004a0120d0080d4acb9cd9dd4be953b5e3`
- DirInfo identity: verified
- Meshes (1): `__sun_Shape`
- Materials (1): `__ShaderfxShader15616000`

#### M7 Small Battlefield — Distant birds

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x0000002d7f2d0809` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0cefa68d5f`
- Descriptor: `0x0000003a1cec4935`, SHA256 `2efd4c7723c7383c4ba8b50ec24016086d409defca6617f8a7f90574758b7a9f`
- DirInfo identity: verified
- Meshes (28): `__still_Shape1`, `__still_Shape2`, `__tokiAShape`, `__tokiBShape`, `__tokiCShape`, `__tokiDShape`, `__tokiShape28`, `__toki_a0Shape1`, `__toki_a0Shape3`, `__toki_a0Shape4`, `__toki_a0Shape5`, `__toki_a0Shape6`, `__toki_aShape14`, `__toki_aShape15`, `__toki_aShape16`, `__toki_aShape17`, `__toki_c0Shape6`, `__toki_cShape14`, `__toki_cShape15`, `__toki_cShape21`, `__toki_cShape22`, `__toki_cShape24`, `__w_toki_b0Shape2`, `__w_toki_b0Shape3`, `__w_toki_b0Shape4`, `__w_toki_b0Shape7`, `__w_toki_b0Shape8`, `__ww_toki_a0Shape6`
- Materials (5): `__ShaderfxShader87_bird_shiftAnim`, `__ShaderfxShader93`, `__ShaderfxShader94`, `__ShaderfxShader95`, `__ShaderfxShader_birds_stay`

#### M8 Small Battlefield — Background waterfalls and mist

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x0000002d2a96274e` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0cba1da218`
- Descriptor: `0x0000003a95202e84`, SHA256 `86ee88a08bc384d366654728cb00c61a49b174f06b98b39454ab1b3c2797468b`
- DirInfo identity: verified
- Meshes (7): `__BG_fallsShape`, `__fall_LShape`, `__fall_RShape`, `__moya_CShape`, `__moya_farShape`, `__moya_mid_CShape`, `__moya_mid_RShape`
- Materials (3): `__ShaderfxShader101_hamon`, `__ShaderfxShader_fall_04ab`, `__ShaderfxShader_moya__01`

#### M9 Small Battlefield — Distant sky city

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x00000033adcc787d` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x12c95625bf`
- Descriptor: `0x00000040ffb4fdc0`, SHA256 `e67d2f9c8fcc2d6f5614deda0efff5b0e604eedef11b987e1092bebe8090cbf3`
- DirInfo identity: verified
- Meshes (4): `__skycity_01Shape`, `__skycity_02Shape`, `__skycity_0Shape3`, `__skycity_0Shape4`
- Materials (1): `__ShaderfxShader_A_Punch_All`

#### M10 Small Battlefield — Background landscape and waterfalls

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x0000002eda4d1fea` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0d58d560ae`
- Descriptor: `0x0000003b689a896c`, SHA256 `cba77e4ee327cb9294a7f57b53abfd5c1b698e75f9cb9b55e1321004f1f61f5a`
- DirInfo identity: verified
- Meshes (10): `__BG_groundShape`, `__LKShape0`, `__LKShape1`, `__LKShape2`, `__LShape`, `__R1Shape`, `__RKShape`, `__RKShape0`, `__RKShape2`, `__RShape2`
- Materials (4): `M3_ShaderfxShader_water04`, `__M2_ShaderfxShader_fall_05`, `__ShaderfxShader_A_Punch_All`, `__ShaderfxShader_fall_03top_ab`

#### M11 Small Battlefield — Distant sky dome

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x0000002fa44f1dfb` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0ed57c000d`
- Descriptor: `0x0000003c5ddc6446`, SHA256 `3d9da8cfc6ace26ed277f35dbf019ef80e39f12aefeefeed55d42c66280bacea`
- DirInfo identity: verified
- Meshes (1): `__bg_far_skydome_2texShape`
- Materials (1): `__Z_bg_Sky_2uv_alp_haderfxShader150`

#### M12 Small Battlefield — Sun

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x0000003593a671ae` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x147fb08db1`
- Descriptor: `0x00000042b276c5ce`, SHA256 `f72cc18f9a9a87a6cc5de345c7a245004a0120d0080d4acb9cd9dd4be953b5e3`
- DirInfo identity: verified
- Meshes (1): `__sun_Shape`
- Materials (1): `__ShaderfxShader15616000`

#### M13 Yoshi's Story — Distant sea and islands

- Stage `yoshi_cartboard` (ID 44, ARC key `yoshi_cartboard`)
- `folder_hash`: `0x0000002a64a18d77` = `stage/yoshi_cartboard/normal/model/bg1_set`
- `group_hash` (leaf, provenance only): `0x0782cecf5c`
- Descriptor: `0x00000037062ffcb6`, SHA256 `848b4bd7cb6790184468116acde7a1a523c6fa295aa95592d665822bb050f268`
- DirInfo identity: verified
- Meshes (9): `TSeaAShape`, `TSeaBShape`, `USeaAShape`, `USeaBShape`, `VIslandShape`, `WSeaAShape`, `XIslandShape`, `XSeaAShape`, `XSeaBShape`
- Materials (19): `sfx_FabTreeC__S_CUS_0xa2001001____NORMEXP16___VTC__AT_GREATER128_`, `sfx_FabTree__S_CUS_0xa2001001___A_AB__NORMEXP16___VTC_`, `sfx_cartboard_bake_lit06__S_CUS_0xa2001001___A_AB__NORMEXP16___VTC_`, `sfx_cartboard_bake_lit06__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong53__S_CUS_0xa2001001___A_AB__NORMEXP16___VTC_`, `sfx_phong85__S_CUS_0xa2001001___A_AB__NORMEXP16___VTC_`, `sfx_phong86__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_sfx_yoshi_cartboard_bake_lit05__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC___S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_sfx_yoshi_cartboard_bake_lit06__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC___S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_1`, `sfx_yoshi_cartboard_bake_lit05__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_yoshi_cartboard_bake_lit05__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_1`, `sfx_yoshi_cartboard_bake_lit05__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_3`, `sfx_yoshi_cartboard_bake_lit05__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_4`, `sfx_yoshi_cartboard_bake_lit05__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC__AT_GREATER128_`, `sfx_yoshi_cartboard_bake_lit05__S_CUS_0xa2001001____NORMEXP16___VTC__AT_GREATER128_`, `sfx_yoshi_cartboard_bake_lit06__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_yoshi_cartboard_bake_lit06__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_2`, `sfx_yoshi_cartboard_bake_lit06__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_4`, `sfx_yoshi_cartboard_bake_lit06__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC__AT_GREATER128_`

#### M14 Yoshi's Story — Background clouds

- Stage `yoshi_cartboard` (ID 44, ARC key `yoshi_cartboard`)
- `folder_hash`: `0x0000002a2301f7a7` = `stage/yoshi_cartboard/normal/model/bg2_set`
- `group_hash` (leaf, provenance only): `0x07c56eb58c`
- Descriptor: `0x00000037eca921d4`, SHA256 `488759b41f603ef99cd03a778374ed79c5b230d41920d1cc26a1e23bf6e84aaa`
- DirInfo identity: verified
- Meshes (23): `BG2_mgShape`, `YCloudAShape`, `YCloudBShape`, `YCloudCShape`, `YCloudDShape`, `YCloudDShape1`, `YCloudDShape2`, `YCloudEShape`, `YCloudFShape`, `YCloudGShape`, `YCloudGShape1`, `YCloudHShape`, `YCloudHShape1`, `YCloudHShape2`, `YCloudHShape3`, `YCloudIShape`, `YCloudIShape1`, `YCloudJShape`, `YCloudKShape`, `YCloudLShape`, `YCloudMShape`, `YCloudNShape`, `YCloudNShape1`
- Materials (9): `sfx_phong0__S_CUS_0xa2001001___NORMEXP16___VTC_`, `sfx_phong46__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong47__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong49__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong51__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong56__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong57__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong58__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong62__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`

#### M15 Pokemon Stadium 2 — Night sky and distant city

- Stage `poke_stadium2` (ID 107, ARC key `poke_stadium2`)
- `folder_hash`: `0x00000031d3665d85` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x109f595b18`
- Descriptor: `0x0000003ec4e98c31`, SHA256 `eda09b57f3b525f0683ede40c7bb15449b5a4a76c640a296fb5b6de4d999e7d3`
- DirInfo identity: verified
- Meshes (1): `nightskyShape`
- Materials (8): `sfx_01_Abuilding01`, `sfx_01_Bbuilding02`, `sfx_02_building01`, `sfx_03_building03`, `sfx_tansp_builfrea`, `sfx_tansp_builfrea1`, `zzz_sfx_01_sky01`, `zzz_sfx_04_ground`

#### M16 Pokemon Stadium 2 — City searchlights

- Stage `poke_stadium2` (ID 107, ARC key `poke_stadium2`)
- `folder_hash`: `0x00000033452ece7d` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x1268a2d320`
- Descriptor: `0x000000403f0b5627`, SHA256 `030eeb87cd342d5ae3839707cb6285892469c3df71bc0fbaf4feabd0ed610162`
- DirInfo identity: verified
- Meshes (11): `builfrea_fShape`, `groupShape253`, `searchlightA01_O_OBJ_O_HIRShape`, `searchlightA02_O_OBJ_O_HIRShape`, `searchlightA03_O_OBJ_O_HIRShape`, `searchlightA04_O_OBJ_O_HIRShape`, `searchlightA05_O_OBJ_O_HIRShape`, `searchlightA06_O_OBJ_O_HIRShape`, `searchlightA07_O_OBJ_O_HIRShape`, `searchlightA09_O_OBJ_O_HIRShape`, `searchlightA10_O_OBJ_O_HIRShape`
- Materials (3): `sfx_phong100_S_CUS_0xa2001081___A_ADD_____NORMEXP16___VTC_`, `sfx_tansp_builfrea`, `sfx_tansp_builfrea1`

#### M17 Smashville — Village background

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000002c57952cc2` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0a3c571c04`
- Descriptor: `0x0000003944377e4f`, SHA256 `545806d880ce6a1fc8642d3eb913984e75553d79642b6ad09f4733ffbc3f14a7`
- DirInfo identity: verified
- Meshes (1): `BGShape`
- Materials (36): `sfx_04_ground01`, `sfx_05_cliff`, `sfx_06_cliffedge`, `sfx_07_groundsoil01`, `sfx_08_rivercliff`, `sfx_09_river_bl`, `sfx_10_riveredge_bl`, `sfx_11_riverfall05`, `sfx_12_riverfall04_bl`, `sfx_13_riverfall02_bl`, `sfx_14_riverfall03_bl`, `sfx_15_ground02`, `sfx_15_groundstone01`, `sfx_16_bg_shadow_bl`, `sfx_17_house_shadow_bl`, `sfx_17_nature`, `sfx_18_house02`, `sfx_19_house09`, `sfx_19_house_roof`, `sfx_20_house04`, `sfx_21_house06`, `sfx_22_house07`, `sfx_23_house03`, `sfx_24_house11`, `sfx_25_house08`, `sfx_26_house10`, `sfx_27_object`, `sfx_27_signboard`, `sfx_28_house05`, `sfx_29_house01`, `sfx_30_bridge`, `sfx_31_sea02`, `sfx_32_sea01_bl`, `sfx_33_sea03_00_bl`, `sfx_33_sea03_01_bl`, `sfx_34_riverfall01_bl`

#### M18 Smashville — Background clock hour hand

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x00000037e5838f27` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x15dfdc7c0f`
- Descriptor: `0x0000004434a90171`, SHA256 `347260da274b190aa3fc4ebafe3d060fd3c19b1adff336f63700d434f871b5cb`
- DirInfo identity: verified
- Meshes (1): `BG_Clock_hourShape`
- Materials (1): `sfx_35_clock`

#### M19 Smashville — Background clock minute hand

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x00000036df2d6df4` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x14358f8b95`
- Descriptor: `0x0000004358f76111`, SHA256 `835a82c26dbaeb7616a57ca8ecc58b698719d869b0d1d91600c39cf482f1b8c4`
- DirInfo identity: verified
- Meshes (1): `BG_Clock_minShape`
- Materials (1): `sfx_35_clock`

#### M20 Smashville — Sky phase 0

- **Preserved**, not an Off target: native sky fill; phase-specific acceptance pending.
- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000002edb810e01` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0c60f825a7`
- Descriptor: `0x0000003bf206bd9c`, SHA256 `1993344ae9ae11b9aed926cede2d6ae89daf158b6670ccea58e12859ff55e75c`
- DirInfo identity: verified
- Meshes (1): `SkyShape00`
- Materials (2): `sfx_00_sky`, `sfx_01_cloud_bl`

#### M21 Smashville — Sky phase 1

- **Preserved**, not an Off target: retaining this active sky corrected Surface whiteout with village suppression unchanged.
- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000002e9c2174d1` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0c27585f77`
- Descriptor: `0x0000003b188060fe`, SHA256 `38c014756ded83a01947412fe91b02ec7d90826a5eaee4f89a59e6c87a2f84d2`
- DirInfo identity: verified
- Meshes (1): `SkyShape01`
- Materials (2): `sfx_00_sky`, `sfx_01_cloud_bl`

#### M22 Smashville — Sky phase 2

- **Preserved**, not an Off target: native sky fill; phase-specific acceptance pending.
- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000002ea1415d61` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0c1a3876c7`
- Descriptor: `0x0000003bf7d2d61f`, SHA256 `737b4d30a3a51ceee7679c7ac59272318357d3444ec873786e6e3129cc9b106c`
- DirInfo identity: verified
- Meshes (1): `SkyShape02`
- Materials (2): `sfx_00_sky`, `sfx_01_cloud_bl`

#### M23 Smashville — Night sky and stars

- **Preserved**, not an Off target: native sky fill; phase-specific acceptance pending.
- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000002fd06ff93b` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0def6186cd`
- Descriptor: `0x0000003c72964258`, SHA256 `7d692ec71c212ee20bee482e76fe167b20d06651569bf82061074ef0bf27f32f`
- DirInfo identity: verified
- Meshes (3): `StarShape`, `cloud3Shape`, `sky3Shape`
- Materials (3): `sfx_00_sky`, `sfx_01_cloud_bl`, `sfx_02_skyster_bl`

#### M24 Smashville — Background window lights 0

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000003a1e32cfaf` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x18ec2926d6`
- Descriptor: `0x00000047880a96d6`, SHA256 `527cca6c7da40807fd68943e58db2bc003e61af6e3364d274f50d6bc72a68c57`
- DirInfo identity: verified
- Meshes (1): `LIGHTShape00`
- Materials (1): `sfx_17_window_light`

#### M25 Smashville — Background window lights 1

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000003a2352e61f` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x18d1490f66`
- Descriptor: `0x0000004767582037`, SHA256 `f266997ff412834f3cb75bb29e4f8cf2011202bd24d3896836fac75e251eb636`
- DirInfo identity: verified
- Meshes (1): `LIGHTShape01`
- Materials (1): `sfx_17_window_light`

#### M26 Smashville — Background window lights 2

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000003a64f29ccf` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x1896e975b6`
- Descriptor: `0x000000478ddefd55`, SHA256 `253b1769ecad537ecedf830710feb95844f4cf9bfb404ed5cdf6e651585087a2`
- DirInfo identity: verified
- Meshes (1): `LIGHTShape02`
- Materials (1): `sfx_17_window_light`

#### M27 Smashville — Background window lights 3

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000003a5992b57f` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x18ab895c06`
- Descriptor: `0x00000047628c4bb4`, SHA256 `f0ef541972f9a53c8c84124d7a66afda19b1914a3e205441817cfbd8de1618ab`
- DirInfo identity: verified
- Meshes (1): `LIGHTShape03`
- Materials (2): `sfx_16_light03_bl`, `sfx_17_window_light`

#### M28 Kalos Pokemon League — Great Hall background

- Stage `poke_kalos` (ID 242, ARC key `poke_kalos`)
- `folder_hash`: `0x000000345b4e2c16` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x164d69b112`
- Descriptor: `0x000000411b2c89ce`, SHA256 `eec3e05d35d8efadf95aae4b91c06a14b7efd662daa0373424cb3d94d38360aa`
- DirInfo identity: verified
- Meshes (1): `B00_BigHall_BGShape`
- Materials (27): `ShaderfxShader11`, `ShaderfxShader17`, `ShaderfxShader_floor`, `ShaderfxShader_floor_futi`, `ShaderfxShader_kabe_crcle`, `ShaderfxShader_metal_vertexlightmap`, `ShaderfxShader_oku_kabe`, `sfx_00_plr0201_kabe01`, `sfx_01_R_ceiling`, `sfx_02_bighall_sqpillar`, `sfx_03_plr0201_chu01`, `sfx_04_plr0201_komono_metal`, `sfx_05_window_glass_AWAI`, `sfx_06_window_glass2`, `sfx_07_hallway_door`, `sfx_08_fox`, `sfx_09_nun`, `sfx_13_bighall_ornament`, `sfx_14_hall_smdoor`, `sfx_15_b3d_efs_light`, `sfx_16_bighall_element_metal`, `sfx_17_bighall_element_doragon`, `sfx_19_flag`, `sfx_20_carpet`, `sfx_21_Stage_Ornament_Noalpha`, `sfx_22_stairwall`, `sfx_23_hall_stair`

#### M29 Kalos Pokemon League — Great Hall decorative lights

- Stage `poke_kalos` (ID 242, ARC key `poke_kalos`)
- `folder_hash`: `0x0000003e1ac8733d` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x20df44b495`
- Descriptor: `0x0000004bef582ba7`, SHA256 `df7fc004084c0d617ea6ea9953617bd8d98fdb50739a86b1f270a5fa820aa8c8`
- DirInfo identity: verified
- Meshes (2): `B00_BigHall_blend_bufferShape`, `fire_meshgroupShape`
- Materials (5): `A_sfx_11_plr021_light02_bl`, `fire_taimatu`, `sfx_05_window_glass_AWAI`, `sfx_27_plr021_warmlight_bl`, `sfx_28_plr0201_light`

## Preserved (not targeted)

These stay native on every setting. They are examples checked against the
catalog, not the full set of untargeted groups.

### Preserved model examples (22)

Platforms, main stages, floating floors, ink meshes, monitors, bridges and
transition groups. Their folder hashes are absent from `MODELS`.

| # | Stage | Key | `folder_hash` | First meshes |
| ---: | --- | --- | --- | --- |
| P1 | Battlefield | `battle_field` | `0x000000312c350aa1` | `After_floatingfloor_renderUVShape1`, `After_floatingfloor_renderUVShape2`, `After_floatingfloor_renderUVShape3` … |
| P2 | Battlefield | `battle_field` | `0x000000343fce69c3` | `After_cyrcle_ring_base_block_bakeShape1`, `After_under_bakeShape`, `crystal_emiShape` … |
| P3 | Battlefield | `battle_field` | `0x0000003369cf1008` | `floating_plateShape`, `main_ringShape` |
| P4 | Small Battlefield | `battle_field_s` | `0x00000033dbefc593` | `After_floatingfloor_renderUVShape1`, `After_floatingfloor_renderUVShape2`, `inkMesh4Shape` … |
| P5 | Small Battlefield | `battle_field_s` | `0x000000363245a881` | `After_cyrcle_ring_base_block_bakeShape1`, `After_under_bakeShape`, `crystal_emiShape` … |
| P6 | Small Battlefield | `battle_field_s` | `0x000000358f646c82` | `floating_plateShape`, `main_ringShape` |
| P7 | Yoshi's Story | `yoshi_cartboard` | `0x00000034713c2b39` | `FloatingFloorShape`, `inkMesh_floatingfloor1Shape`, `inkMesh_floatingfloor2Shape` … |
| P8 | Yoshi's Story | `yoshi_cartboard` | `0x0000002d46527583` | `BGrassAShape`, `BGrassBShape`, `BTreeAAShape` … |
| P9 | Yoshi's Story | `yoshi_cartboard` | `0x000000306a3b4e23` | `CRift1Shape` |
| P10 | Yoshi's Story | `yoshi_cartboard` | `0x00000037124d526c` | `Yoshi_CartBoardShape` |
| P11 | Pokemon Stadium 2 | `poke_stadium2` | `0x00000033bbb23ea7` | `groupShape78`, `inkMesh_StageShape`, `inkMesh_st_AshibaLShape` … |
| P12 | Pokemon Stadium 2 | `poke_stadium2` | `0x0000003364621744` | `StageMainShape`, `groupShape17` |
| P13 | Pokemon Stadium 2 | `poke_stadium2` | `0x0000002d487925d2` | `Air_DactFShape`, `Air_DactShape`, `BaseShape` … |
| P14 | Pokemon Stadium 2 | `poke_stadium2` | `0x00000035a8750554` | `AuroraVision11Shape`, `AuroraVision2Shape`, `MoniterMain2Shape` … |
| P15 | Pokemon Stadium 2 | `poke_stadium2` | `0x000000350f61f2d1` | `Poke_StadiumShape2` |
| P16 | Smashville | `animal_village` | `0x0000003553584044` | `Substage_O_OBJ_O_HIRShape`, `inkMesh2Shape` |
| P17 | Smashville | `animal_village` | `0x000000331f7da877` | `MainStageShape`, `inkMesh1Shape`, `shadowcast_meshShape` |
| P18 | Smashville | `animal_village` | `0x0000003689d076fc` | `Animal_VillageShape` |
| P19 | Kalos Pokemon League | `poke_kalos` | `0x0000003cc3754d2b` | `B00_BattleRing_BigHallShape`, `battlering_side_bighallShape`, `inkMesh1Shape` |
| P20 | Kalos Pokemon League | `poke_kalos` | `0x00000043c1051b1a` | `B00_YAKUMONO_bighall_surinukeShape`, `inkMesh2Shape`, `inkMesh3Shape` |
| P21 | Kalos Pokemon League | `poke_kalos` | `0x000000384374f592` | `W02_joint_bridge_Shape0`, `inkMesh10Shape` |
| P22 | Kalos Pokemon League | `poke_kalos` | `0x0000003235e708b8` | `Poke_KalosShape` |

#### P1 Battlefield — preserved

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x000000312c350aa1` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x12bf759851`
- Descriptor: `0x0000003effd672e0`, SHA256 `7102e5b8723a444a71c00174c9685675c09d4d27c09d909089207b2f8c1c28f3`
- DirInfo identity: verified
- Meshes (6): `After_floatingfloor_renderUVShape1`, `After_floatingfloor_renderUVShape2`, `After_floatingfloor_renderUVShape3`, `inkMesh3Shape`, `inkMesh4Shape`, `inkMesh5Shape`
- Materials (6): `fx_battlefield_linegainB`, `sfx_Lightmap_INK`, `sfx_battlefield_linegainA1`, `sfx_battlefield_relief01_baked_F`, `sfx_floatingfloor_baked_F_gold`, `sfx_floatingfloor_retouch_baked_F`

#### P2 Battlefield — preserved

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x000000343fce69c3` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x15bfa1b388`
- Descriptor: `0x00000041a8ce79ce`, SHA256 `1c4fc9db2c84187093a70a44648716ab9a598de94afba012705ea80e014dbd45`
- DirInfo identity: verified
- Meshes (7): `After_cyrcle_ring_base_block_bakeShape1`, `After_under_bakeShape`, `crystal_emiShape`, `goldShape`, `inkMesh1Shape`, `lineShape`, `test_ring_chains_meshgroupShape`
- Materials (17): `ShaderfxShader1`, `ShaderfxShader127`, `ShaderfxShader136`, `ShaderfxShader147`, `ShaderfxShader148`, `ShaderfxShader150`, `ShaderfxShader151`, `ShaderfxShader152`, `ShaderfxShader153`, `ShaderfxShader161`, `ShaderfxShader31_ring_yellow`, `sfx_Gold_lightmap`, `sfx_Lightmap_INK`, `sfx_battlefield_linegainA`, `sfx_battlefield_linegainB`, `sfx_center_blue`, `sfx_relief02_baked_E`

#### P3 Battlefield — preserved

- Stage `battle_field` (ID 0, ARC key `battlefield`)
- `folder_hash`: `0x0000003369cf1008` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x146372909d`
- Descriptor: `0x000000409e87e07b`, SHA256 `67d42cc827812c359a28a36ed029c40dd36567a741ebc03d478092a629a4d452`
- DirInfo identity: verified
- Meshes (2): `floating_plateShape`, `main_ringShape`
- Materials (1): `shader_transition01`

#### P4 Small Battlefield — preserved

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x00000033dbefc593` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x12bf759851`
- Descriptor: `0x00000040a5a34a3a`, SHA256 `3ba694587c17894ce8d818e33a7f7c0c228fe1f451b1ed12d994484785ba2de0`
- DirInfo identity: verified
- Meshes (4): `After_floatingfloor_renderUVShape1`, `After_floatingfloor_renderUVShape2`, `inkMesh4Shape`, `inkMesh5Shape`
- Materials (6): `fx_battlefield_linegainB`, `sfx_Lightmap_INK`, `sfx_battlefield_linegainA1`, `sfx_battlefield_relief01_baked_F`, `sfx_floatingfloor_baked_F_gold`, `sfx_floatingfloor_retouch_baked_F`

#### P5 Small Battlefield — preserved

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x000000363245a881` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x15bfa1b388`
- Descriptor: `0x000000431f20f03a`, SHA256 `1c4fc9db2c84187093a70a44648716ab9a598de94afba012705ea80e014dbd45`
- DirInfo identity: verified
- Meshes (7): `After_cyrcle_ring_base_block_bakeShape1`, `After_under_bakeShape`, `crystal_emiShape`, `goldShape`, `inkMesh1Shape`, `lineShape`, `test_ring_chains_meshgroupShape`
- Materials (17): `ShaderfxShader1`, `ShaderfxShader127`, `ShaderfxShader136`, `ShaderfxShader147`, `ShaderfxShader148`, `ShaderfxShader150`, `ShaderfxShader151`, `ShaderfxShader152`, `ShaderfxShader153`, `ShaderfxShader161`, `ShaderfxShader31_ring_yellow`, `sfx_Gold_lightmap`, `sfx_Lightmap_INK`, `sfx_battlefield_linegainA`, `sfx_battlefield_linegainB`, `sfx_center_blue`, `sfx_relief02_baked_E`

#### P6 Small Battlefield — preserved

- Stage `battle_field_s` (ID 347, ARC key `battlefield_s`)
- `folder_hash`: `0x000000358f646c82` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x146372909d`
- Descriptor: `0x00000042cd522f05`, SHA256 `67d42cc827812c359a28a36ed029c40dd36567a741ebc03d478092a629a4d452`
- DirInfo identity: verified
- Meshes (2): `floating_plateShape`, `main_ringShape`
- Materials (1): `shader_transition01`

#### P7 Yoshi's Story — preserved

- Stage `yoshi_cartboard` (ID 44, ARC key `yoshi_cartboard`)
- `folder_hash`: `0x00000034713c2b39` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x115f5ed9a5`
- Descriptor: `0x00000041058b86aa`, SHA256 `e47c170a9abfab5ff514a182dbc5f9c23e15ee0440a09952df97b343181478c3`
- DirInfo identity: verified
- Meshes (5): `FloatingFloorShape`, `inkMesh_floatingfloor1Shape`, `inkMesh_floatingfloor2Shape`, `inkMesh_floatingfloor3Shape`, `inkMesh_ringShape`
- Materials (3): `ShaderfxShader52`, `sfx_phong1__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong1__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_1`

#### P8 Yoshi's Story — preserved

- Stage `yoshi_cartboard` (ID 44, ARC key `yoshi_cartboard`)
- `folder_hash`: `0x0000002d46527583` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0a6d088d2a`
- Descriptor: `0x0000003ac50085d9`, SHA256 `c85d626b087924211971115799a637427759db72ad605aa8bc0851217768c095`
- DirInfo identity: verified
- Meshes (17): `BGrassAShape`, `BGrassBShape`, `BTreeAAShape`, `BTreeABShape`, `BTreeBAShape`, `BTreeBBShape`, `BTreeCAShape`, `BTreeCBShape`, `BTreeDAShape`, `BTreeDBShape`, `CGroundAShape1`, `CGroundAShape2`, `CGroundAShape3`, `CGroundAShape4`, `CGroundAShape5`, `CGroundCShape1`, `GroundShape`
- Materials (20): `sfx_FabTreeA__S_CUS_0xa2001001____NORMEXP16___VTC__AT_GREATER200_`, `sfx_cardboard04__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_1`, `sfx_cardboard05__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_cartboard_bake_lit02__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_2`, `sfx_cartboard_bake_lit02__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_3`, `sfx_phong46__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_1`, `sfx_phong46__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_2`, `sfx_phong57__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_1`, `sfx_phong57__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_2`, `sfx_phong64__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong67__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong68__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong69__S_CUS_0xa2001001___A_AB__NORMEXP16___VTC_`, `sfx_phong71__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong72__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong74__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong75__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_phong76__S_CUS_0xa2001001___A_AB__NORMEXP16___VTC_`, `sfx_yoshi_cartboard_bake_lit04__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_`, `sfx_yoshi_cartboard_bake_lit04__S_CUS_0xa2001001___A_OFF__NORMEXP16___VTC_1`

#### P9 Yoshi's Story — preserved

- Stage `yoshi_cartboard` (ID 44, ARC key `yoshi_cartboard`)
- `folder_hash`: `0x000000306a3b4e23` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0deb49fbb8`
- Descriptor: `0x0000003dcd5732d2`, SHA256 `c2864a051595c9cc94f785997552df0280394e9a132293f3886721b5988b40ff`
- DirInfo identity: verified
- Meshes (1): `CRift1Shape`
- Materials (1): `sfx_phong59__S_CUS_0xa2001001___A_AB__NORMEXP16___VTC_`

#### P10 Yoshi's Story — preserved

- Stage `yoshi_cartboard` (ID 44, ARC key `yoshi_cartboard`)
- `folder_hash`: `0x00000037124d526c` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x146372909d`
- Descriptor: `0x00000044adb18bed`, SHA256 `718a66568df52ff05ba1f02026ad9e545773bad7b8cc89523a96dfa62a541996`
- DirInfo identity: verified
- Meshes (1): `Yoshi_CartBoardShape`
- Materials (1): `shader_transition01`

#### P11 Pokemon Stadium 2 — preserved

- Stage `poke_stadium2` (ID 107, ARC key `poke_stadium2`)
- `folder_hash`: `0x00000033bbb23ea7` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x12963e23fa`
- Descriptor: `0x0000004035b113ca`, SHA256 `cd7e883cf1530e16c0d0add1b47284fa0be9b60325d6e88edb395f754ba9fd97`
- DirInfo identity: verified
- Meshes (4): `groupShape78`, `inkMesh_StageShape`, `inkMesh_st_AshibaLShape`, `inkMesh_st_AshibaRShape`
- Materials (8): `ShaderfxShader123`, `sfx_PokemonBall2_D1M_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_light_S_CUS_0xe2041001__A_ADD____VTC_1`, `sfx_lineB1_D1M_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_metal_parts02_S_CUS_0xa2011001___A_OFF_____NORMEXP16___VTC_`, `sfx_phong3_S_CUS_0xe2041001__A_ADD____VTC_1`, `sfx_phong93_S_CUS_0xa2011001___A_OFF_____NORMEXP16___VTC_2`, `sfx_phong96_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`

#### P12 Pokemon Stadium 2 — preserved

- Stage `poke_stadium2` (ID 107, ARC key `poke_stadium2`)
- `folder_hash`: `0x0000003364621744` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x1249ee0a19`
- Descriptor: `0x00000040023f5d5b`, SHA256 `a4fa7acce97e23ef7a971500b34952fb894b2e4b90971ead79c2c82526eafa34`
- DirInfo identity: verified
- Meshes (2): `StageMainShape`, `groupShape17`
- Materials (12): `sfx_PokemonBall2_D1M_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_light_S_CUS_0xe2041001__A_ADD____VTC_1`, `sfx_lineB1_D1M_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_lineB1_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_lineB2_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_metal_parts02_S_CUS_0xa2011001___A_OFF_____NORMEXP16___VTC_`, `sfx_phong3_S_CUS_0xe2041001__A_ADD____VTC_1`, `sfx_phong70_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_phong91_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_phong93_S_CUS_0xa2011001___A_OFF_____NORMEXP16___VTC_2`, `sfx_phong96_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_phong97_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`

#### P13 Pokemon Stadium 2 — preserved

- Stage `poke_stadium2` (ID 107, ARC key `poke_stadium2`)
- `folder_hash`: `0x0000002d487925d2` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x0c924d3c58`
- Descriptor: `0x0000003a96130901`, SHA256 `bf11ec628d90efdae6f8290a5256cd6a9c378c8fdb7f2c62c99e9e2c026dd263`
- DirInfo identity: verified
- Meshes (16): `Air_DactFShape`, `Air_DactShape`, `BaseShape`, `StageMain_sideShape`, `Stage_partsShape`, `Stage_partsShape05`, `Stage_partsShape10`, `fenceBShape`, `groupShape394`, `groupShape395`, `groupShape396`, `groupShape397`, `metal_partsBShape`, `metal_partsBaShape`, `metal_partsShape`, `ring_lightShape`
- Materials (14): `sfx_05_alp_light04`, `sfx_hikou_00_metal_parts03`, `sfx_light_S_CUS_0xe2041001__A_ADD____VTC_`, `sfx_light_S_CUS_0xe2041001__A_ADD____VTC_1`, `sfx_metal_parts01_S_CUS_0xa2011001___A_OFF____NORMEXP16___VTC_`, `sfx_metal_parts01_S_CUS_0xa2011001___A_OFF_____NORMEXP16___VTC_`, `sfx_metal_parts01_S_CUS_0xa2011001___A_OFF_____NORMEXP16___VTC_3`, `sfx_metal_parts02_S_CUS_0xa2011001_____NORMEXP16___VTC_`, `sfx_metal_parts07_S_CUS_0xa2011001___A_OFF_____NORMEXP16___VTC_`, `sfx_phong38_S_CUS_0xa2011001___A_OFF_____NORMEXP16___VTC_`, `sfx_phong39_S_CUS_0xa2011001_____NORMEXP16___VTC_`, `sfx_phong73_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`, `sfx_phong92_S_CUS_0xa2011001___A_AB_____NORMEXP16___VTC_`, `sfx_phong96_S_CUS_0xa2011041___A_OFF_____NORMEXP16___VTC_`

#### P14 Pokemon Stadium 2 — preserved

- Stage `poke_stadium2` (ID 107, ARC key `poke_stadium2`)
- `folder_hash`: `0x00000035a8750554` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x14c4666718`
- Descriptor: `0x00000042b355b785`, SHA256 `8b2a576869cf2bd01cf90c999a632eabb04afb5b3819308b21670d9ab394fc86`
- DirInfo identity: verified
- Meshes (6): `AuroraVision11Shape`, `AuroraVision2Shape`, `MoniterMain2Shape`, `MoniterMain3Shape`, `MoniterMainShape`, `polySurfaceShape1433`
- Materials (5): `SFX_vision`, `sfx_vision_02_base00`, `sfx_vision_03_back`, `sfx_vision_04_main01`, `sfx_vision_05_main00`

#### P15 Pokemon Stadium 2 — preserved

- Stage `poke_stadium2` (ID 107, ARC key `poke_stadium2`)
- `folder_hash`: `0x000000350f61f2d1` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x146372909d`
- Descriptor: `0x000000426bae80fb`, SHA256 `3ca1a0a34f8d9ebec09667134fa0c37106172e74bada6d3652d119bb231ede58`
- DirInfo identity: verified
- Meshes (1): `Poke_StadiumShape2`
- Materials (1): `shader_transition01`

#### P16 Smashville — preserved

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000003553584044` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x1324f918c0`
- Descriptor: `0x000000420a366401`, SHA256 `c93941ca34fca114ea84305ebaff4f7fe7b51c941b25d17ede5e236af39630fa`
- DirInfo identity: verified
- Meshes (2): `Substage_O_OBJ_O_HIRShape`, `inkMesh2Shape`
- Materials (9): `sfx_41_stage_woodpattern01`, `sfx_41_stagewood01`, `sfx_42_stage01`, `sfx_43_stage_spotlightmesh`, `sfx_45_stage_woodpattern02`, `sfx_48_stage_woodpattern04`, `sfx_50_stagelight`, `sfx_54_carpet`, `z_ink`

#### P17 Smashville — preserved

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x000000331f7da877` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x11cb38ced2`
- Descriptor: `0x000000408d51544d`, SHA256 `17e1c42ffe27c77d9b5e1f80af90dc4ea39b90f8461f7d8c81cc586702cd2094`
- DirInfo identity: verified
- Meshes (3): `MainStageShape`, `inkMesh1Shape`, `shadowcast_meshShape`
- Materials (17): `sfx_41_stage_woodpattern01`, `sfx_41_stage_woodpattern02`, `sfx_41_stagewood01`, `sfx_41_stagewood02`, `sfx_41_stagewood03`, `sfx_42_stage01`, `sfx_43_stage_seat`, `sfx_43_stage_spotlightmesh`, `sfx_45_stage_woodpattern02`, `sfx_46_stage_wood03`, `sfx_47_stamp_bl`, `sfx_48_stage_woodpattern04`, `sfx_50_stagelight`, `sfx_53_shadow01_bl`, `sfx_53_shadow03_bl`, `sfx_54_carpet`, `z_ink`

#### P18 Smashville — preserved

- Stage `animal_village` (ID 95, ARC key `animal_village`)
- `folder_hash`: `0x0000003689d076fc` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x146372909d`
- Descriptor: `0x00000043892327ea`, SHA256 `6036ba179e855a04bfb538831470d24544be86e3002e68cb89bcb37c96dbe6fc`
- DirInfo identity: verified
- Meshes (1): `Animal_VillageShape`
- Materials (1): `shader_transition01`

#### P19 Kalos Pokemon League — preserved

- Stage `poke_kalos` (ID 242, ARC key `poke_kalos`)
- `folder_hash`: `0x0000003cc3754d2b` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x1e3c290381`
- Descriptor: `0x0000004931b920bd`, SHA256 `ca4da9d75bd6a41df1809c7bb8af4368c6b2d3ec62e5fae139b9d8c2cac6277a`
- DirInfo identity: verified
- Meshes (3): `B00_BattleRing_BigHallShape`, `battlering_side_bighallShape`, `inkMesh1Shape`
- Materials (14): `ShaderfxShader18`, `ShaderfxShader19_ink`, `battlering_side_ShaderfxShader3_green`, `battlering_side_ShaderfxShader4`, `battlering_side_ShaderfxShader5`, `battlering_side_ShaderfxShader7`, `battlering_side_ShaderfxShader_green`, `battlering_side_sfx_01_c_06_Green_Light`, `battlering_side_sfx_monyou`, `sfx_29_Stage_FrameTEST_hosoi`, `sfx_30_Stage_Ornament_BIG`, `sfx_31_Stage_Ornament_BIG`, `sfx_32_Magic_Light`, `sfx_34_Glass4_bl`

#### P20 Kalos Pokemon League — preserved

- Stage `poke_kalos` (ID 242, ARC key `poke_kalos`)
- `folder_hash`: `0x00000043c1051b1a` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x25346ad81d`
- Descriptor: `0x00000050c2c76935`, SHA256 `5a0a48bb70178928a8ef7e15befc2977b4843a0b599b9224080a9d2bcafebcdf`
- DirInfo identity: verified
- Meshes (3): `B00_YAKUMONO_bighall_surinukeShape`, `inkMesh2Shape`, `inkMesh3Shape`
- Materials (6): `ShaderfxShader19_ink`, `sfx_29_Stage_FrameTEST_hosoi`, `sfx_31_Stage_Ornament_BIG`, `sfx_34_Glass4_bl`, `sfx_38_Floating_Stage`, `sfx_39_Green_Light`

#### P21 Kalos Pokemon League — preserved

- Stage `poke_kalos` (ID 242, ARC key `poke_kalos`)
- `folder_hash`: `0x000000384374f592` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x1a17cc4e52`
- Descriptor: `0x0000004544561bb6`, SHA256 `4a386b1f2bf9136fa23448c7d36ca1d094a2a1595963c195fe1038877a9bcb2d`
- DirInfo identity: verified
- Meshes (2): `W02_joint_bridge_Shape0`, `inkMesh10Shape`
- Materials (5): `ShaderfxShader_ink`, `sfx_46_Brick_metal`, `sfx_47_plr0201_komono`, `sfx_48_Stage_Ornament_BIG`, `sfx_49_Yakumono_Water`

#### P22 Kalos Pokemon League — preserved

- Stage `poke_kalos` (ID 242, ARC key `poke_kalos`)
- `folder_hash`: `0x0000003235e708b8` (literal folder name unresolved)
- `group_hash` (leaf, provenance only): `0x146372909d`
- Descriptor: `0x0000003facf7bb15`, SHA256 `618b917e873d59f629ae5b0d765ad90f825ea20214c4d6522aa4efee719db60d`
- DirInfo identity: verified
- Meshes (1): `Poke_KalosShape`
- Materials (2): `shader_transition01`, `shader_transition01_blend`

### Preserved particle cues

Stock EFFN labels in the censused stage packages that are not in
`BACKGROUNDS`, so they are never suppressed. Labels are as recorded in the
header census (uppercase). Fighter, item and shared `sys_*` effects are also
never matched.

| Package | Preserved labels | Review note |
| --- | --- | --- |
| `yoshi_cartboard` | `STG_CARTBOARD_SMOKE` | Stage smoke cue |
| `poke_stadium2` | `STG_STADIUM_KEMURI` | Transformation smoke cue |
| `poke_kalos` | `STG_KALOS_HINOKO`, `STG_KALOS_FIREPILLAR_L`, `STG_KALOS_SPLASH`, `STG_KALOS_SHIBUKI`, `STG_KALOS_SMOKE`, `STG_KALOS_DUST`, `STG_KALOS_DUST2`, `STG_KALOS_TYAKUDAN`, `STG_KALOS_POKEMON`, `STG_KALOS_POKEMON_OUT`, `STG_KALOS_THUNDER`, `STG_KALOS_HAMON`, `STG_KALOS_IMPACT`, `STG_KALOS_WAVE`, `STG_KALOS_HOLD` | Hazard, transformation and Pokemon cues |

Pokemon Stadium 2's monitors are preserved as model group P14 above.

## Review checklist

Fill the per-target columns above. Use one short note or a capture/log path
per cell; leave Status `pending` until every column for that row has evidence.

- **On/Off/On:** target visible On, gone Off, back On, each after a stage reload.
- **Geometry/cues:** platforms, walls, ledges, hazards, transformations,
  timers/telegraphs, fighters, items and hit effects unchanged; no preserved
  cue above disappears.
- **Recreation:** behaviour across model recreation and stage state changes
  (Smashville time of day, Stadium/Kalos transformations, rematch, KO/results).
- **Trace:** `TRACE_EFFECTS` reachability and suppression events as listed in
  `STAGES.md`, decoded with `tools/resolve_trace.py`.
- **FPS:** tracing removed, equal-duration stock-clock captures On vs Off, same
  stage, fighters, actions, render mode and camera; record frame-time
  distribution and worst one-second window. No benefit is assumed.
- Negative controls: other stages and Battlefield/Omega forms with the row Off
  show no change.
