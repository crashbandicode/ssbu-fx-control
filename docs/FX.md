# Character effects

On forwards original requests. Off suppresses the named character effects
below for a fighter whose row is Off, when the scope allows it. Reduced keeps
selected move cues and removes surrounding decoration. Its goal is to approach
Off's frame-time stability; a visible change alone does not establish that
benefit ([performance method](FX_PERFORMANCE_METHOD.md)).

This policy is Online Deluxe's character-FX feature, moved into its own
plugin. The tables, the request classification (`effects_reduced::classify`,
`request_hash`, `eff_variant::forward`) and the hooks are Deluxe's, ported from
`intpa/ssbu-online-deluxe-rewrite` at `f3fe436`. What changed is the mode:
Deluxe had one global FX mode; this plugin has one per covered fighter.

## Who owns a request

Each effect request is decided with the mode of the fighter that owns it
(`src/fighters.rs`):

1. When the policy reads the requesting object's context (category, kind,
   motion, status), a matching **move profile** names the fighter. Weapons
   count: Falcon Punch bird `0x58`, PK Thunder `0x61`/`0x163`, Explosive
   Flame `0x110`/`0x111`, Autoreticle `0x112`, Gigaflare burst `0x246`
   (burst statuses), Scintilla `0x247`, Shadow Flare `0x248`/`0x249`.
2. A tracked **fighter** object (category 0) with no matching move is still
   that fighter, by kind.
3. Otherwise the **name** decides, through a map built at compile time from
   every name the policy acts on without a move: the Off tables
   (`effects::TARGET_NAMES`, `effects_catalog::LEGACY_NAMES`), the
   character-wide Reduced tables (Pyra, Mythra, Byleth, Inkling, Ganondorf),
   `chrom_mc_1`, `palutena_backlight2` and the Gigaflare screen overlay. Each
   name is classified by its character prefix; a name with no prefix, or two,
   fails the build.

| Prefix | Fighter |
| --- | --- |
| `demon_` | Kazuya |
| `captain_` | Captain Falcon |
| `ness_`, `P_Ness` | Ness |
| `lucas_`, `P_Lucas` | Lucas |
| `ike_`, `P_Ike` | Ike |
| `chrom_`, `P_Chrom` | Chrom |
| `roy_`, `P_Roy` | Roy |
| `palutena_`, `P_Palutena` | Palutena |
| `edge_`, `P_Edge` | Sephiroth |
| `eflame_`, `P_Eflame` | Pyra |
| `elight_`, `P_Elight` | Mythra |
| `master_` | Byleth |
| `inkling_` | Inkling |
| `ganon_` | Ganondorf |
| `kirby_` | Kirby |

A request no fighter owns passes through untouched, whatever the rows say.
Since 0.2.0 only the ten fighters with a ledger move have a page row and an INI
line (Captain Falcon, Ness, Lucas, Palutena, Sephiroth, Pyra, Mythra, Byleth,
Inkling, Kirby); Kazuya, Ike, Chrom, Roy and Ganondorf always stay On, and
their Off tables are compiled but unreachable.

Shared names inside a move table follow the move's owner, never their prefix:
`sys_flash` (Ness and Lucas up special), `sys_smash_flash` and
`sys_smash_flash_s` (Sephiroth smashes), Ike's pillar layers that Chrom's and
Roy's neutral special reuse (`ike_firepiller00/02`, `ike_fireground00`) and
Sephiroth's prefix-less emitter-style parents (`flare1`, `aura1`, `line1`, and
so on). Those names need a context; without one they pass through. The test
`fighters::tests::move_table_names_follow_the_move_owner` lists them and fails
when the tables change without the list.

With every row at one mode, each decision equals Deluxe's global-mode decision
(`policy::tests::the_same_inputs_match_the_single_mode_pipeline`).

## All

Moving **All** sets every row. Reduced on a row without a Reduced variant sets
that row On, never Off. While the rows differ, All has no value: the menu
shows `Unavailable` (the host's rendering of `CHOICE_UNAVAILABLE`, since a
fourth "Mixed" choice cannot be read-only on its own), and the INI has no
`all=` line.

## Coverage

The expanded catalog includes 314 distinct names from the peer's unconditional
legacy tables, three verified Inkling roller splash parents and 19 qualified
move groups. These overlap the initial 123-name policy. Aliases and
emitter-child names are source candidates, not proof that every name reaches a
request hook. The README has the per-move table.

The plugin hooks the ordinary EffectModule request wrappers plus ten ACMD
creation variants: EFFECT, EFFECT_FOLLOW, EFFECT_FOLLOW_ALPHA,
EFFECT_FOLLOW_NO_STOP, EFFECT_FLW_POS, EFFECT_FLW_POS_NO_STOP, LANDING_EFFECT,
EFFECT_FOLLOW_FLIP, EFFECT_FOLLOW_FLIP_ALPHA and EFFECT_FOLLOW_NO_STOP_FLIP.
Flip variants filter their first two effect names independently, each with its
own owner; joint, transform, scale and other arguments remain intact.

On is immediate passthrough: with every row On the hooks return before reading
the scope or any context. Off substitutes the named `null` resource and calls
the original, so downstream bookkeeping still runs. Only `edge_fire3_screen2`'s
screen request is skipped. New effects follow the setting during the same
match; already created effects expire naturally. There are no per-frame
deletion sweeps.

