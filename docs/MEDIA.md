# Media and testing notes

The examples show what the controls change. They are archived captures,
separate from the operator's current testing report.

<a id="files"></a>

## Examples

| Example | What to look for |
| --- | --- |
| [Fighter comparison](media/palutena-autoreticle-on-off.webp) | Palutena's cyan staff glow disappears in Off; targeting and hits remain. |
| [Stage comparison](media/smashville-on-off.jpg) | Smashville's village scenery disappears in Off; the sky and playable platform remain. |
| [Menu demo](media/fx-control-edit.gif) | Captain Falcon's row cycles On → Off → Reduced → On. |

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

## Operator-reported testing

On **2026-10-25 UTC**, the operator reported FX Control and Online Deluxe
tested in **Eden and on an Erista Switch without issues observed**. No build,
hash or scenario list was supplied.

The README links firmware **23.0.1**, Atmosphère **1.12.1** and Skyline
runtime beta **`2eb226f`**, checked against upstream releases on that date.
Compatibility is the operator's report, not an additional exact-build test.

No [Reduced variant](FX.md#moves-and-variants) is accepted as a performance
improvement; [stage phase and performance checks](STAGES.md#remaining-runtime-acceptance)
remain open. Appearance changes and these captures do not establish FPS gains.

[Hashes, identities and methods](MEDIA_REFERENCE.md) · [Measurement protocol](HARNESS.md)
