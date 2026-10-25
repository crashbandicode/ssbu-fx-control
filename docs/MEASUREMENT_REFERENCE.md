# Measurement reference

Full detail behind the [harness](HARNESS.md) (current protocol) and the
[performance method](FX_PERFORMANCE_METHOD.md) (historical emulator method
and trace reading). Event encodings are in the
[diagnostic event reference](DIAGNOSTICS_REFERENCE.md).

## Move data

`src/moves.rs` lists each move under study with:

| Field | Meaning |
| --- | --- |
| `layers` | Every character-prefixed parent request its scripts make at the pinned dump (`moves::PIN`, WuBoytH/SSBU-Dumped-Scripts), plus the `sys_*` requests they share (never culled) |
| `variants` | Numbered cull sets 1..N. Variant *n* nulls exactly the layers in its set when the fighter's row is **Reduced** and `variant.<move>=n` (default 1). `variant.<move>=0` leaves the move original while the row stays Reduced for the fighter's other moves |
| `status` | Per variant: `Candidate` until a console capture accepts or rejects it |

Candidates run from the lightest cut to the heaviest. The pick is the
lowest-numbered variant that passes the acceptance rule. The 0.1.x cull sets
survive as candidates where a ledger move had one.

## Steps in detail

1. **Layers.** Read the move's fighter and weapon scripts at `PIN` and list
   the `EFFECT*` request names (`layers`) and the `sys_*` ones (`shared`).
   `EFFECT_OFF_KIND` / `EFFECT_DETACH_KIND` names are not requests. When the
   scripts are incomplete or the move is not in the dump, run the trace
   (`TRACE_EFFECTS`) once on the console or in Eden and take the names from
   the log.
2. **Candidates.** Write variants 1..N: first the layers that are pure
   decoration during the move (trails, illusions, secondary fires), then the
   bodies, keeping at least the cue that tells the opponent what is coming
   (the throw, the explosion, the projectile). A variant that nulls every
   layer is Off, not Reduced; the tests refuse it.
3. **Console passes.** Set the fighter's row to Reduced and the move's
   variant row (Deluxe page, or `variant.<move>=n` in the INI), capture,
   analyse; repeat per variant without leaving Training: the row applies at
   once. Deluxe 0.5.3 needs **ZR held** with Left/Right for any edit in its
   window in Training, and 0.5.6 or later needs it in every scene (a plain
   press is ignored; ZL + D-pad swaps the controlled fighter in Training).
   Where a candidate is close, bisect with `cull=` before adding a new
   variant.
4. **Record.** A ledger row per move and variant; `status` in `moves.rs`
   follows the ledger; a VALIDATION-style entry in the Deluxe repository
   (`docs/VALIDATION.md`).
5. **Media.** On / accepted-variant GIF pair for the README, taken in Eden.

## Harness switches

