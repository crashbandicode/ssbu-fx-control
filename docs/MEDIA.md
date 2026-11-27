# Media and testing notes

The examples show what the controls change. They are archived Eden captures of
development builds, not of the 0.3.0 release.

<a id="files"></a>

## Examples

| Example | What to look for |
| --- | --- |
| [Fighter comparison](media/palutena-autoreticle-on-off.webp) | Palutena's cyan staff glow disappears in Off; targeting and hits remain. |
| [Stage comparison](media/smashville-on-off.jpg) | Smashville's village scenery disappears in Off; the sky and playable platform remain. |
| <video src="https://git.syntaxbyexample.com/intpa/ssbu-fx-control/media/blob/0d536c48034953127e1d3bd8d7f7eff87c266fe9" aria-label="Menu demo" autoplay muted loop playsinline controls></video> [Open the clip](media/fx-control-edit.mp4) | Captain Falcon's row cycles On → Off → Reduced → On. |

## Fighter comparison

Palutena Autoreticle, **On above Off**, captured in Eden on 2026-10-14 with
FX Control **0.1.0** and Deluxe **0.5.0**, Scope All. The two casts are aligned
by visible staff movement and reticle appearance; they are not frame-identical.
This is an On/Off example, not a demonstration of today's Reduced variants.
[Exact capture and encoding](MEDIA_REFERENCE.md#fighter-comparison)

## Smashville On/Off comparison

**On above Off**, from separate 2026-10-22 development-build screenshots.
Some native background objects remain. Animation and platform positions differ.
[Exact capture and composition](MEDIA_REFERENCE.md#smashville-on-off-comparison)

<a id="what-the-gif-shows"></a>
<a id="menu-capture-identity"></a>
<a id="capture-identity"></a>
<a id="menu-encoding"></a>
<a id="encoding"></a>

## Menu demo

The 2026-10-24 clip uses the released Deluxe **0.7.0** and an FX **0.2.1
development build**, with Scope All. The shipped scope default is Online.
All reads `Unavailable` while fighter rows differ. It shows menu editing,
not gameplay or the Stages page. The [original still](media/fx-control-window.png)
is retained. [Source details](MEDIA_REFERENCE.md#menu-capture-identity)

## Testing status

Checked 2026-11-13.

- **Eden:** dated emulator samples only: the stage rows on 2026-10-21/22
  ([summary by stage](STAGE_REFERENCE.md#summary-by-stage)), the menu capture
  above and earlier fighter captures
  ([emulator results](FX_EMULATOR_RESULTS.md)). The 0.1.0, 0.2.1 and 0.3.0
  release manifests all say `runtime_tested: false`.
- **Switch:** FX Control 0.1.0 (`505dfecd…`) was on the card on 2026-10-15,
  when Online Deluxe's console move captures ran with every FX row On
  (Deluxe `docs/VALIDATION.md`, "0.5.2 — 2026-10-15: console move captures");
  those captures chose the [ledger moves](FX.md#moves-and-variants) and tested
  no Off or Reduced setting. A read-only card audit on 2026-10-23 found the
  development build `8bf2bdb4…` on the card (0.3.0 keeps its runtime source), next to firmware
  23.0.1, Atmosphère 1.12.1 files and Skyline `beta` (`2eb226f`) files. No
  test result from that card is recorded here. The card readbacks of
  2026-11-02 to 2026-11-12 list no FX Control NRO, only its settings in
  `ultimate/ssbu_fx_control/`.
- No [Reduced variant](FX.md#moves-and-variants) is accepted, no desync-detector
  test has been run, and the
  [stage phase and performance checks](STAGES.md#remaining-runtime-acceptance)
  remain open. Appearance changes and these captures do not establish FPS gains.

[Hashes, identities and methods](MEDIA_REFERENCE.md) · [Measurement protocol](HARNESS.md)