## Differences from the peer

Off removes included layers even where the peer merely shrank them. Reduced
retains selected cues and lighter layers. The policy is not an exact copy of
the peer.

Shared effect names require the correct fighter/weapon category, kind and
move. There is no owner traversal or global victim timer. Forty-one named
projectile/position, reflector/counter and hitbox cues are preserved,
including Palutena's up-air marker; they are listed under `excluded_cues` in
the policy. General hit/shield/KO feedback remains outside the filter.
Article/model/posture changes, camera/LOD changes, color/flash/afterimage
command hooks and unidentified raw hashes are not ported. Generic
`sys_attack_arc_d` and unobserved creation variants (NO_SCALE, COLOR,
UNSYNC_VIS, RND, VARIATION) are outside current coverage. In particular, a
separate article or scripted tint can remain visible in Off.

[Initial targets](../reference/fx-policy.json) and
[expanded targets and qualifications](../reference/fx-expanded-policy.json)
record the policy; raw source presence alone does not establish runtime
coverage.

## Evidence and provenance

User archive: effect disable-reduce source.zip, May 14 master/screen sources.
The source SHA and per-profile source groups are in the expanded policy.
Vanilla dumped scripts independently identify the actual PK Thunder names:
[Ness](https://github.com/WuBoytH/SSBU-Dumped-Scripts/blob/59004685238a3f6f93bae905d3a4e079701c40b4/smashline/lua2cpp_ness/ness/SpecialAirHi.txt) and
[Lucas](https://github.com/WuBoytH/SSBU-Dumped-Scripts/blob/59004685238a3f6f93bae905d3a4e079701c40b4/smashline/lua2cpp_lucas/lucas/SpecialAirHi.txt).
The Reduced keep/cull names are parent requests at that pin, regenerated by
`tools/generate_reduced_families.py`.

Palutena/Sephiroth creation names were corroborated with vanilla scripts and
local EFF resource names extracted read-only from data.arc. No Nintendo asset
or compiled peer plugin is included.

The bounded primitive Lua bridge follows skyline-smash bed642e: L2CAgent 0x48
bytes, 24-byte L2CValue return forces AArch64 x8, with unused MaybeUninit
padding. At most 32 primitive arguments are retained; unknown or oversized
commands pass through.

All runtime evidence so far was gathered with **Online Deluxe** builds and its
single global FX row (NRO prefixes `c1706bbe`, `5d47c704`, `833cc40b`,
`24e714e7`, `7c435729`, `b99...`, `c0e9a625`): Palutena/Sephiroth Off/On,
Sephiroth neutral-B suppression, the Reduced families in the
[runtime matrix](FX_RUNTIME_MATRIX.md). ssbu-fx-control runs the same request
pipeline but has **no runtime acceptance of its own yet**: per-fighter rows,
the plugin's own online hooks and the Deluxe page are host-tested only.

## Reduced implementation

Since 0.2 the request policy is per move (`src/moves.rs`): each move under
study lists its requested layers and numbered cull-set variants; the INI
selects one variant per move (`variant.<move>=N`) and Reduced nulls exactly
that set, keeping every other layer. A move with a context profile claims a
request only from an actor in that move (a Gigaflare layer requested
elsewhere is preserved); a move without one (Pyra's specials, Kirby's hammer,
PK Fire) claims its layers by name. Variants are accepted only through the
console protocol in [HARNESS](HARNESS.md). The character-wide and
appearance-chosen tables of 0.1.x are on the archive branch.

In 0.1.x, Palutena's up-air resource had a separate lighter copy (an EFF
sibling, see below) so its visible cue remained; that copy no longer loads. The Off policy deliberately
protects that up-air parent, so Off is not a zero-cost reference for that
parent.

### EFF siblings (historical, off since 0.2.0)

In 0.1.x, EFF siblings existed for Palutena up air, Shadow Flare
charge/projectile layers, the Inkling roller splashes and Chrom's first side-B
trail. The plugin registered one ARCropolis load callback per file
(`ef_palutena.eff`, `ef_edge.eff`, `ef_inkling.eff`, `ef_chrom.eff`); each
loaded the stock file into ARCropolis's buffer and appended that file's
siblings, which answered the original's `kill_kind`/`detach_kind` and ACMD
`EFFECT_OFF_KIND`/`EFFECT_DETACH_KIND` so cleanup worked after Reduced was
switched back to On.

Since 0.2.0 the feature is off: the plugin registers no ARCropolis load
callback, no sibling file loads, and Reduced is exactly the per-move variant
from `src/moves.rs`. ARCropolis is not needed by this plugin. Chrom and Inkling
have no Reduced variant. Event codes 91 to 95 are no longer emitted; the
`kill_kind` and `detach_kind` hooks (264, 265) are still installed. The code
survives only on the branch `archive/0.1.x-appearance-variants`.

The roller runtime check (Deluxe `24e714e7`) exposed an Off-table alias gap:
the peer names `inkling_roller`, while the game requests
`inkling_roller_splash_walk`, `_run` and `_air`; those three parents are in the
Off table. Ground ink and gameplay articles remain outside the filter.
