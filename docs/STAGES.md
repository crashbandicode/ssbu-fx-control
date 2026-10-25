# Stage backgrounds

Each of Elite Legal Stages' eleven normal stages has an **On** (default) or
**Off** row on the **Stages** page (`fx_control_stages`) and a `stage.<key>`
line in [config.ini](CONFIGURATION.md#keys). Elite only supplies the list; this
plugin neither requires nor changes it.

![Smashville On above Off: village scenery disappears while the blue sky, fighters and playable platform remain](media/smashville-on-off.jpg)

*Smashville On (top) and Off (bottom): two separate 2026-10-22 development
captures, not frame-matched and not the 0.3.0 binary.
[Media notes](MEDIA.md#smashville-on-off-comparison)*

[What Off hides](#what-off-hides) · [Using the rows](#using-the-rows) ·
[Remaining runtime acceptance](#remaining-runtime-acceptance) ·
[Target inventory](STAGE_TARGETS.md) · [Internals and evidence](STAGE_REFERENCE.md)

## What Off hides

Off hides 47 named background particles on five stages and 18 selected scenery
model groups on the other six. It is selective, not a removal of the whole
background: playable geometry, stage simulation, unclassified scenery and
contrast-critical backdrops stay native. No stock-clock FPS improvement is
claimed.

| Stage | Off changes |
| --- | --- |
| Battlefield | Hides distant birds, waterfalls/mist, sky city and sun. Keeps landscape and sky dome |
| Small Battlefield | Hides the same four groups. Keeps landscape and sky dome |
| Final Destination | Hides 5 particles: particle, swarm, electricity, two ring auras. Keeps all scenery |
| Yoshi's Story | Hides sea and islands. Keeps `bg2_set` sky/cloud fill and `stg_cartboard_smoke` |
| Lylat Cruise | Hides 5 particles: background battle ship, space 1–3, asteroid. Keeps all scenery |
| Pokemon Stadium 2 | Hides city searchlights. Keeps night sky/city, transformation smoke and monitors |
| Smashville | Hides village, two clock hands and four window-light groups. Keeps all four sky phases |
| Kalos Pokemon League | Hides Great Hall decorative lights. Keeps backdrop, hazard, transformation and Pokemon effects |
| Town and City | Hides 2 particles: background wind and bubbles. Keeps all scenery |
| Northern Cave | Hides 5 particles: Lifestream, Holy/meteor waves, Holy line, Lifestream tentacle. Keeps all scenery |
| Hollow Bastion | Hides 30 particles: waterfall/bubbles, background switches/trolleys, Dive to Heart and stained-glass effects. Keeps all scenery |

Matching is exact: particle name, stage-effect caller and stage ID, or full
model-folder path and stage ID. The [target inventory](STAGE_TARGETS.md) lists
every key, name, hash and preserved example; IDs and guards are in the
[reference](STAGE_REFERENCE.md).

## Using the rows

- **On / Off only.** There is no Reduced. **All backgrounds** sets all eleven
  rows.
- **Separate from fighters.** Fighter All, Reduced and Scope do not affect stage
  rows, and stage rows apply offline and online.
- **Reload the stage for a consistent result.** New requests follow the row
  at once and captured models may change on their next draw, but effects
  already created finish normally and turning On does not bring back missed
  ones. [Details](STAGE_REFERENCE.md#applying-changes)
- **Hook failure.** If a hook fails its install checks, its rows and **All
  backgrounds** become unavailable; the other hook's rows still work.
- **Without Deluxe**, edit the INI. `stages=` applies first, then each
  `stage.<key>` line; Reduced and unknown values are ignored. To hide only
  Final Destination's and Northern Cave's targets:

```ini
stages=on
stage.end=off
stage.ff_cave=off
```

Stage rows are saved by the plugin, not the host
([who writes the file](CONFIGURATION.md#who-writes-the-file)). Page layout:
[MENU](MENU.md#stages-page).

## Remaining runtime acceptance

These controls ship in **0.3.0** and are guarded for Smash **13.0.5** only.
Dated Eden emulator samples show sampled suppression or the intended Off result
on most stages ([summary by stage](STAGE_REFERENCE.md#summary-by-stage)). Lylat
Cruise is not accepted, because its whiteout also occurs without FX. Hollow
Bastion was checked only with modified resources.

Still open for all eleven stages: traced On / Off / On reloads, gameplay and
negative-control checks, every phase and transformation, and untraced
stock-clock performance captures
([checklist](STAGE_REFERENCE.md#acceptance-checklist)). None of this is Switch
evidence; the later [operator testing report](MEDIA.md#operator-reported-testing)
is separate.
