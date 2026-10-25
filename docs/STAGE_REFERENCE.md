# Stage background reference

Internals and evidence behind the [Stages guide](STAGES.md): where the stage
list and IDs come from, the 13.0.5 hook guards, trace events and the dated
runtime history. The per-target inventory is [STAGE_TARGETS](STAGE_TARGETS.md);
what still needs testing is the [acceptance checklist](#acceptance-checklist).

[Stage list](#stage-list-and-ids) · [Matching](#matching) ·
[Stock resources](#stock-resource-evidence) ·
[Particle guard](#request-boundary-13-0-5-only) ·
[Model guard](#model-render-boundary-13-0-5-only) ·
[Trace events](#stage-trace-events) · [Applying changes](#applying-changes) ·
[Runtime history](#runtime-history) · [Acceptance checklist](#acceptance-checklist)

![Two Off paths. Particle: the request at 0x3563430 becomes the named null resource only when the caller, name, live stage ID and row Off all match. Model: a group recorded at construction by a healthy registry from the primary vtable and initializer has its draw argument at 0x35d73e0 forced off only when the registry is healthy and the bucket is free, the renderer has one recorded path and the primary vtable, the stage ID and path pair is in MODELS, the row is Off and the native argument requests the draw. Native visibility is not changed; both paths need their install-time byte guards.](media/stage-off-paths.svg)

## Stage list and IDs

The legal list comes from the mod-written `settings.toml` and the eleven
label/key pairs in archived Elite NRO SHA256
`6a10a9777a80814c57f9d49a4760b8c8fdbcdb77b1ec09e7c68a65b00679abc5`.
IDs come from skyline-smash `bed642e37da23bbf2db2544bb6ac119007d1cbf0`,
`src/lua_const.rs::StageID`, not the unrelated sequential `smush_info_shared`
enum. Prior Deluxe logs corroborate 347/361, not every ID individually.

Battlefield ARC keys are **`battlefield`/`battlefield_s`**, while stable INI keys
remain **`battle_field`/`battle_field_s`**.

## Matching

Exact particle names are the hand-reviewed `BACKGROUNDS` table in `src/stages.rs`;
no wildcards, texture names, shared `sys_*` effects, or fighter emitter names are
matched. The `MODELS` table identifies exact **full normal-model folder paths**
plus stage ID, independently excluding other form roots with identical leaf
names. Classification uses stock package semantics, parsed mesh/material
assignments and separate gameplay groups, with the contrast-preservation
exclusions established by the Ally review. Unclassified scenery remains visible.

## Stock resource evidence

An offline wrapper around the existing `smash-arc` 0.6.0 reader extracted eight
packages from the local `data.arc`. Their observed paths are
`effect/stage/<key>/ef_<key>.eff` for the eight stages below. EFFN magic, header
count and exact uppercase `STG_*` names in the first 4096 bytes were checked; the
name count equals the header count in every file. This is a bounded header
census, not a complete EFF parser. Assets stay outside the repo.

| Package key | Bytes | SHA256 |
| --- | ---: | --- |
| end | 1782424 | `096d3cf97c6ccdafe7b5a9b5c431090bf2ba85a28689672ae3a02b986fc76541` |
| yoshi_cartboard | 53760 | `e6513760739fc9fd7a2a9ee9c5c8bbb38ec6939dd8c045793eb3d24f5df8770a` |
| fox_lylatcruise | 1384960 | `eea02b9b14c1845b7d4685ff749429696f87639cf86d1a8a82f75b1bfcac360c` |
| poke_stadium2 | 168520 | `9d98c2561b2ee3288fbe85f8280257c3128eed8cfb17d84f98e359be959273e4` |
| poke_kalos | 6111992 | `f0a42cceccda8f2212d44b2fe1603b43ff044ce133ede9bdeff159e9af432f39` |
| animal_city | 479744 | `b0c024f4d6cab108930b2ee9b8340886d330c21b94e5f1e5291e16aeab7d906c` |
| ff_cave | 2929152 | `e401df427302d538af482f463091335a699c7b1e3a8c677f19483be7e3e2b4a5` |
| trail_castle | 2626104 | `1b7f7c2f0aed9a6663ef69b4003c519e5e6fe61d47baf182c10ac9c760958d49` |

## Request boundary (13.0.5 only)

Ordinary local static analysis, **not REA or runtime evidence**, used uncompressed
NSO SHA256 `8b3dc2eb011f26dda1f87947e4f62665e8d8fe3d84397e85ecf9b3cd9d0137ce`,
build ID `21450c647b8c5940ed5b68218ea128fd3c5d597d000000000000000000000000`.

- **Entry.** EffectManager request entry **`0x3563430`** saves x1 at `+0x4c`,
  masks it to 40 bits at `+0x68` and looks up the resource before effect
  allocation. A missing resource returns 0 through the original unlock/return
  path (`+0x22c`, `+0x238`).
- **Change.** The inline hook validates eight entry words and these four semantic
  words. It changes only x1 to the existing named `null` resource; original
  execution continues. No object deletion, kill/detach hooks, allocation or
  locks are added.
- **Callers.** Only callers returning to **`0x25efcdc`, `0x25efe14`, `0x25effa4`,
  `0x25f0f1c`, `0x2626a70`, `0x2627e90`, `0x2627f44`** qualify. These
  stage-effect paths call that manager directly. Other manager callers,
  including fighter/item requests, remain original even with a matching name.
- **Names.** A lowercase name Hash40 is looked up in a compile-time sorted
  47-entry table. Stage-record comparisons at `0x297ed94`/`0x297f14c` (Lylat)
  and `0x26636dc` (Town bubbles) corroborate lowercase handles despite
  uppercase EFFN labels.
- **Record flow.** The stage record hash at +8 flows via `0x2626be0` into
  request-object+`0x40`, loaded into x1 at `0x25efcac` before the manager call
  at `0x25efcd8`. The three `0x262...` return sites use the same post-lookup
  record field and manager entry. Their direct `bl` instructions are at
  `0x2626a6c`, `0x2627e8c` and `0x2627f40`; the string/hash slot wrappers
  resolve a slot to its effect hash before requesting it. These are exact
  sites, not a range or a rule accepting every caller with a similarly shaped
  argument block.
- **Caller audits.** Surface code 119 observed all five FD names here with
  name/stage/Off true but the former caller gate false. Static bytes and
  runtime evidence are separated in external
  `fx-surface-1022/fd-caller-audit/REPORT.md` (ordinary Capstone analysis, not
  REA). Lylat code 119 then identified `0x25f0f1c`: its `bl` at `0x25f0f18` is
  the indexed sibling using the same stage-slot list, filler and request block.
  Its narrow audit is `fx-surface-1022/particle-followup/REPORT.md`; neighboring
  returns `0x25f0bc0`/`0x25f0dd4` remain excluded. No additional effect names
  were added.
- **Stage ID.** `_ZN3app14sv_information8stage_idEv` checks the absent
  singleton/inactive-stage sentinel and returns −1; only the exact normal stage
  ID for that name qualifies. Unknown IDs and other stage/name pairs pass
  unchanged. Alternate-form behavior still needs the negative-control checks in
  [Remaining runtime acceptance](STAGES.md#remaining-runtime-acceptance).
- **Cost.** Every row On and tracing absent is a fast return before name/stage
  queries. Otherwise filtering uses bounded binary search, atomic state and
  bounded logs. Fighter scope is deliberately not consulted: the online mark can
  arrive about three seconds after stage load, after persistent ambient effects
  have started.
- **Unchanged.** Existing fighter hooks, Deluxe online-source selection and scene
  sites are unchanged. No ARCropolis callbacks or stage-asset rewriting are
  introduced.

Static evidence/receipts are archived outside the repo under
`plugin-api-tangent-20261012/stage-background-1021/`: `RESOURCE-CENSUS.json`,
`STATIC-BOUNDARY.json`, analysis scripts, package extracts and the task plan.

## Model-render boundary (13.0.5 only)

The draw hook changes **only w1 at `0x35d73e0`**, the call to `0x35fab50` that
updates the current renderer's draw records. Requested visibility at renderer
`+0x4b0` and effective visibility at `+0x4b1` stay native. Their getters can feed
back into stage-wrapper state, which is why the earlier setter override was
rejected. The stage visibility routine `0x25edb60` is not hooked.

1. **Capture** actual resource identity at **`0x35e26a0`**, after the primary
   constructor's initial resource-binding call has returned. x22 is the renderer;
   w23 retains the search-path index already validated by native lookup. Only
   primary vtable `0x5232670` with the expected resource initializer is accepted.
2. **Resolve** a **SearchLookup index**, not a DirInfo index. The bounded lookup
   mirrors `0x35c26e8..0x35c2750`: ResourceManager `0x5332f20` → tables `+0x78` →
   search section `+8`; check the index count, apply optional index map `+0x28`,
   check the path count, then read the `0x20`-byte path-list entry at `+0x30`.
   Its parent at `+8` is the full model folder, without a trailing slash. Filename
   and extension must be `model.numdlb` / `numdlb`. Mapped-memory checks occur
   here, during construction, not on every renderer pass.
3. **Store** only selected full paths in a fixed 128×8-entry association table.
   It retains numeric addresses, not native references; it neither prolongs
   object lifetime nor allocates model resources. Conflicting paths become
   unclassified.
4. **Invalidate** before base construction (`0x35c32d0`), destruction
   (`0x35c3ab0`), and **all three identified rebinding entries** (`0x35c7aa0`,
   `0x35d0e30`, `0x35f73e0`). Later rebinds cannot re-associate from a wrapper's
   stale directory. Clones and constructors outside the verified capture path
   remain native.
5. **Draw.** At the draw call, use an eight-entry lookup, the current stage ID,
   exact full folder path, and that row's Off bit. The renderer must still carry
   primary vtable `0x5232670`, and only a native request to draw (w1 bit 0) is
   changed: native false never becomes true. The rendering hot path performs no mapping query or resource traversal.
   Each of the 128 buckets has its own shared-reader `RwLock`; draw uses
   `try_read`. Readers coexist, and a different bucket's writer cannot make this
   lookup skip. Same-bucket writer contention leaves the current pass native.
   Writers wait only for short eight-entry bookkeeping sections in their bucket;
   no engine call or I/O occurs under that lock. Capacity exhaustion or a
   poisoned writer lock disables model suppression until restart.
6. **Effect.** `0x35fab50` updates current-entry draw bytes. It does not change
   the renderer's visibility getters, stage wrapper, resource-set indices or
   transitions. Some auxiliary slots and other passes (e.g. shadows/reflections)
   may remain visible; complete disappearance and work saved require runtime
   observation.

All seven model hooks are installed together only after byte guards validate
their maximum patch spans and the identity/visibility flows. Particle and model
capabilities remain separate; a rejected model hook or failed registry disables
its six rows and the bulk All control, while the particle rows remain usable.

### Model evidence

Stock model evidence is under external `stage-background-remaining-1021/`:
`MODEL-INVENTORY.md`, two ARC hierarchy inventories, `PARSED-MODELS.json` (218)
and `PARSED-BATTLEFIELD-MODELS.json` (38), and `MODEL-TARGETS.json`. A bounded
SearchLookup walk extracted only small MODL/material descriptors; ssbh_lib 0.19.0
parsed complete MODL entries. Nintendo files remain outside the repository.

The original 29 reviewed identities (now 18 targets and 11 preserved backdrops)
were checked against the normal/model folder hierarchy, SearchLookup descriptor
parents, DirInfo paths and descriptor SHA256s. 22 additional explicit
preservation examples include Battlefield's main ring/floating floors, Yoshi's
ground/floating floors/Randall candidate, Stadium's main stage/ink meshes/monitors,
Smashville's main/substage, Kalos's battle ring, pass-through platforms/bridge,
and transition groups. Neither an `inkMesh` nor a known playable-group identity
is included. Names and assignments support this candidate classification;
runtime geometry/cue checks are still required.

## Stage trace events

`TRACE_EFFECTS` adds **event 116** for first-seen targeted stage/model groups,
separate from particle event 114 and fighter event 110. **Event 115** records
forced draw-record overrides, sampled first 16 and every 128. Both pack stage ID
in b's high 32 bits and ordinal/count in its low 32 bits, with the **full
folder-path Hash40** in a. They share the existing bounded 512-key trace ledger.
`tools/resolve_trace.py` decodes all three trace types; model-path hashes refer
to `MODELS` and the review inventory, not the effect-name dictionary. The
previous setter-based packages' leaf hashes are not interchangeable with this
candidate's trace keys. All event codes are listed in
[CONFIGURATION_REFERENCE](CONFIGURATION_REFERENCE.md#event-codes).

## Applying changes

New requests follow the selected policy. Already created effects finish
normally; turning On does not respawn missed effects. Captured models follow
their next draw-record update; On restores the native argument on that pass.
Live changes and restoration were observed on the Ally; the comparisons also
unload and reload each stage to cover persistent requests. In-place resource
rebinding drops the association and leaves that instance native; a newly
captured constructor instance is needed to target it again.

Both pages ship in one NRO, using the existing SDK pin / descriptor 4. Stage
rows use the plugin's background saver exclusively (`no_persist`, no host config
path); game hooks and menu callbacks perform no file I/O.

## Runtime history

Dated, scenario-specific emulator evidence. It is separate from the later
[operator testing report](MEDIA.md#operator-reported-testing), and none of it is
Switch or performance evidence. 0.3.0 retains the runtime source of the
Surface-tested `8bf2bdb4…` candidate.

### Summary by stage

Latest samples in Eden (Training, normal forms). Complete phase coverage and
per-emitter visual effectiveness remain unverified.

- **Battlefield forms:** the four decorative groups Off left floor, ledges,
  platforms and fighters clear; sky city disappeared Off and returned On.
  After the lock correction, Small Battlefield showed no island flashes in two
  Off samples.
- **Final Destination:** all five names reached the gate and sampled
  suppression (events 114/112); ring aura absent Off, back on fresh On reload.
- **Yoshi's Story:** stable Off sample; sea/islands restored On.
- **Lylat Cruise:** all five names reached the seventh audited caller with
  sampled suppression (event 112); ship and fighters readable. Its baseline
  whiteout also reproduces without FX, so it is not an accepted stage.
- **Pokemon Stadium 2, Kalos:** searchlights-only and decorative-lights-only
  passed sampled geometry/contrast checks.
- **Smashville:** keeping the phase-1 sky native removed the Off whiteout in
  two samples; other time-of-day phases remain untested.
- **Town and City:** bubble requests showed sampled suppression (event 112);
  wind was not observed.
- **Northern Cave:** all five names showed sampled suppression (event 112).
- **Hollow Bastion:** checked only with modified Eternal Heart resources, for
  geometry.

### Ally visibility review — 2026-10-21/22

External evidence: `plugin-api-tangent-20261012/stage-playability-ally-1021/RESULTS.md`
and its screenshot prefixes. Normal forms, Training, fixed-camera On/Off/On
reloads, SSBU 13.0.5 on Eden/AMD. The original eight plugins remain installed.

- **Battlefield forms.** Baseline 29 targets caused reversible whiteout.
  Preserving only Battlefield's sky was insufficient: isolation 1
  (`b90f042b49d119d1`) reproduced glare with its landscape group alone. Both
  landscape and sky now remain native. Isolation 2 (`e4d415b279bb1581`) tested
  the separate four-group decorative subset on both Battlefield forms: main
  floor, ledges, platforms and fighters remain clear, without the former blue
  band/whiteout. Distant sky city disappears Off and returns On; later Off
  samples also show reduced mist. Events 115/116 reached all eight selected full
  paths. This is sampled visibility/override evidence, not complete removal of
  every mesh/pass or an isolated efficacy test of each group.
- **Yoshi's Story, Stadium 2, Kalos.** Sea/islands-only, searchlights-only and
  decorative-lights-only passed sampled geometry/contrast in isolation 1. Their
  former sky/backdrop targets caused whiteout; On restoration and visible Off
  differences were observed. This does not cover every phase or draw pass.
- **Smashville and particle stages.** Smashville's nighttime sample preserved
  geometry. Other time-of-day groups remain untested. Final Destination, Town
  and City, and Northern Cave cave-phase samples were readable, but no
  qualifying particle events 112/114 were recorded. A bounded Town and City
  Off/reload repeat in isolation 2 was also readable and again recorded neither
  event. Its current log has contiguous sequences 1..529 and no loss markers.
  Predicate misses are silent, so an unrequested phase cannot yet be
  distinguished from a hash/caller/live-stage-ID mismatch. Hook installation and
  a clean image do not establish suppression efficacy.
- **Lylat Cruise.** Whiteout also reproduces in a fresh process with FX absent,
  with Vanilla+ either On or Off. Its baseline cause is unresolved; it is not an
  accepted stage.
- **Hollow Bastion** was tested with **Eternal Heart** resources (Dive to the
  Heart), not stock resources. Only that modified sample's geometry was checked.

### Surface correction review — 2026-10-22

External evidence: `plugin-api-tangent-20261012/fx-surface-1022/`, including
immutable source/NRO/ELF manifests, event logs, input receipts and moving
captures. Normal stages in Training, SSBU 13.0.5 on Eden/NVIDIA, isolated
profile with only Deluxe `2f5a1d4e...` and FX. No Training Modpack or ARCmods.
These are not Switch tests or performance measurements; 200-image MJPEG samples
are captured images, not emulated-frame counts.

| Build | Change under test |
| --- | --- |
| `e4d415b2...` | Original candidate |
| `6590c497...` | Observational diagnostics |
| `26e978a1...` | Registry-only lock correction |
| `39744aa9...` | Smashville sky retained |
| `748230cd...` | Three audited return addresses added |
| `8bf2bdb4...` | Seventh caller `0x25f0f1c` added |

- **Contention.** Original `e4d415b2...` and observational `6590c497...`
  reproduced SBF island flashes and Smashville blue/white bloom transitions.
  Diagnostics found actual model lookup contention in the former global
  reader/writer mutex.
- **Registry.** Registry-only `26e978a1...` removes reader-reader and
  cross-bucket contention. SBF Off/On/Off showed no island flashes in two
  200-image Off samples; On restored the islands. The sampled diagnostic counts
  reported no contended lookups. Native sky, landscape, geometry and fighters
  remained visible. This does not prove every draw pass or same-bucket lifecycle
  transition is contention-free.
- **Smashville sky.** With the registry corrected, Smashville Off was
  consistently white. Retaining its sky while leaving village/clock/window
  targets unchanged (`39744aa9...`) removed that whiteout in two Off samples; On
  restored the village. The observed active sky was phase 1 (`0x2e9c2174d1`).
  All four sky identities now remain native; the other time-of-day phases and
  transitions still need runtime acceptance.
- **Yoshi's Story** with `39744aa9...` showed a stable Off sample and restored
  its sea/islands On. Sky, fighters and playable geometry remained visible;
  event 116 reached `0x2a64a18d77`, with no contended lookups in the sampled
  counters.
- **Final Destination.** Caller correction `748230cd...` preserves that model
  policy and adds only the three audited return addresses. Fresh FD Off now
  reaches events 114/112 for all five FD names, with code 119 showing all four
  predicates true. Sampled geometry/fighters remain visible; ring aura is absent
  Off and returns on fresh On reload.
- **Lylat Cruise.** `748230cd...` reached the correct names/stage/Off state but
  rejected return `0x25f0f1c`. The narrowly audited seventh-caller candidate
  `8bf2bdb4...` was then tested with fresh Off / On / Off reloads. All five
  configured Lylat names reached that exact caller with all four predicates true
  while Off; event 112 sampled suppression for all five. The same names also
  appeared with the row-Off predicate false while On. Ship, fighters and native
  scenery stayed readable in the captured samples. CPU and initial phase
  differed across reloads; these are request-gate and sampled visibility checks,
  not an isolated visual disappearance comparison for each emitter or a claim to
  remove all background FX.
- **Town and City, Northern Cave.** Longer `748230cd...` captures also reached
  Town and City's bubble requests at existing return `0x25efe14`, and all five
  Northern Cave names at `0x25efcdc`, with all predicates true and sampled
  event 112 suppression. Town's wind was not observed. Those rows target their
  listed particles, not the town/cave geometry. Their early quiet captures did
  not establish a broken request path.

Each corrective build passed 90 Rust tests, host/Skyline Clippy with denied
warnings, no-default-features build, Skyline release and focused formatting.
Tracing was enabled for attribution; no stock-clock FPS benefit is inferred.

## Acceptance checklist

Still required for each of the eleven stages
([status](STAGES.md#remaining-runtime-acceptance)):

1. Capture On / Off / On after reload with `TRACE_EFFECTS` to establish
   event 114/116 reachability and event 112/115 suppression.
2. Check stage geometry, platforms, hazards, fighters/items, hit effects,
   loading, KO/results and rematch.
3. Include other stages and alternate forms as negative controls.
4. Check that persistent visuals disappear and reappear only as documented,
   including Smashville time-of-day changes, stage hazards/transformations and
   model recreation.
5. Then remove tracing and compare equal-duration stock-clock captures: same
   stage, fighters, actions, render mode and camera. Record frame-time
   distributions and worst one-second windows; no FPS benefit is inferred from
   fewer resource requests.

Historical Ally tests and the isolated Surface correction fixture are separate.
These emulator results do not establish Switch runtime compatibility or full
phase, negative-control and performance acceptance.
