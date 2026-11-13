# ssbu-fx-control

> AI-assisted, human-directed and reviewed.

A one-NRO Skyline plugin for **Super Smash Bros. Ultimate 13.0.5** that
turns down visual effects for ten fighters and eleven stages. It only changes
what is drawn: hitboxes, gameplay and the camera stay as they are.

**[Download 0.3.0](https://git.syntaxbyexample.com/intpa/ssbu-fx-control/releases/tag/v0.3.0)**

Tested in **Eden** only. No Switch test of 0.3.0 is recorded, and the plugin
was not installed on the test Switch as of 2026-11-12.
[Testing status](docs/MEDIA.md#testing-status)

## Fighter effects

![Palutena's Autoreticle with effects On above Off: the cyan staff glow disappears while the targeting reticle and hits remain](docs/media/palutena-autoreticle-on-off.webp)

*Palutena Autoreticle, On above Off. Eden, 2026-10-14, FX Control 0.1.0 (`eb627b8`). [Capture notes](docs/MEDIA.md#fighter-comparison)*

Each fighter is **On** (default), **Off** or **Reduced**. Reduced keeps the
cue that shows what a move does; a fighter without a variant offers On and
Off only. **Scope** defaults to **Online**; set it to **All** for offline
matches and Training. Variants are not yet proven to raise FPS.
[Fighters and variants](docs/FX.md)

## Stages

![Smashville On above Off: village scenery disappears while the blue sky, fighters and playable platform remain](docs/media/smashville-on-off.jpg)

*Smashville On above Off. Eden, 2026-10-22, development build `39744aa9…` (before 0.3.0). [Media notes](docs/MEDIA.md#smashville-on-off-comparison)*

Each stage is **On** (default) or **Off**; there is no Reduced. Stage rows
apply online and offline, separately from fighter Scope and All. Reload the
stage after a change. Off hides selected background effects and scenery,
not the whole background, and no FPS gain is claimed.
[Stages](docs/STAGES.md)

## Controls

With [Online Deluxe](https://git.syntaxbyexample.com/intpa/ssbu-online-deluxe-rewrite)
0.5.6 or later (the pages work from 0.5.3):

1. Hold **L+R** and tap **D-pad Down** until **FX Control** or **Stages**
   appears.
2. **Up/Down** selects a row.
3. Hold **ZR** (not ZL) and press **Left/Right** to change it.

**All** sets every fighter row at once. [The pages](docs/MENU.md)

## Install

1. Copy `libssbu_fx_control.nro` to
   `sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/`,
   replacing any older copy, and restart the game.
2. Online Deluxe is optional. Without it, edit
   `sd:/ultimate/ssbu_fx_control/config.ini`, created on first start
   ([configuration](docs/CONFIGURATION.md)).
3. Don't use it with an Online Deluxe older than 0.5.0: those versions
   have character-effect hooks of their own.

## More

[Stage targets](docs/STAGE_TARGETS.md) · [Measurement harness](docs/HARNESS.md) ·
[Performance records](docs/FX_PERFORMANCE_METHOD.md) · [Provenance](reference/README.md)

Build with `cargo skyline build --release` (or `tools/Build.ps1`); the NRO is
`target/aarch64-skyline-switch/release/libssbu_fx_control.nro`. Test with
`cargo test`.

License: the repository has no license file.
