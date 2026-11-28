# Media source reference

Exact capture identities, hashes and transformations for the
[media and testing guide](MEDIA.md). These are archived Eden demonstrations of
development builds; the [testing status](MEDIA.md#testing-status) is separate.
The original menu still is retained. The stage comparison shows gameplay, not
the Stages settings page.

## Switch card evidence

The sources of the [testing status](MEDIA.md#testing-status)'s Switch line,
on the Surface under `/home/intpa/`:

- 2026-10-15: `smash-training-lobby/artifacts/switch-input-display-7.1.2/switch-before-7.1.2.json`,
  a card listing with `libssbu_fx_control.nro` 581632 B, SHA-256 `505dfecd…`
  (the v0.1.0 release asset's hash).
- 2026-11-02 to 2026-11-12: `ssbu-cpu-pilot-install/card-2026-11-02/listing.txt`
  and the plugin readbacks of every `smash-training-lobby/artifacts/card-install-*`
  receipt (2026-11-10 to 2026-11-12) list no `libssbu_fx_control.nro`.

### 2026-10-23 audit

`smash-training-lobby/artifacts/enhancements/console-practice91/trace-control100/ENVIRONMENT-SANITY.md`
is a read-only audit of the card over DBI MTP.

- `libssbu_fx_control.nro` on the card: SHA-256 `8bf2bdb451266feb…`, the
  development build of the [Surface correction review](STAGE_REFERENCE.md#surface-correction-review-2026-10-22).
  The 0.3.0 release manifest records that its runtime source equals that build's
  (`source_inputs_match_tested_stage_candidate`).
- Firmware 23.0.1 from the console's error reports (2026-10-14 to 2026-10-23);
  `atmosphere/package3` and `stratosphere.romfs` byte-identical to Atmosphère
  1.12.1; `exefs/subsdk9` and `main.npdm` byte-identical to Skyline's `beta`
  release (2026-04-12, commit `2eb226fa9e4ab023dc00cb0db10ad4ec50ac4fa3`).
- It is an identity audit, not a test of the plugin.

## Files

| File | Capture |
| --- | --- |
| [Menu still](media/fx-control-window.png) | 380×258. The FX Control page, every row On |
| <video src="https://git.syntaxbyexample.com/intpa/ssbu-fx-control/media/blob/f4c35348b2c251117c00f4cefe6ced4be20be067" aria-label="Menu demo" autoplay muted loop playsinline controls></video> [Open the clip](media/fx-control-edit.mp4) | 1280×720; 7 s, 60 frames/s. A Captain Falcon row edit, the window in the upper left over the main menu |
| [Stage comparison](media/smashville-on-off.jpg) | 800×1004. Separate Smashville On (top) and Off (bottom) captures |
| [Fighter comparison](media/palutena-autoreticle-on-off.webp) | 480×606; 3.5 s, 41 frames. Palutena Autoreticle On (top) and Off (bottom) |

SHA-256:

- `fx-control-window.png`:
  `7dfdef3520dc78925aa8c7a065a6c0176ef29ea54ea7fa33ae6e33a0dc136350`
- `fx-control-edit.mp4`:
  `18b579e706d6188b8b5748ea1542634609a8a47fdfa056be0d62cf8fc0b8e18d`
- `smashville-on-off.jpg`:
  `224f44152fa98d12f122896ac4c74d6daded5698854eb30d56b50ac9e0c24454`

## What the GIF shows

The heading is kept for existing links; the GIF is now the clip. Sampled by
clip time:

- **0 s:** Captain Falcon On, All On.
- **1.3 s:** Captain Falcon Off, All `Unavailable`.
- **3.3 s:** Captain Falcon Reduced, All `Unavailable`.
- **5.5 s:** Captain Falcon On again, All On.

The clip shows **Scope Online**, the shipped default. The menu still shows
**Scope All**, its capture fixture's setting. Both start with every fighter
row On.

They are menu demonstrations only. They show no gameplay, no effect being
reduced and no frame rate, and they are not performance evidence. In the clip
the game's own main menu reacts to the same inputs behind the window: ZR opens
and closes its Dashboard, which dims the menu.

## Menu capture identity

**The clip** was captured on **2026-11-28** in Eden on Ubuntu, Smash
**13.0.5**, in Online Deluxe's re-shoot of its own clips (run
`filmx2-b-rc16-yoga-1`, take `x2-04-fx-control-edit`). The file is
byte-identical to Deluxe's `docs/media/fx-control-edit.mp4`.

- **Online Deluxe:** the released 0.10.8 NRO, build
  `0.10.8-e7720fd64c92-1794800172`, SHA-256
  `30cb25ddea78b2f14ec4da0c4a0306fb314eed7a4cafc3a29d17beca72770c33`.
- **FX Control:** 0.3.1, NRO SHA-256
  `08370081a6d5e4a5e413ad1f5b6abeed962c2b76ce105b97a4658b7cf872bf4b`.
- **Eden:** the operator's test build `20261117-d34ddcc9b733`, AppImage
  SHA-256 `e8dcc67326bdee93f1246e2696a5de36619aa4e5b618ab4329bcaec8af5d95bc`,
  muted, fullscreen at 1x on a private 1920×1080 display.
- **Other plugins loaded:** the operator's test lineup of that day, with
  TrainingRooms 0.2.0-rc.16 and a test-only Horizon-OC stand-in.

**The menu still** was captured on **2026-10-24** in an isolated, muted Eden
profile on Ubuntu (the Surface fixture), Smash **13.0.5**, during Online
Deluxe's 0.7.0 media refresh. The three entries below are the still's.

- **Online Deluxe:** the released 0.7.0 NRO, build
  `0.7.0-9a7f81ce1937-1792670948`, SHA-256
  `2ff456161c52522c6881956d0ad3a5a037a1a3fc74c8757a1e3a7afe49012cae`.
- **FX Control:** a 0.2.1 **development** build,
  `0.2.1-0c0d7e62c9d0-dirty-1792682041`, NRO SHA-256
  `8bf2bdb451266feba0e57a6b6996aceaf334b9cf190add0e8219aa9d8e1efdd8`.
  It was built from a working tree with uncommitted changes on top of
  `0c0d7e6`. It is **not** the clean 0.2.1 release, and it is **not** a 0.3.0
  capture. The still shows the FX Control page only. The rows visible there
  (Scope, All and the first eight fighters) match that page's order.
- **Eden:** the running binary reports commit
  `10bcd2d849843b146a79c61a21df615e1bdcfcde`, SHA-256
  `64cf47519891440f4a5adfe4d3dfeff180fa4d90b61eb424ee1550e2a3fc3004`.

Online Deluxe's
[media provenance](https://git.syntaxbyexample.com/intpa/ssbu-online-deluxe-rewrite/src/branch/main/docs/MEDIA_PROVENANCE.md)
records the still's full session.

## Menu encoding

- **PNG:** byte-identical to Deluxe's `docs/media/fx-control-window.png` at
  Deluxe commit `5164caa`, a source-pixel close-up of the window.
- **Clip:** seconds 0.5 to 7.5 of one continuous 60 frames/s film of the
  whole 1920×1080 display. It is the film's upper-left 1280×720 at its own
  pixels, encoded as H.264. Online Deluxe made the cut; this repository
  carries the same file.
- No upscaling, redrawing or compositing.

The still's crop receipts and review frames were kept outside the repository
in `docs-notes-fx-1025/`, and its original captures in
`deluxe-media-refresh-1024/`; neither folder was found on the Surface, Yoga or
XPS on 2026-11-13. The clip's receipts and review frames are in the
operator's evidence folder `phase23/refilm/film-x2/`; its raw film was deleted
after the cut.

## Smashville On/Off comparison

Two separate **2026-10-22** Eden screenshots, stacked with On above Off. Both
show normal Smashville in Training, Link versus Little Mac. Off removes the
targeted village scenery while retaining the blue sky, fighters and playable
platform. Some native background objects remain visible. Fighter poses, the
moving platform and other animation differ: these are not frame-matched images
or an animation of a live toggle. They establish no FPS improvement.

### Source identity

- **FX Control:** development build `0.2.1-0c0d7e62c9d0-dirty-1792678653`,
  NRO SHA-256
  `39744aa94af57ccb81024f2667cb303690f8eddbf499271a522e132abbf54c5b`.
  This historical predecessor retained Smashville's four sky fills. It is not
  a capture of the published 0.3.0 binary.
- **Online Deluxe:** NRO SHA-256
  `2f5a1d4e4d99c58e0b351fd3af72e4f7e1cf121e017357d130a18fb2b151fc01`.
- **Fixture:** isolated Surface Ubuntu/Eden profile, Smash 13.0.5, normal
  Smashville, phase 1. Eden binary identity is the same as the menu capture above.
- **On source:** `sky-smash-on-motion.png`, SHA-256
  `bdfa01c503888e5d651e43d06e0cabc573831ba4d6562c9eae1dbfc24892c9ec`.
- **Off source:** `sky-smash-off-motion.png`, SHA-256
  `3f3680bd45cbeb3cd8fb8971f45658f36d3d2fb1655fff5470417edc42de782c`.
- Archived settings screenshots, install receipt and the Off/On/Off runtime
  results establish the states. The paired recordings were made separately
  around 14:26 UTC (Off) and 14:28 UTC (On), not simultaneously.

### Composition

Each 1280×720 source is cropped at x 41, y 22 to 1198×674, removing emulator
chrome and black sidebars, then downscaled to 800×450. The unretouched gameplay
images are stacked with 48-pixel state-label bands and an 8-pixel gap. The
366,635-byte JPEG uses quality 92, 4:4:4 chroma and optimized encoding. Only the
labels and surrounding bands are added; no gameplay pixels are redrawn.

The originals and the run's install receipt are on the Surface in
`/home/intpa/fxs1022/` (`runs/sky3974/`, `SKY3974-INSTALL.json`; hashes checked
2026-11-13). The composition receipt and visual review were kept in
`fx-readme-mobile-1025/`, not found on the Surface, Yoga or XPS on 2026-11-13.
See [stage validation](STAGES.md#remaining-runtime-acceptance) for remaining
phase, negative-control and performance checks.

## Fighter comparison

`media/palutena-autoreticle-on-off.webp` shows **Palutena's Autoreticle** with
On above Off. The cyan staff ornament/glow is absent in Off; the targeting
reticle, projectile flashes and hit feedback remain. Both recordings end with
three hits and CPU damage 13.2%. They are separate casts, not identical game
frames, and do not measure performance or establish exhaustive coverage.

### Source identity

- Recorded **2026-10-14** in Eden, Smash **13.0.5**, normal Training grid,
  Palutena versus Kazuya, CPU Stop, Scope All. The FX page and reset screenshots
  confirm the states and zero-damage setup in the archived campaign.
- **FX Control 0.1.0:** build `0.1.0-eb627b89e654-1792012419`, commit
  `eb627b89e654e2fc636e38de58c557d2099c61a7`, module ID
  `ecd9eb519e46f438b8388502b3a24da6bfc860330b33279c65164230f6477814`.
- **Online Deluxe 0.5.0:** build `0.5.0-44830e5d9255-1792012237`, commit
  `44830e5d92558fdc951f53d89a14478884a61aa0`. These are the install-log identities;
  older abbreviated staging IDs in the original brief are superseded.
- **On:** `gif-fx-palutena-autoreticle-on.gif`, SHA-256
  `d63623842b79cfcb853d91edef35a2af1578960bbcded710b8d0b69633c6bfd7`.
- **Off:** `gif-fx-palutena-autoreticle-off.gif`, SHA-256
  `607df40ee70e1e641cd9ad07a9c7d34290ede9cdd10e2b22fc0b4f0f6d953888`.
- Each original is 640×360, 73 frames, 6000 ms; the recorder reported zero
  drops. The capture is historical, not a demonstration of the 0.3.0 binary
  or its current Reduced variants. No source code or runtime was changed to
  produce this documentation example.

### Alignment and encoding

Both clips lower the staff at frame 7 (600 ms), sweep at frame 8 (680 ms),
and acquire the yellow reticle at frame 9 (770 ms). Those visible actions
establish approximate cast alignment without an offset. Later hit timing and
poses differ slightly. Recording start times alone were not used as alignment.

The source interval is **350–3850 ms** in each clip. Full, uncropped 640×360
frames are stacked with 40-pixel state-label bands and an 8-pixel gap, then
downscaled to **480×606**. No gameplay pixels are redrawn or interpolated.
The looping animated WebP uses quality 90, 12 frames/s, 3500 ms; 42 input
frames become 41 encoded frames after identical-frame merging. It is
2,725,836 bytes, smaller than the reviewed GIF candidates, with SHA-256:

`57d13cde4f44ce11b11711821ec3440e04164aa076f589dc891a2a717293dbec`

The original recordings, install logs, settings/reset images and campaign
observations were kept in `fx-gifs-1014/`; the source contact sheets, frame
map, encoding receipts and frame reviews in `fx-fighter-docs-1025/`. Neither
folder was found on the Surface, Yoga or XPS on 2026-11-13, so the source
hashes above can no longer be re-checked.
