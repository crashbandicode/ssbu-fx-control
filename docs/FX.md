# Character effects

On forwards original requests. Off suppresses the named character effects
below for a fighter whose row is Off, when the scope allows it. Reduced keeps
selected move cues and removes surrounding decoration. Its goal is to approach
Off's frame-time stability; a visible change alone does not establish that
benefit ([performance method](FX_PERFORMANCE_METHOD.md)).

This policy is Online Deluxe's character-FX feature, moved into its own
plugin. The Off tables, the request classification (`effects_reduced::classify`,
`request_hash`) and the hooks are ported from
`intpa/ssbu-online-deluxe-rewrite` at `f3fe436`. What changed is the mode:
Deluxe had one global FX mode; this plugin has one per covered fighter, and
Reduced is the per-move variants of `src/moves.rs` plus the harness `cull=`
list (see Reduced implementation), not Deluxe's character-wide tables.

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
   (`effects::TARGET_NAMES`, `effects_catalog::LEGACY_NAMES`), the name tables
   of the moves in `src/moves.rs` and the Gigaflare screen overlay. Each
   name is classified by its character prefix; a name with no prefix, or two,
   fails the build.

| Prefix | Fighter |
| --- | --- |
| `captain_` | Captain Falcon |
| `ness_`, `P_Ness` | Ness |
| `lucas_`, `P_Lucas` | Lucas |
| `palutena_`, `P_Palutena` | Palutena |
| `edge_`, `P_Edge` | Sephiroth |
| `eflame_`, `P_Eflame` | Pyra |
| `elight_`, `P_Elight` | Mythra |
| `master_` | Byleth |
| `inkling_` | Inkling |
| `kirby_` | Kirby |

A request no fighter owns passes through untouched, whatever the rows say.
The plugin covers exactly ten fighters (`fighters::ALL`: Captain Falcon, Ness,
Lucas, Palutena, Sephiroth, Pyra, Mythra, Byleth, Inkling, Kirby), each with a
page row and an INI line. Kazuya, Ike, Chrom, Roy and Ganondorf were removed
from the code in 0.2.0 (no fighter entry, Off tables, move profiles or name
prefixes): their requests pass through untouched, like any uncovered fighter's.
The 16 move profiles that remain are Falcon Punch, Ness up special, Lucas up
special, Palutena (rapid jab, neutral special, up special, side special, up
smash, tilts, wing smash, up air) and Sephiroth (forward smash, purple dash,
neutral special, down special, side special).

Shared names inside a move table follow the move's owner, never their prefix:
`sys_flash` (Ness and Lucas up special), `sys_smash_flash` and
`sys_smash_flash_s` (Sephiroth smashes) and Sephiroth's prefix-less
emitter-style parents (`flare1`, `aura1`, `line1`, and so on). Those names need
a context; without one they pass through. The test
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

The catalog in the code is `effects_catalog::LEGACY_NAMES` (276 names from the
peer's unconditional legacy tables, including three verified Inkling roller
splash parents), `effects::TARGET_NAMES` (47 names, 15 of them also in the
legacy table) and 16 qualified move profiles (`effect_context::Profile`). The
ledgers in `reference/` keep the import-time numbers, from before Kazuya, Ike,
Chrom and Roy left: 317 global names, 19 profiles and 124 initial targets.
Aliases and emitter-child names are source candidates, not proof that every
name reaches a request hook. The README has the per-move table.

The plugin hooks the ordinary EffectModule request wrappers plus ten ACMD
creation variants: EFFECT, EFFECT_FOLLOW, EFFECT_FOLLOW_ALPHA,
EFFECT_FOLLOW_NO_STOP, EFFECT_FLW_POS, EFFECT_FLW_POS_NO_STOP, LANDING_EFFECT,
EFFECT_FOLLOW_FLIP, EFFECT_FOLLOW_FLIP_ALPHA and EFFECT_FOLLOW_NO_STOP_FLIP.
Flip variants filter their first two effect names independently, each with its
own owner; joint, transform, scale and other arguments remain intact.

On is immediate passthrough: with every row On the hooks return before reading
the scope or any context, except while the `TRACE_EFFECTS` marker is latched
(the EffectModule request hooks then read the actor's context) and for the
Special Zoom probe names, which read the scope first. Off substitutes the named `null` resource and calls
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
The Reduced layer and cull names in `src/moves.rs` are parent requests at that
pin, written by hand.

Palutena/Sephiroth creation names were corroborated with vanilla scripts and
local EFF resource names extracted read-only from data.arc. No Nintendo asset
or compiled peer plugin is included.

The bounded primitive Lua bridge follows skyline-smash bed642e: L2CAgent 0x48
bytes, 24-byte L2CValue return forces AArch64 x8, with unused MaybeUninit
padding. At most 32 primitive arguments are retained; unknown or oversized
commands pass through.

Runtime evidence for the Reduced families was gathered with **Online Deluxe**
builds and its single global FX row (NRO prefixes `c1706bbe`, `5d47c704`,
`833cc40b`, `24e714e7`, `7c435729`, `b99...`, `c0e9a625`): Palutena/Sephiroth
Off/On, Sephiroth neutral-B suppression, the Reduced families in the
[runtime matrix](FX_RUNTIME_MATRIX.md). FX Control 0.1.0 ran in Eden next to
Deluxe 0.5.0 builds for the per-move captures in the quota section of the
[emulator results](FX_EMULATOR_RESULTS.md). 0.2.0 has **no runtime acceptance of
its own yet**: its per-move variants, the descriptor version 4 page and the
plugin's own online hooks are host-tested only.

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
sibling, see below) so its visible cue remained; that copy was deleted in
0.2.0. The Off policy deliberately protects that up-air parent, so Off is not
a zero-cost reference for that parent.

### EFF siblings (deleted in 0.2.0)

The 0.1.x EFF sibling feature (ARCropolis load callbacks that appended lighter
copies of Palutena up air, Shadow Flare, the Inkling roller and Chrom side B)
was deleted in 0.2.0: the modules, hooks 264 and 265 (`kill_kind`,
`detach_kind`) and the ACMD hooks 0x11a and 0x11b (`EFFECT_OFF_KIND`,
`EFFECT_DETACH_KIND`) are gone, and ARCropolis is not needed. Reduced is
exactly the per-move variant from `src/moves.rs`; Inkling has no Reduced
variant. The code survives only on the branch
`archive/0.1.x-appearance-variants`.

The roller runtime check (Deluxe `24e714e7`) exposed an Off-table alias gap:
the peer names `inkling_roller`, while the game requests
`inkling_roller_splash_walk`, `_run` and `_air`; those three parents are in the
Off table. Ground ink and gameplay articles remain outside the filter.
