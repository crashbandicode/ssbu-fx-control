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

## Status: 0.2.0 (unreleased), measurement first

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
| Blazing End | Pyra | 1 trails and illusion off · 2 + sword flame · 3 everything but throw and return | SpecialS/AirS/Flick/Catch, esword Fly/Rotate/Back/Hit |
| Flame Nova | Pyra | 1 hold aura and trail · 2 + hold sword, ring1 · 3 only sword and explosions | SpecialNStart/Hold/N1/N4 (N2, N3 pending) |
| Prominence Revolt | Pyra | 1 fire2, trail end, smoke, pillar ground · 2 + every sword fire and the pillar impact | SpecialHiStart/Hi, firepillar SpecialHi |
| Specials | Mythra | 1 sword light layers (0.1.x; specials not extracted) | 0.1.x |
| Explosive Flame | Palutena | 1 appear, wand lights, backlight off (0.1.x) | SpecialS, explosiveflame Explode |
| PK Thunder | Ness | 1 launch burst off (0.1.x) | SpecialAirHi, pkthunder Move |
| PK Fire | Ness | 1 hand flash off · 2 + fire pillar | SpecialS/AirS, pkfire Shoot/Pillar |
| PK Thunder | Lucas | 1 launch burst off (0.1.x) | SpecialAirHi, pkthunder Move |
| Hammer | Kirby | 1 charge glow off · 2 + smoke and wind · 3 only arc and max burst | SpecialS*/AirS*, kirby_hammer SpecialS* |
| Specials | Byleth | 1 axe flare 3 and hold 2 (0.1.x; specials not extracted) | 0.1.x |
| Roller | Inkling | EFF sibling keeping the Splash emitter (no cull) | 0.1.x runtime check |
| Gigaflare | Sephiroth | 1 first burst and sign off, overlay skipped (0.1.x) | edge_fire Burst S/M/L |

Rows without a move here (Kazuya, Ike, Roy, Ganondorf) offer On / Off; their
Off tables are unchanged from 0.1.x. Chrom keeps Reduced for its EFF sibling.

## Install

1. Copy `libssbu_fx_control.nro` to
   `sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/`.
2. [ARCropolis](https://github.com/Raytwo/ARCropolis) is needed for the
   Reduced **EFF siblings** (Palutena up air, Shadow Flare, Inkling roller,
   Chrom side B): the plugin registers ARCropolis load callbacks for those four
   `.eff` files. Without ARCropolis those cues stay original in Reduced (never
   Off); everything else works.
3. [Online Deluxe](https://git.syntaxbyexample.com/intpa/ssbu-online-deluxe-rewrite)
   is optional. With Deluxe 0.5.3 or later the plugin gets its own page in
   Deluxe's window (0.2.0 uses the version 4 header and inline rows, which
   Deluxe 0.5.0 to 0.5.2 refuse: no page, effects still work from the INI)
   and uses Deluxe's online tracking. Without Deluxe, edit the INI and the
   plugin tracks online play itself.

Settings live in `sd:/ultimate/ssbu_fx_control/config.ini`, created with the
defaults on first start (everything On, scope Online, variant 1 per move).
See [configuration](docs/CONFIGURATION.md).

Do not run it next to an older Deluxe (0.4.x) with that FX row set to Off or
Reduced: both would filter the same requests, and both register ARCropolis
callbacks for the same four files.

## The Deluxe page

With Online Deluxe 0.5.0 or later, the plugin registers an own page titled
**FX Control** in Deluxe's window cycle (L+R or ZL+ZR, then D-pad Down):
settings, plugins, **FX Control**, performance, hidden. Up/Down selects a row,
Left/Right changes it. In a match the page becomes Deluxe's compact strip.

| Row | Choices | Notes |
| --- | --- | --- |
| Scope | Online / All | Online: online play outside Training. With a Deluxe older than 0.5.0 there is no online signal: the row reads `Unavailable` and Online never applies (effects stay original until `scope=all` is set in the INI). |
| All | On / Off / Reduced | Sets every fighter row. Shows `Unavailable` while the rows differ. |
| Kazuya ... Kirby | On / Off / Reduced | One row per covered fighter. On is green, Reduced yellow. Rows without a variant offer On / Off. |
| Captain Falcon … Sephiroth (group headers) | — | One header per fighter with variants; not selectable, Up/Down skip it. |
| Falcon Punch, Blazing End, … Gigaflare | `Original  Variant 1 .. N` on one line, the selected one bracketed | One inline row per move under its fighter's header (the `variant.<move>` value). Left/Right move the selection; it takes effect at once while the fighter's row is Reduced; Original leaves that move untouched. Needs Deluxe 0.5.3 (plugin API v4). |
| EFF variants | Ready / Pending / Unavailable | Read-only. Ready: a sibling file loaded. Pending: none loaded yet. Unavailable: ARCropolis missing or a file rejected. |

![FX Control page](docs/media/fx-control-page.png)

## Documents

- [Measurement harness](docs/HARNESS.md): the acceptance rule, the steps per
  move, `variant.*`, `cull=` and the `TRACE_EFFECTS` trace.
- [Character effects](docs/FX.md): who owns a request, coverage, the hooks.
- [Configuration](docs/CONFIGURATION.md): INI, scope and online detection,
  markers, diagnostics and event codes.
- [Performance method](docs/FX_PERFORMANCE_METHOD.md) and the
  [emulator results](docs/FX_EMULATOR_RESULTS.md) and
  [runtime matrix](docs/FX_RUNTIME_MATRIX.md) of the 0.1.x line.
- [Provenance](reference/README.md).

## Build

`tools/Build.ps1` runs `cargo skyline build --release`; `-Package` archives
the NRO, its ELF and a manifest with `tools/package.py`. Host tests:
`cargo test`; lint: `cargo clippy --all-targets -- -D warnings`.
