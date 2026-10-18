# ssbu-fx-control

> **Disclaimer:** this project was developed with the assistance of AI
> (Claude, Codex and other coding agents) under the operator's direction and
> review. Nothing is claimed beyond what is recorded in these documents.

A Skyline plugin for **Super Smash Bros. Ultimate 13.0.5**, delivered as **one
NRO**, that turns selected character effects **On**, **Off** or **Reduced**
for each covered fighter, with an **All** switch. It is the character-FX
feature of [Online Deluxe](https://git.syntaxbyexample.com/intpa/ssbu-online-deluxe-rewrite)
moved into its own plugin.

- **On** (default) forwards every original request.
- **Off** substitutes the named `null` resource for the covered effects of
  that fighter, so the request path still runs.
- **Reduced** applies, per move, a numbered **variant**: a cull set of that
  move's effect layers chosen to keep the console at 59 fps while the move
  plays, with the gameplay cue kept. Reduced never silently performs Off: a
  fighter without a variant is offered On / Off only.

Changes are visual request-path only. Gameplay articles, hitboxes, camera,
hitstop, RNG and simulation are never touched. Effects already on screen
finish normally; new requests follow the setting.

## Status: 0.2.1, measurement first

0.2.1 is the 0.2.0 code with the plugin SDK pinned to Deluxe 0.5.4 (`a63bc8c`)
and the documentation brought in line with the code. It registers with
descriptor version 4, so the page needs Deluxe 0.5.3 or later.

The 0.1.x line (branch `archive/0.1.x-appearance-variants`, release 0.1.0)
chose its Reduced variants by appearance; some measured slower than On. This
line restarts from measurements: `src/moves.rs` lists the moves that the
2026-10-15 console captures put on the ledger, every effect layer their
scripts request, and 1..N candidate variants per move. **No variant is
accepted yet.** The protocol that accepts one is [docs/HARNESS.md](docs/HARNESS.md);
the console result so far is that Vanilla and LessLag hold 60 on every
move and only LLUltra drops, so every pass runs on LLUltra.

| Move | Fighter row | Variants (lightest first) | Source |
| --- | --- | --- | --- |
| Falcon Punch | Captain Falcon | 1 wings off (0.1.x) · 2 wings, hold and flash off | fighter + weapon SpecialN |
| Blazing End | Pyra | 1 trails and illusion off · 2 + sword flame · 3 everything but sword open, throw and return | SpecialS/AirS/Flick/Catch, esword Fly/Rotate/Back/Hit |
| Flame Nova | Pyra | 1 hold aura and trail · 2 + hold sword, ring1 · 3 only sword and explosions | SpecialNStart/Hold/N1/N4 (N2, N3 pending) |
| Prominence Revolt | Pyra | 1 fire2, trail end, smoke, pillar ground · 2 + every sword fire and the pillar impact | SpecialHiStart/Hi, firepillar SpecialHi |
| Specials | Mythra | 1 sword light layers (0.1.x; specials not extracted) | 0.1.x |
| Explosive Flame | Palutena | 1 appear, wand lights, backlight off (0.1.x) | SpecialS, explosiveflame Explode |
| PK Thunder | Ness | 1 launch burst off (0.1.x) | SpecialAirHi, pkthunder Move |
| PK Fire | Ness | 1 hand flash off · 2 + fire pillar | SpecialS/AirS, pkfire Shoot/Pillar |
| PK Thunder | Lucas | 1 launch burst off (0.1.x) | SpecialAirHi, pkthunder Move |
| Hammer | Kirby | 1 charge glow off · 2 + smoke and wind · 3 only arc and max burst | SpecialS*/AirS*, kirby_hammer SpecialS* |
| Specials | Byleth | 1 axe flare 3 and hold 2 (0.1.x; specials not extracted) | 0.1.x |
| Roller | Inkling | none: Reduced was an EFF sibling in 0.1.x, which is deleted in 0.2.0; Inkling offers On / Off | 0.1.x runtime check |
| Gigaflare | Sephiroth | 1 first burst and sign off, overlay skipped (0.1.x) | edge_fire Burst S/M/L |

Kazuya, Ike, Chrom, Roy and Ganondorf have no move here and are removed from
the code: no fighter entry, no Off tables, no move profiles, no name prefixes.
Their effect requests pass through untouched, like any uncovered fighter's, and
their INI lines are unknown keys (ignored on load, kept on rewrite). The 0.1.x
EFF sibling feature (Palutena up air, Shadow Flare, Inkling roller, Chrom side
B) is deleted: the `kill_kind` / `detach_kind` hooks (264, 265) and the ACMD
`EFFECT_OFF_KIND` / `EFFECT_DETACH_KIND` hooks (0x11a, 0x11b) are no longer
installed, the plugin registers no ARCropolis callback, and Reduced is exactly
the per-move variant from `src/moves.rs`. ARCropolis is not needed by this
plugin. The removed code survives only on the branch
`archive/0.1.x-appearance-variants`.

## Install

1. Copy `libssbu_fx_control.nro` to
   `sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/`.
2. [Online Deluxe](https://git.syntaxbyexample.com/intpa/ssbu-online-deluxe-rewrite)
   is optional. With Deluxe 0.5.3 or later the plugin gets its own page in
   Deluxe's window (0.2.0 uses the version 4 header and per-move value rows,
   which Deluxe 0.5.0 to 0.5.2 refuse: no page, effects still work from the INI)
   and uses Deluxe's online tracking. Without Deluxe, edit the INI and the
   plugin tracks online play itself.

Settings live in `sd:/ultimate/ssbu_fx_control/config.ini`, created with the
defaults on first start (everything On, scope Online, variant 1 per move).
See [configuration](docs/CONFIGURATION.md).

Do not run it next to an older Deluxe (0.4.x) with that FX row set to Off or
Reduced: both would filter the same requests.

## The Deluxe page

With Online Deluxe 0.5.3 or later, the plugin registers an own page titled
**FX Control** in Deluxe's window cycle (L+R, then D-pad Down):
settings, plugins, **FX Control**, performance, hidden. Up/Down selects a row,
Left/Right changes it (with ZR held while a fight is live). In a match the
page becomes Deluxe's compact strip.
Deluxe 0.5.3 has an INI key `hide_pages=plugins,performance` and
Settings rows that take pages out of this cycle, so it can also run settings,
FX Control, hidden.

The page has 33 rows:

| Row | Choices | Notes |
| --- | --- | --- |
| Scope | Online / All | Online: online play outside Training. Deluxe 0.5.0 to 0.5.2 gives the online signal but refuses the page, so the INI alone sets the scope. With a Deluxe older than 0.5.0 there is no online signal either: Online never applies (effects stay original until `scope=all` is set in the INI). |
| All | On / Off / Reduced | Sets the ten fighter rows. Shows `Unavailable` while the rows differ. |
| Captain Falcon, Ness, Lucas, Palutena, Sephiroth, Pyra, Mythra, Byleth, Inkling, Kirby | On / Off / Reduced | One row per fighter with a move on the ledger (`src/moves.rs`), in this order. On is green, Reduced yellow. Inkling offers On / Off only (the roller has no variant). Kazuya, Ike, Chrom, Roy and Ganondorf are not covered: no row, requests pass through untouched. |
| Group headers | — | One header per fighter with variants; not selectable, Up/Down skip it. |
| Falcon Punch, PK Thunder, PK Fire, … Hammer | `Original`, `Variant 1` .. `Variant N` | One value row per move under its fighter's header, showing the selected variant (the `variant.<move>` value); Left/Right change it in place. It takes effect at once while the fighter's row is Reduced; Original leaves that move untouched. Needs Deluxe 0.5.3 (plugin API v4). |

## Documents

- [Measurement harness](docs/HARNESS.md): the acceptance rule, the steps per
  move, `variant.*`, `cull=` and the `TRACE_EFFECTS` trace.
- [Character effects](docs/FX.md): who owns a request, coverage, the hooks.
- [Configuration](docs/CONFIGURATION.md): INI, scope and online detection,
  markers, diagnostics and event codes.
- [Performance method](docs/FX_PERFORMANCE_METHOD.md) and the
  [emulator results](docs/FX_EMULATOR_RESULTS.md) and
  [runtime matrix](docs/FX_RUNTIME_MATRIX.md): records of Online Deluxe
  builds, except the results' quota section, which ran FX Control 0.1.0.
- [Provenance](reference/README.md).

## Build

`tools/Build.ps1` runs `cargo skyline build --release`; `-Package` archives
the NRO, its ELF and a manifest with `tools/package.py`. Host tests:
`cargo test`; lint: `cargo clippy --all-targets -- -D warnings`.