| Switch | Where | What it does |
| --- | --- | --- |
| `variant.<move>=N` | `config.ini`, or the move's row on the Deluxe page | Selects the Reduced variant per move (0 = original). Written by the plugin only when not the default. |
| `cull=<name>,<name>` | `config.ini` | Up to 16 extra request names (with a covered fighter's prefix) nulled while that fighter's row is Reduced and the scope allows. Bisection tool: null one more layer without editing the tables. Read at start; never rewritten. |
| `TRACE_EFFECTS` | Marker file in `sd:/ultimate/ssbu_fx_control/` | Latched at start. Records each fighter request the first time it is seen (code 110, up to 512 keys per boot), plus stage particle code 114 and scenery-group code 116. `tools/resolve_trace.py events.log` prints the names per actor kind. |
| `TRACE_STAGE_DIAGNOSTICS` | Marker file in the same directory | Latched once when stage hooks install. Codes 117, 118 and 119. Independent of `TRACE_EFFECTS` and of stage On/Off. Absent file keeps the previous stage hot path. |

The trace reads the actor's kind, motion and status for every request while
the marker is present (uncovered fighters included), so keep `TRACE_EFFECTS`
absent from measurement captures.

Why the console and not the emulator: the Switch's LLUltra budget is CPU
update plus GPU time inside one vblank, and a move's cost there is what the
rule is about. Eden under a CPU quota ranks candidates cheaply (the 0.1.x
120 % quota table in [emulator results](FX_EMULATOR_RESULTS.md)) but never
accepts one.

## Historical emulator method

Written for Online Deluxe's single FX row; it produced the 2026-10-08
records in [emulator results](FX_EMULATOR_RESULTS.md). It never accepts a
variant. ssbu-fx-control has no timing capture of its own; the traces came
from Online Deluxe's `CAPTURE_TIMING` installed alongside it, or from another
recorded frame-time source.

The target was to approach **Off mode's frame-time stability under LessLag
and LLUltra** while keeping a recognizable cue. Looking less busy is not
sufficient. Removing a decoration is useful only if it removes enough of the
expensive work.

### Matched comparisons

- **Record** the exact NRO hash, game version, emulator build, CPU/GPU,
  driver, graphics settings, render profile, FX scope, stage, fighters and
  inputs.
- **Fix the host:** one emulator instance, 100% game speed, no FPS++, no
  concurrent builds; the same mods and settings across On, Off and Reduced.
- **Fix the scene:** warm the move and stage shaders first. Save a Training
  state with fixed position, facing, damage and charge, and reload it before
  each trial. Use the same recorded input, including the full hold for
  Gigaflare; do not compare different charge levels. Close menus and take
  screenshots outside timing windows when possible.
- **Repeat:** counterbalanced On, Off and Reduced blocks for both LessLag and
  LLUltra, at least five repetitions per condition when a spike can be
  reproduced. Keep before/during/after intervals, not just idle FPS.
- **Confirm** from diagnostic events that scope and profile were actually
  applied and that Reduced used its intended path. Check move timing,
  damage/hits, visibility and post-effect cleanup separately.

### Critical-zoom reproduction

The operator clarified that the spike may occur during
[Special Zoom](https://www.ssbwiki.com/Special_Zoom), not only on the red
final-stock finishing screen. Ordinary attack captures do not establish
critical-zoom coverage.

- Verify an actual connecting move with its blue Special Zoom; keep the red
  finishing-hit screen as a separate fixture. Keep a screenshot outside the
  measured trials to identify which scene actually ran.
- Keep the same stage, positions, damage, move and charge in On, Off and
  Reduced under both LessLag and LLUltra. Warm the critical-zoom shaders as
  well as the move shaders.
- Keep intentional hitstop, camera timing and cinematic slow motion
  unchanged. Measure presentation cadence around the hit; a frozen animation
  during hitstop is not itself a dropped frame.

The normal FX policy preserves critical-zoom backgrounds and `bg_finishhit`,
so normal Off is not a comparator that removes those backgrounds. If all
three modes spike there, separate the move effects from the shared critical
presentation before changing policy. This measurement implies no removal of
camera logic, hitstop, gameplay state or simulation timing.

**`PROBE_CRITICAL_SCREEN`** (diagnostic-only startup marker in
`sd:/ultimate/ssbu_fx_control/`) allows the exact `bg_criticalhit` /
`sys_bg_criticalhit` names and their eight color variants to be suppressed
while every fighter row is Off (All = Off) and the scope allows it. These
backgrounds belong to no fighter. It does not suppress the camera, hitstop,
slow motion, red finishing backgrounds or unrelated screen effects; On and
Reduced ignore it. Keep it absent from user installations. Compare normal Off
with diagnostic Off using the same binary and fresh emulator processes, then
repeat the original condition to check drift. Its events 96 and 97 are in the
[diagnostic event reference](DIAGNOSTICS_REFERENCE.md#critical-screen-probe-96-97);
a missing event or an unchanged screenshot does not prove suppression.

### Decision

| Observation | Meaning |
| --- | --- |
| On has no reproducible spike | Performance unverified. Do not claim Reduced fixes a Switch dip because the emulator stayed at 60 |
| Off has the same spike | The target effect may not be the bottleneck, the filter may not have reached the real request, or another cost dominates. Resolve that before tuning Reduced |
| Off improves, Reduced does not | Reduced fails the performance goal. Keep enough of the expensive body to recognize the move, but cut its particles, lifetime, shader/overdraw work or other demonstrated cost further |
| Reduced approaches Off across repeated matched trials while preserving the cue | A candidate for the console rule in [HARNESS](HARNESS.md), which alone accepts a variant; an emulator result never does |

If emulator settings are changed to reproduce a dip, keep both the normal
setting result and the reproduction settings. Prefer accurate rendering and a
stage/scene that reproduces the actual move cost. Host contention, arbitrary
slow motion and a generally sub-60 baseline are not substitutes for
reproducing the Switch effect-specific bottleneck. The Switch comparison
remains required for a Switch-performance claim. No overclocking behavior is
added.

## Reading frame-time traces

These rules apply to any frame-time trace, historical or current.

**What is observed.** Online Deluxe's bounded `CAPTURE_TIMING` trace records
CPU-side guest present calls at 19.2 MHz. Consecutive present-entry intervals
can expose a game-side cadence spike. They are **not** GPU completion, host
display presentation, scanout or physical input latency. The render-callback
series is a separate observation and must not be mislabeled as present
timing.

**Per-frame, not averages.** Use actual per-frame intervals when available.
Eden's smoothed status FPS can corroborate a sustained drop; 1000/FPS is an
effective period, not a recovered individual-frame time series. A mean near
60 does not prove that every frame met its deadline.

**What to report per condition:** median, p95, p99 and worst interval;
intervals over 16.67/20/25/33.33 ms; the longest consecutive run of slow
intervals; total excess time above the frame budget during the move; capture
completeness, dropped-event counts and observation boundaries. Review raw
plots and repeated trials instead of treating one noisy maximum as decisive.

**Retained benefit.** When On has a repeatable effect-specific cost above
Off, also report the fraction of that improvement Reduced keeps, from each
matched block's excess time:

```text
retained_benefit = (excess_On - excess_Reduced) / (excess_On - excess_Off)
```

Near 1 approaches Off; 0 keeps none of Off's measured benefit. Do not clamp
negative or above-1 results, and do not compute the ratio when the
denominator is indistinguishable from run-to-run noise. Report the underlying
times and variation alongside it. A good ratio does not excuse missing cues
or remaining severe spikes; review both tail intervals and the move visually.

## Capacity-limited diagnostic traces

A trace ending with `reason=capacity` cannot support a full
requested-duration claim. In the c0 Palutena follow-up, the complete
ten-second callback trace provides the guest input anchor. An API slice is
usable only when its actual ordered, gap-free coverage independently includes
the entire baseline and move window plus a predeclared margin (B+3 seconds
for the B..B+2.5-second window). Keep the original capacity reason and
skipped-event counts. Do not relabel the file as elapsed or bypass the
full-trace analyzer's rejection.

Report such observations as bounded CPU-side API intervals. They do not
measure GPU completion, displayed frames or input-to-photon latency. If the
coverage check fails, exclude the trial and keep the failed receipt.
