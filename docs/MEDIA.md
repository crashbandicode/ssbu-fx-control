# Media and testing notes

Where the README's two images come from, what they show, and the operator's
testing report.

## Operator-reported testing

On 2026-10-25 (UTC) the operator reported that both plugins, FX Control and Online
Deluxe, were tested in Eden and on an Erista Switch without issues observed.

- The report names no build, hash or scenario list, so none is assigned here.
- The README's compatibility line (Switch firmware 23.0.1, Atmosphère 1.12.1,
  Skyline beta `2eb226fa9e4ab023dc00cb0db10ad4ec50ac4fa3`, released
  2026-04-12) gives the latest upstream releases, checked against their release
  pages on 2026-10-25. Compatibility with them is the user's report; it does
  not tie a build to any of them. The Skyline entry is the runtime's `beta`
  release, not a Rust crate version.
- It is not a performance acceptance. No Reduced variant is accepted yet;
  acceptance follows [HARNESS](HARNESS.md) and is recorded per move in
  [Moves and variants](FX.md#moves-and-variants).
- It is separate from the media below, which show the menu only.

## Files

| File | Size | Length | Shows |
| --- | --- | --- | --- |
| [fx-control-window.png](media/fx-control-window.png) | 380×258 | | The FX Control page, every row On |
| [fx-control-edit.gif](media/fx-control-edit.gif) | 380×258 | 8 s, 80 frames | A Captain Falcon row edit |

SHA-256:

- `fx-control-window.png`:
  `7dfdef3520dc78925aa8c7a065a6c0176ef29ea54ea7fa33ae6e33a0dc136350`
- `fx-control-edit.gif`:
  `baba9df75c760a8832acdd6ff75740091703d9b1c14dbf45e9c6c4d6a89f033d`

## What the GIF shows

Sampled by GIF time:

- **0 s:** Captain Falcon On, All On.
- **3.5 s:** Captain Falcon Off, All `Unavailable`.
- **5.5 s:** Captain Falcon Reduced, All `Unavailable`.
- **7.9 s:** Captain Falcon On again, All On.

Both images show **Scope All** and start with every fighter row On. These are
the capture fixture's settings; the shipped default scope is Online.

They are menu demonstrations only. They show no gameplay, no effect being
reduced and no frame rate, and they are not performance evidence.

## Capture identity

Captured on **2026-10-24** in an isolated, muted Eden profile on Ubuntu
(the Surface fixture), Smash **13.0.5**, during Online Deluxe's 0.7.0 media
refresh.

- **Online Deluxe:** the released 0.7.0 NRO, build
  `0.7.0-9a7f81ce1937-1792670948`, SHA-256
  `2ff456161c52522c6881956d0ad3a5a037a1a3fc74c8757a1e3a7afe49012cae`.
- **FX Control:** a 0.2.1 **development** build,
  `0.2.1-0c0d7e62c9d0-dirty-1792682041`, NRO SHA-256
  `8bf2bdb451266feba0e57a6b6996aceaf334b9cf190add0e8219aa9d8e1efdd8`.
  It was built from a working tree with uncommitted changes on top of
  `0c0d7e6`. It is **not** the clean 0.2.1 release, and those uncommitted
  changes are not part of this branch. The rows visible in the images (Scope,
  All and the first eight fighters) match this branch's page order.
- **Eden:** the running binary reports commit
  `10bcd2d849843b146a79c61a21df615e1bdcfcde`, SHA-256
  `64cf47519891440f4a5adfe4d3dfeff180fa4d90b61eb424ee1550e2a3fc3004`.

Online Deluxe's
[media provenance](https://git.syntaxbyexample.com/intpa/ssbu-online-deluxe-rewrite/src/branch/main/docs/MEDIA_PROVENANCE.md)
records the full session.

## Encoding

- **PNG:** byte-identical to Deluxe's `docs/media/fx-control-window.png` at
  Deluxe commit `5164caa`, a source-pixel close-up of the window.
- **GIF:** cropped from Deluxe's 480×340 `docs/media/fx-control-edit.gif`
  (SHA-256 `cd3038e4e5ea0e7a099cf9daf4a9912b85213ea26844a187e41f148321ff2f72`)
  at x 0, y 0, 380×258 source pixels, keeping the full 0–8 s at 10 images/s.
  The tighter crop drops background so the labels read larger on a phone.
  It was re-encoded with a two-pass FFmpeg palette.
- No upscaling, redrawing or compositing.

Crop receipts and review frames are kept outside the repository in the
operator's evidence directory `docs-notes-fx-1025/`; the original captures are
in `deluxe-media-refresh-1024/`.
