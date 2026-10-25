# Reduced FX performance method

There is **one current acceptance protocol: [HARNESS](HARNESS.md).** This
page summarizes the historical emulator method behind the 2026-10-08
[emulator results](FX_EMULATOR_RESULTS.md) and how to read any frame-time
trace. Full text: [measurement reference](MEASUREMENT_REFERENCE.md#historical-emulator-method).

| Protocol | Metric and decision |
| --- | --- |
| Current [HARNESS](HARNESS.md) | Console render callbacks: no one-second window below 59. The only route to acceptance |
| Historical Online Deluxe / Eden | Interval percentiles, excess time, retained benefit. Never accepts a variant |

ssbu-fx-control has no timing capture; traces come from Online Deluxe's
`CAPTURE_TIMING` or another recorded frame-time source.

## Matched comparisons

The historical target was Off's frame-time stability under LessLag and
LLUltra with a recognizable cue. Trials fixed the binary, host, settings and
mods; warmed shaders; reloaded one saved Training state; replayed the same
input and charge; and ran counterbalanced On, Off and Reduced blocks, at
least five per condition, confirming from diagnostics that the intended path
ran. [Checklist](MEASUREMENT_REFERENCE.md#matched-comparisons).

## Critical-zoom reproduction

Blue Special Zoom and the red finishing screen are separate fixtures. Normal
Off keeps critical backgrounds, so the diagnostic-only
`PROBE_CRITICAL_SCREEN` marker is the comparator.
[Procedure and limits](MEASUREMENT_REFERENCE.md#critical-zoom-reproduction).

## What the measurements mean

- `CAPTURE_TIMING` present-entry intervals are CPU-side cadence, **not** GPU
  completion, display, scanout or input latency. Render callbacks are a
  separate series.
- Use per-frame intervals. Smoothed FPS and a mean near 60 do not prove every
  frame met its deadline.
- Report median, p95, p99 and worst interval; counts over
  16.67/20/25/33.33 ms; the longest slow run; excess time above budget; and
  capture completeness.
- When On has a repeatable cost over Off:

```text
retained_benefit = (excess_On - excess_Reduced) / (excess_On - excess_Off)
```

Near 1 approaches Off. Do not clamp it, and do not compute it when the
denominator is within noise. [Full rules](MEASUREMENT_REFERENCE.md#reading-frame-time-traces).

## Decision

| Emulator observation | Meaning |
| --- | --- |
| On has no reproducible spike | Performance unverified |
| Off has the same spike | The effect may not be the bottleneck; resolve before tuning |
| Off improves, Reduced does not | Reduced fails; cut more of the costly body |
| Reduced approaches Off, cue kept | Candidate for the console rule only |

A Switch-performance claim needs the Switch.
[Decision detail](MEASUREMENT_REFERENCE.md#decision).

## Capacity-limited diagnostic traces

A `reason=capacity` trace supports only slices whose gap-free coverage
includes the whole baseline and move window plus a predeclared margin.
[Rule](MEASUREMENT_REFERENCE.md#capacity-limited-diagnostic-traces).
