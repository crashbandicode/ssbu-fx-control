# Reduced FX emulator measurements

**Historical records.** Every artifact below except the 2026-10-15 quota
section is an **Online Deluxe** NRO with the FX feature built in and one
global FX row. The request policy is the one ssbu-fx-control ports, but only
the quota section used a ssbu-fx-control build (0.1.0). None is a Switch
measurement or accepts a Reduced variant ([HARNESS](HARNESS.md) does), and
none is the current build-less
[operator-reported testing](MEDIA.md#operator-reported-testing). Archive
paths refer to the operator's machine.

Related: [method](FX_PERFORMANCE_METHOD.md) ·
[runtime matrix](FX_RUNTIME_MATRIX.md) · [source review](FX_REVIEW.md).

## Findings at a glance

| Campaign | Finding |
| --- | --- |
| [Explosive Flame, 1x](#explosive-flame-at-normal-resolution-2026-10-08) — Deluxe `833c`, 30 measured trials | No repeatable On-over-Off penalty; Reduced benefit unverified |
| [Visual and diagnostic checks](#additional-visual-and-diagnostic-checks) — Deluxe `833c`, visuals, traces, 6 pilots | Reduced cues identifiable, cleanup normal; Inkling Off name mismatch found |
| [Palutena, 2x](#higher-resolution-palutena-pilot-24e) — Deluxe `24e`, 6 pilots | No clear On-over-Off cost |
| [Gigaflare, 2x](#higher-resolution-gigaflare-pilot-24e) — Deluxe `24e`, 6 pilots | No clear On-over-Off cost |
| [Blue Special Zoom](#blue-special-zoom-2026-10-08) — Deluxe `7c`, 30 complete trials | No repeatable critical-zoom spike |
| [Peer report follow-up](#follow-up-to-the-peer-s-lesslag-llultra-report-c0-october-8) — Deluxe `c0`, 36 measured trials | No consistent On-specific slowdown |
| [120 % CPU quota](#per-move-frame-drops-under-a-120-cpu-quota-2026-10-15) — FX Control 0.1.0, 27 single 30 s rows | Off had fewer slow intervals in 25 of 27 rows; not a Switch measurement |

## Explosive Flame at normal resolution — 2026-10-08

**Result.** This configuration did not establish a repeatable On-over-Off
performance penalty. Reduced's performance goal remains **unverified**; no
percentage of Off's benefit is claimed. Visual checks separately confirmed an
intact flame burst in Reduced, suppression in Off, and normal cleanup.

```text
NRO SHA-256: 833cc40bb7db272108684f96972f46103a3f876470dff9e1ecee974776c0f53d
Eden SHA-256: 0b41c3c0c78fccd53615895abf86a2fc3cdaa51bcf8b42cc7d4df0b732e6fc23
Game: 13.0.5
CPU: Intel Core Ultra 7 258V
Graphics: Intel/ANV, Vulkan, High accuracy, Immediate host presentation
Scale: 1×; guest internal resolution 1080p
Speed: Standard, 100%; FPS++ unapplied; DRS off
```

| Setup | Detail |
| --- | --- |
| Host | One Eden instance ran the complete series |
| Scene | Training, Palutena against an idle Greninja; a saved grounded position placed the leftward far flame onscreen between the fighters. It hit neither fighter |
| Other mods | Training Modpack, the corrected gamepad overlay, G&W fix, Elite Legal Stages and standalone ImGui remained installed |
| Trials | Per profile, one warmup per FX mode, then five trials per mode; condition order counterbalanced in five blocks. All 36 ten-second captures completed without truncation; the 30 measured trials (excluding the six predeclared warmups) passed metadata checks |
| Checks | Before/after snapshots kept the committed profile and resolution unchanged; shader polling reported zero. The policy log calls LLUltra `ultra`; its older `mode` field records the startup mode and is not used as the live-policy check |
| Window | Starts at the first guest sample containing B, lasts 2.5 guest seconds. B plus left-stick input was received for approximately 400 ms. The preceding 0.75 seconds is a local baseline |
| Hygiene | No screenshots, pause, menu edits or setting changes inside a capture. CLI status polling ran every 0.5 seconds under every condition |

**Median of five per-trial p99 intervals**, in milliseconds. These are two
different CPU observations; neither measures displayed frames.

| Profile | Observation | On | Reduced | Off |
|---|---|---:|---:|---:|
| LessLag | Render callback | 17.47 | 17.76 | 17.60 |
| LessLag | Guest present-call starts | 18.79 | 19.31 | 19.84 |
| LLUltra | Render callback | 18.17 | 18.38 | 18.50 |
| LLUltra | Guest present-call starts | 18.24 | 18.62 | 18.59 |

- The distributions overlap, including their move-window excess times.
  Occasional slow intervals also appeared in Off. A single maximum or a
  near-60 average does not isolate the effect's cost.
- The analysis helper's noise rule is descriptive, not a significance test;
  the conclusion also follows from the raw repeated trials. No trial was
  discarded merely because its baseline exceeded 16.67 ms.
- A controlled higher-resolution stress run can test whether the filter
  removes measurable work, but it would be synthetic stress, not a
  reproduction of the first-generation Switch's bottleneck. Switch
  performance and other effect families remain separate acceptance work.

Evidence (raw captures, per-trial settings/status snapshots, input receipts,
interval tables, scripts and SHA-256 manifest):
`/home/intpa/.local/share/ssbu-mod-backups/fx-palu-833c-20261008/`

## Additional visual and diagnostic checks

Same 833c session. No timing claim uses these screenshots.

- **Coverage.** Full Gigaflare On/Off/Reduced, Shadow Flare
  charge/projectile cues, Chrom's first side-B trail, Inkling's running roller
  and Ike's full eruption. Reduced kept identifiable visuals, and effects
  expired after the move. Shadow Flare and roller cleanup also survived
  switching to On while effects were active. Chrom's very short trail was
  inspected at 20% speed, with 100% restored afterward.
- **Ike.** The finite decision trace recorded `ike_volcano_add4` and
  `ike_volcano_add2` as Reduced culls, keeping the hold, volcano and
  middle-body requests. On, Reduced and Off casts all produced the observed
  10% self-damage and 39% target damage. This is a narrow observation, not a
  simulation checksum.
- **Gigaflare pilots.** Six unpaused full-Gigaflare pilot captures (one
  On/Off/Reduced per profile) completed their ten-second windows at 1x. They
  do not establish a repeatable On-versus-Off deficit or a Reduced
  performance benefit. The remaining normal eruption bodies can still be
  substantial; counting removed emitters is not a measurement of cost.
- **Inkling.** Off still showed the original roller splash because its legacy
  name did not match the three actual request names. That correction needs
  its own new-build runtime check. Earlier leftward roller attempts ran off
  the Training platform and are excluded from cleanup acceptance.

Evidence (raw evidence, scripts and hashes):
`/home/intpa/.local/share/ssbu-mod-backups/fx-lifecycle-833c-20261008/`

## Higher-resolution Palutena pilot — 24e

| Setup | Detail |
| --- | --- |
| Host | 2x host resolution on the same Intel/ANV machine |
| Scene | Training, idle Joker; a saved wide grounded arrangement with the far flame fully visible between the fighters, missing the CPU. Earlier offscreen fixture attempts were rejected before measurement |
| Trials | Six excluded warmups and six pilots: one On/Off/Reduced pilot under each of LessLag and LLUltra |
| Checks | Each ten-second trace completed without truncation, with zero sampled shader compilation. Game-speed and graphics settings remained fixed |

Move-window guest present-call p99:

| Profile | On | Reduced | Off |
| --- | ---: | ---: | ---: |
| LessLag | 18.70 ms | 20.15 ms | 18.67 ms |
| LLUltra | 18.45 ms | 18.05 ms | 18.52 ms |

This one-trial-per-condition pilot did not reproduce a clear On-over-Off
cost. It is neither a repeated-trial performance pass nor evidence that
Reduced regresses, and no benefit ratio is claimed. The 2x scene is synthetic
stress and differs from the earlier 1x fixture; it is not a controlled
resolution A/B.

## Higher-resolution Gigaflare pilot — 24e

Full Gigaflare was visually confirmed before the 2x series, with the saved
Sephiroth-versus-idle-Falcon fixture. Six warmups and six pilots completed
across both profiles, one pilot per FX mode per profile. The 7.5-second
B-anchored move windows all passed metadata checks without trace truncation.

| Profile | Present-call p99 On | Reduced | Off |
| --- | ---: | ---: | ---: |
| LessLag | 19.30 ms | 19.35 ms | 19.81 ms |
| LLUltra | 18.65 ms | 19.09 ms | 19.40 ms |

These pilots again did not show a clear On-over-Off cost and establish no
Reduced performance benefit. Do not read the ordering of single trials as a
reliable ranking. The captures observe CPU-side cadence, not GPU completion.

Evidence for both 24e pilots (raw pilots, settings, receipts, analyzer and
SHA-256 manifest):
`/home/intpa/.local/share/ssbu-mod-backups/fx-stress-24e-20261008/`

## Blue Special Zoom, 2026-10-08

**Result. No repeatable critical-zoom lag spike was reproduced in this
fixture.**

| Setup | Detail |
| --- | --- |
| Build | Candidate `7c43572918cdc473` |
| Scene | Captain Falcon's connecting neutral-B Special Zoom against idle Fox on the Training stage (31.5% / one hit). On, Off and Reduced screenshots show the critical presentation |
| Host | A fresh single Eden process: Vulkan/ANV, accurate GPU, 1x, 100% speed, FPS++ Off, the same full mod stack. Its 9 GiB/no-swap cgroup recorded no limit or OOM events. `PROBE_CRITICAL_SCREEN` was absent |
| Trials | After warming all three modes, five counterbalanced trials per FX mode and render profile: **30 complete captures** with no shader compilation, trace gaps or menu visibility |
| Observation | Guest CPU present-entry cadence, not GPU completion or displayed frames |
| Window | B-button receipt + 0.65–1.80 seconds, anchored from the raw timing CSV; it approximates the observed zoom, not a logged background-request lifetime. Host-time input leases differ by about one game frame between trials |

| Profile | FX | Median trial impact p99 | Worst impact interval | Impact intervals >25 ms |
| --- | --- | --- | --- | --- |
| LessLag | On | 18.97 ms | 19.64 ms | 0 / 345 |
| LessLag | Off | 18.91 ms | 20.85 ms | 0 / 345 |
| LessLag | Reduced | 19.06 ms | 24.01 ms | 0 / 345 |
| LLUltra | On | 18.62 ms | 20.04 ms | 0 / 343 |
| LLUltra | Off | 18.60 ms | 19.16 ms | 0 / 343 |
| LLUltra | Reduced | 18.38 ms | 19.28 ms | 0 / 343 |

- A preceding six-trial pilot had one 34.12 ms LessLag-On interval; the five
  subsequent On repetitions did not reproduce that impact hitch. Occasional
  longer intervals outside impact remain in the raw data, including a
  35.47 ms LessLag-Reduced recovery interval. They are not discarded or
  attributed to zoom without evidence.
- All modes kept the blue background. Code 96 recorded no allowlisted
  critical-background requests through the installed boundaries, so the
  diagnostic suppression path was not runtime validated. This is **not** a
  background-present/background-absent comparison and does not establish zero
  GPU cost, a Reduced benefit, or Switch performance. No zoom appearance
  change was accepted. Red final-stock Finish Zoom is a different, untested
  fixture.
- The earlier 24e pilot had baseline variation and a CPU-heavy watcher; it is
  exploratory only. The six 7c pilot CSV copies initially raced file export,
  were refreshed from finalized originals and are recorded in
  `completion-refresh.json`; the repeated driver waits for both completion
  markers before copying.

Evidence: raw `RESULTS.json`, all 30 API/timing CSV pairs, status/config
snapshots and the bounded driver, under the critical-zoom evidence directory
in `~/.local/share/ssbu-mod-backups/`.

## Follow-up to the peer's LessLag/LLUltra report — c0, October 8

**Result.** The user clarified that “with the mod” in the peer's report meant
LessLag or LLUltra. The claimed CPU hitbox cause remains a hypothesis. This
follow-up did not reproduce a consistent On-specific slowdown on this host,
and it does not establish an Off or Reduced performance benefit.

~~~text
NRO SHA-256:
c0e9a625eb44a7359545cb3264b2e7e58d45cf15ea7c7d79ba721fbb5be8dce7
Eden SHA-256:
0b41c3c0c78fccd53615895abf86a2fc3cdaa51bcf8b42cc7d4df0b732e6fc23
Raw evidence and per-file hashes:
/home/intpa/.local/share/ssbu-mod-backups/c0-all-runtime-20261008/evidence.zip
Summary inside archive:
c0-all-a/palu-36-measured-summary.json
~~~

| Setup | Detail |
| --- | --- |
| Host | The same Intel Core Ultra 7 258V / Vulkan ANV host, one Eden instance: Smash 13.0.5, High GPU accuracy, 1x/1080p, 100% speed, inactive DRS, FPS++ Off. The embedded Deluxe window was hidden during capture. No sampled shader builds during the measured windows |
| Mods | TMP, corrected SGPO, ARCropolis and the fixed G&W plugin present; standalone ImGui, Elite and the peer's separate effect-removal assets absent |
| Scene | Training, stopped Ivysaur. A saved **hit** fixture connected Explosive Flame for 18.2% / seven hits; a separate **whiff** fixture kept the visible explosion between the fighters at zero damage |
| Trials | Each fixture covered Vanilla, LessLag and LLUltra crossed with On, Off and Reduced: 54 ten-second callback captures, 18 excluded warmups and **36 measured trials**, two per cell. The second repetition reversed FX order |
| Checks | Mode/config snapshots, actual render-wait skips and the guest B sample. Input leases were timed by the host, not frame-exact TAS. Screenshots were taken outside measured windows |
| Window | Guest B receipt through B+2.5 seconds, with a consistent B-0.8-second pre-move baseline |

**Worst present-entry interval across two measured trials**, in
milliseconds; descriptive, not a ranking or significance test.

| Fixture | Profile | On | Off | Reduced |
| --- | --- | ---: | ---: | ---: |
| Hit | Vanilla | 19.690 | 19.009 | 18.913 |
| Hit | LessLag | 22.103 | 19.455 | 18.920 |
| Hit | LLUltra | 20.581 | 19.338 | 18.907 |
| Whiff | Vanilla | 19.904 | 20.423 | 19.080 |
| Whiff | LessLag | 18.460 | 18.670 | 18.898 |
| Whiff | LLUltra | 21.909 | 25.064 | 25.113 |

- Hit trials contained 2,687 move-window present intervals, none over 25 ms.
  Whiff trials contained 2,684, with two just over 25 ms: one LLUltra-Off and
  one LLUltra-Reduced. Across both fixtures, all 5,386 callback intervals were
  below 25 ms. Neither the isolated maxima nor near-60 cadence proves every
  displayed frame met its deadline.
- **API trace capacity.** The extra c0 diagnostics filled the API trace's
  8,192-event buffer after roughly 4.4–5.8 seconds. Every accepted API file
  keeps its actual `reason=capacity` metadata, and coverage through at least
  B+3 seconds was required individually
  ([rule](MEASUREMENT_REFERENCE.md#capacity-limited-diagnostic-traces)). Only
  the fully covered move/baseline slices are reported. The callback files
  completed all ten seconds with no skipped samples. The full-duration API
  analyzer's result was withheld; these are not complete ten-second API
  measurements. Earlier hit/hit2 attempts that failed the capacity gate are
  kept as failed evidence and excluded.
- **Review.** Fable 5.1 xhigh independently recomputed all 27 hit windows,
  including warmups, and checked configuration, engine commitment, input
  anchors and API coverage. Its review did not analyze the whiff matrix; the
  root reviewed that completed matrix. The review supports a bounded negative
  result, not a general claim that LessLag cannot slow an emulator.
- **Scope.** Both hit and whiff create and check the attack's hitboxes. The
  hit additionally changes hitlag, damage, reaction and impact visuals;
  positions, facing and camera also differ between fixtures. Comparisons are
  therefore within each fixture only. These CPU-side callback/present
  observations cannot attribute cost to hitbox creation, measure GPU
  completion/scanout or predict Switch performance. No gameplay or hitbox
  edits were made.

Further reproduction needs the peer's emulator/version, hardware, graphics
settings and scene, or a trace from the affected setup; repeating the same
local fixture would not resolve those differences. Switch On/Off/Reduced
measurements remain necessary before claiming Reduced avoids its lag spikes.

<!-- quota-campaign-2026-10-15:begin -->
## Per-move frame drops under a 120 % CPU quota — 2026-10-15

Operator request: measure, under Switch-like constraints, whether Off or
Reduced helps with frame drops, per covered move, and show it in the README
table.

| Setup | Detail |
| --- | --- |
| Host | Eden GIF build on the Yoga (Intel ANV, Vulkan, docked 1×, 100 % speed), isolated profile |
| Quota | Eden's systemd unit under `CPUQuota=120%` with `CPUQuotaPeriodSec=5ms`, the setting that reproduces the console's LessLag character-select dip in Online Deluxe's measurements; at 175 % nothing dips |
| Builds | FX Control 0.1.0 (`505dfecd…`) with Scope All, next to Online Deluxe on its LessLag profile with the Performance view open. Rows through Kazuya ran next to Deluxe 0.5.0 release `d9dd7df2…`; the rest next to the teardown-guard candidate `cc7f07f6…`, whose only behaviour change is the first three seconds after a scene transition, outside every capture window |
| Scene | Training against a stopped Kazuya |
| Capture | Per move and mode: one 30 s Deluxe `CAPTURE_TIMING` window (render-callback intervals, about 1,800 per window) while the move was cast three times, plus an 8 s GIF with the Performance view in frame |
| Metric | Intervals longer than 20 ms, and the longest interval (ms). 27 of 27 rows complete |

| Move | On (> 20 ms / worst) | Off | Reduced | Deluxe build |
| --- | --- | --- | --- | --- |
| `ness-pk-thunder` | 53 / 65 | 51 / 34 | 63 / 32 | release d9dd7df2 |
| `lucas-pk-thunder` | 44 / 28 | 24 / 26 | 43 / 26 | release d9dd7df2 |
| `sephiroth-gigaflare` | 149 / 97 | 74 / 29 | 90 / 35 | release d9dd7df2 |
| `sephiroth-shadow-flare` | 117 / 36 | 67 / 28 | 82 / 31 | release d9dd7df2 |
| `sephiroth-scintilla` | 66 / 33 | 50 / 30 | 55 / 30 | release d9dd7df2 |
| `sephiroth-dash-attack` | 89 / 38 | 62 / 33 | 77 / 32 | release d9dd7df2 |
| `sephiroth-smashes` | 88 / 32 | 146 / 35 | 95 / 32 | release d9dd7df2 |
| `ike-eruption` | 57 / 34 | 49 / 29 | 53 / 28 | release d9dd7df2 |
| `roy-flare-blade` | 62 / 30 | 46 / 35 | 145 / 35 | release d9dd7df2 |
| `chrom-flare-blade` | 92 / 32 | 31 / 36 | 31 / 27 | release d9dd7df2 |
| `chrom-side-b` | 40 / 26 | 26 / 30 | 54 / 28 | release d9dd7df2 |
| `palutena-explosive-flame` | 79 / 82 | 55 / 31 | 90 / 31 | release d9dd7df2 |
| `palutena-jab` | 113 / 32 | 62 / 30 | 58 / 30 | release d9dd7df2 |
| `palutena-autoreticle` | 69 / 30 | 63 / 33 | 53 / 30 | release d9dd7df2 |
| `palutena-warp` | 52 / 28 | 50 / 30 | 41 / 27 | release d9dd7df2 |
| `palutena-tilts` | 65 / 32 | 46 / 25 | 49 / 29 | release d9dd7df2 |
| `palutena-usmash` | 54 / 27 | 41 / 31 | 48 / 27 | release d9dd7df2 |
| `palutena-fsmash` | 98 / 30 | 76 / 31 | 96 / 32 | release d9dd7df2 |
| `palutena-uair` | 66 / 30 | 44 / 29 | 44 / 34 | release d9dd7df2 |
| `kazuya-fsmash` | 66 / 30 | 47 / 26 | 50 / 27 | release d9dd7df2 |
| `falcon-punch` | 78 / 176 | 64 / 35 | 87 / 33 | candidate cc7f07f6 |
| `pyra-moves` | 245 / 118 | 208 / 30 | 245 / 35 | candidate cc7f07f6 |
| `mythra-moves` | 240 / 54 | 159 / 32 | 229 / 33 | candidate cc7f07f6 |
| `byleth-moves` | 133 / 46 | 151 / 35 | 131 / 36 | candidate cc7f07f6 |
| `inkling-roller` | 149 / 45 | 136 / 32 | 102 / 30 | candidate cc7f07f6 |
| `inkling-smashes` | 135 / 32 | 112 / 31 | 112 / 30 | candidate cc7f07f6 |
| `ganondorf-moves` | 64 / 29 | 55 / 30 | 41 / 30 | candidate cc7f07f6 |

**Reading.**

- Off had fewer slow intervals than On in 25 of 27 rows and more in 2.
  Reduced usually sits between the two but not always (Roy's Reduced window
  was the slowest of its row).
- The longest intervals separate more clearly: 5 On windows contain a spike
  of 60 ms or more (Captain Falcon 176 ms, Pyra 118 ms, Gigaflare 97 ms,
  Explosive Flame 82 ms, Ness 65 ms), while no Off window exceeds 36 ms.
- These are single 30 s runs. The baseline under the quota already has 1–3 %
  of intervals over 20 ms with no move at all, cast timing and hit outcomes
  differed between modes, and differences of a few dozen intervals are within
  run-to-run noise.
- The direction is consistent with the effects costing CPU time under a tight
  budget. It is not a Switch measurement and does not establish a per-move
  gain. This campaign called for Switch On/Off/Reduced follow-up with
  `CAPTURE_TIMING`; current variant acceptance follows [HARNESS](HARNESS.md)
  with `CAPTURE_MATCH`.

Evidence: `fx-perf-1015/` in the operator's evidence root (captures with
timing/api CSVs, analysis.json, the -perf GIFs, driver notes).
<!-- quota-campaign-2026-10-15:end -->
