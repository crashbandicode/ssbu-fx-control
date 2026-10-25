# FX runtime acceptance matrix

**Historical records; no row applies to 0.2.0 or later.** Every row was
observed with an **Online Deluxe** build (`833c`, `24e`, `7c`, `b99`, `c0`;
timings in [emulator results](FX_EMULATOR_RESULTS.md)) and its single global
FX row, before the feature moved into this plugin. ssbu-fx-control runs the
same request pipeline and Off tables per fighter; the Reduced keep/cull
tables behind these rows were replaced in 0.2.0 by the per-move variants of
`src/moves.rs`. None of these checks has been repeated with a ssbu-fx-control
NRO yet, and the current
[operator-reported testing](MEDIA.md#operator-reported-testing) names no
build.

Implementation coverage and runtime acceptance are separate. Reduced changes
specific visual layers, not every move a character can perform. Performance
benefits remain subject to the [measurement method](FX_PERFORMANCE_METHOD.md)
and acceptance to [HARNESS](HARNESS.md). The tested artifact and scene
matter; later builds are not automatically retested.

| What happened in 0.2.0 | Rows |
| --- | --- |
| [Keep/cull table replaced by `src/moves.rs` variants](#replaced-by-per-move-variants) | 7 |
| [Cull set with no `src/moves.rs` entry](#no-entry-in-src-moves-rs); Reduced classifies only through `moves::layer`, so it does not apply them | 4 |
| [Code removed](#removed-in-0-2-0); it survives only on the branch `archive/0.1.x-appearance-variants` | 9 |

## Replaced by per-move variants

Open a family for its historical change, visual evidence and remaining check.

<details><summary>Palutena Explosive Flame</summary>

- **Reduced change:** Keep flame body/finish, remove surrounding layers
- **Visual evidence:** 833c On/Off/Reduced and expiry; 24e 2x On fixture
- **Remaining check:** No clear performance gap in 1x repeated or 2x pilot trials

</details>

<details><summary>Sephiroth neutral-B</summary>

- **Reduced change:** Keep burst body, remove secondary body/screen wash
- **Visual evidence:** 833c full Gigaflare, 49.3%/8 hits; 24e Megaflare and 7c small Flare On/Reduced/On, expiry and cull/keep traces
- **Remaining check:** Switch performance

</details>

<details><summary>Mythra sword</summary>

- **Reduced change:** Keep sword/open cues, remove light layers
- **Visual evidence:** 24e tilt On/Reduced/On and expiry, cull/keep trace; neutral-B 41.5%/five hits all modes tested
- **Remaining check:** light_few branch not observed; beams/buster/swap remain original; Switch performance

</details>

<details><summary>Byleth axe</summary>

- **Reduced change:** Keep axe flare/slash, remove extra flare/hold
- **Visual evidence:** 24e On/Off/Reduced, On restoration and expiry; 37.8%/one hit vs DK; cull/keep trace
- **Remaining check:** Switch performance; bow/spear remain original

</details>

<details><summary>Captain Falcon</summary>

- **Reduced change:** Keep punch body/hold, remove named wing auxiliary layers
- **Visual evidence:** 24e On/Reduced/On and expiry; cull/keep trace confirms both wing requests filtered
- **Remaining check:** Main bird silhouette remains; Switch performance; Falcon Kick remains original

</details>

<details><summary>Ness</summary>

- **Reduced change:** Keep steer/projectile and self-launch body, remove bomb layer
- **Visual evidence:** 24e On/Reduced/On actual PK Thunder self-hit, recognizable launch, 31.5%/one hit vs Hero; trace85 attack kept/84 bomb culled, clean expiry
- **Remaining check:** Switch performance

</details>

<details><summary>Lucas</summary>

- **Reduced change:** Keep steer/projectile and self-launch body, remove bomb layer
- **Visual evidence:** 24e actual PK Thunder self-hit On/Reduced/On and expiry; trace85 attack kept/84 bomb culled
- **Remaining check:** Host-time paths not frame matched; Switch performance

</details>

## No entry in src/moves.rs

<details><summary>Palutena other targeted moves</summary>

- **Reduced change:** Selected wand/backlight layers
- **Visual evidence:** 24e jab, neutral/up-B, tilts, three smashes On/Reduced/On and expiry; sampled cull/keep traces
- **Remaining check:** Tilt trace08 and teleport_start not observed; Switch performance

</details>

<details><summary>Sephiroth other targeted moves</summary>

- **Reduced change:** Extra smash/dash/counter layers
- **Visual evidence:** 24e forward smash/dash/untriggered counter; 7c up/down-smash On/Reduced/On; c0 grounded triggered-counter On/Reduced/On retains reactive shield and attack, with clean expiry and cull traces
- **Remaining check:** Down-smash retained names and counter hold_attack not observed; airborne/opposite-facing counter not separately checked; Switch performance

</details>

<details><summary>Inkling smashes</summary>

- **Reduced change:** Keep splash body, remove selected secondary layers
- **Visual evidence:** b99 full-ink down-smash culls slosher_trace and retains splash; verified dry down-smash retains sole splash2; On/Reduced/On and expiry; c0 with shared26w hot-swap gives a recognizable forward-smash splash in On/Reduced, both one hit/23.8%, with clean expiry and splash keep trace
- **Remaining check:** Forward-smash trace2 cull remains unobserved; Switch performance

</details>

<details><summary>Pyra sword (tilt)</summary>

- **Reduced change:** Keep sword/open cues, remove firetrail pair
- **Visual evidence:** 24e tilt On/Reduced/On and expiry, 14.4%/one hit; cull/keep trace; neutral-B body remains
- **Remaining check:** Broad main arc remains, including sampled Off tilt; major specials remain original; Switch performance

</details>

## Removed in 0.2.0

Sibling-based rows (a lighter EFF copy of the resource) are marked *sibling*.

<details><summary>Palutena up-air (sibling)</summary>

- **Reduced change:** Lighter sibling retaining blue wing cue
- **Visual evidence:** 833c On/Reduced/On, cleanup
- **Remaining check:** Switch performance

</details>

<details><summary>Sephiroth side-B (sibling)</summary>

- **Reduced change:** Lighter charge/projectile/orb siblings
- **Visual evidence:** 833c compact orbs and Reduced-to-On cleanup
- **Remaining check:** Switch performance

</details>

<details><summary>Inkling roller (sibling)</summary>

- **Reduced change:** Keep Splash emitter, remove extra emitters
- **Visual evidence:** 833c/24e run and cleanup; b99 RUN/WALK/AIR sibling requests observed, On/Reduced/On recognizable roller cue and expiry
- **Remaining check:** Airborne scene not separately frame-matched; Switch performance

</details>

<details><summary>Chrom first side-B (sibling)</summary>

- **Reduced change:** Trail lifetime 3 to 2 in guarded sibling
- **Visual evidence:** 833c recognizable trail and expiry
- **Remaining check:** Other side-B branches remain original

</details>

<details><summary>Chrom neutral-B</summary>

- **Reduced change:** Keep charge-level body, remove extra hold/spark
- **Visual evidence:** 24e short/full On/Reduced/On, clean expiry; full 12% self/63% target/one hit; cull/keep trace
- **Remaining check:** Switch performance

</details>

<details><summary>Ike neutral-B</summary>

- **Reduced change:** Keep volcano bodies, remove additional layers
- **Visual evidence:** 833c full charge, 10% self/39% target, cull/keep trace
- **Remaining check:** Switch performance

</details>

<details><summary>Roy neutral-B</summary>

- **Reduced change:** Keep eruption body, remove extra start layer
- **Visual evidence:** 24e short/full On/Reduced/On, clean expiry; full 12% self/45.3% target/one hit; cull/keep trace
- **Remaining check:** Switch performance

</details>

<details><summary>Ganondorf</summary>

- **Reduced change:** Keep punch/sword cues, remove flash/trace
- **Visual evidence:** 24e punch and charged sword On/Reduced, expiry, cull/keep trace
- **Remaining check:** Charged damage comparison was not frame-matched; Switch performance

</details>

<details><summary>Kazuya forward smash</summary>

- **Reduced change:** Keep line/aura/start, remove electrical/sign flashes
- **Visual evidence:** 24e charged forward smash On/Reduced/On; trace5077871101534 keeps aura/start/line and culls electric/sign layers; clean expiry
- **Remaining check:** Other smash profiles remain original; Switch performance

</details>

## Reading the evidence

Pausing, slow motion and screenshots verify cues and cleanup, not frame-time
benefits. A listed retained cue must remain identifiable during play, and a
successful menu change is not proof that the intended effect hook ran.
Concurrent-instance captures are visual checks only; paused images do not
establish frame-time gains.

The 24e family captures and request traces are archived with a SHA-256
manifest at `~/.local/share/ssbu-mod-backups/fx-families-24e-20261008/`.
