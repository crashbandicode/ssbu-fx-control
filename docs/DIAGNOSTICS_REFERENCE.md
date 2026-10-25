# Diagnostic event reference

Encodings and limits of the trace events the [harness](HARNESS.md) and the
[performance method](FX_PERFORMANCE_METHOD.md) use. Every code here records a
request or a hook call. **None is a rendered frame or a frame-rate figure.**
`tools/resolve_trace.py` prints the codes and encodes them with `--json`; it
lists 117/118/119 beside the 110/114/116 groups and does not merge them.

Markers live in `sd:/ultimate/ssbu_fx_control/`:

- `TRACE_EFFECTS`: codes 110, 114, 116.
- `TRACE_STAGE_DIAGNOSTICS`: codes 117, 118, 119.
- `PROBE_CRITICAL_SCREEN`: codes 96, 97.

| Code | Records |
| --- | --- |
| 110 | First sighting of each fighter request (name, actor category, kind, status) |
| 114 | First sighting of each stage particle request |
| 116 | First sighting of each scenery group |
| 117 | Marker state at stage-hook install |
| 118 | Cumulative model-draw hook calls per stage and reason |
| 119 | First sighting of an allowlisted stage particle name or caller |
| 96 | First 32 matched critical-background requests |
| 97 | Marker state at startup |

## Fighter trace (110, 114, 116)

`TRACE_EFFECTS` is latched at start. Every EffectModule `req*` hook records
the first time it sees each (name, actor category, kind, status) as code 110,
up to 512 keys per boot. The same marker records stage particle code 114 and
scenery-group code 116. Names come from `reference/effect_names.txt`
(regenerate with `tools/export_effect_names.py`; add script names by hand
under `# manual`).

While the marker is present the trace reads the actor's kind, motion and
status for every request, uncovered fighters included. Keep it absent from
measurement captures.

## Stage diagnostics (117, 118, 119)

### Latch (117)

The plugin reads `TRACE_STAGE_DIAGNOSTICS` once, during stage-hook install,
and latches it. Creating or deleting the file in a running session does
nothing until the next process start; there is no per-frame reload. Code 117
is written at that install: `a` = 1 when the file was present, 0 when absent.
Install also appends the reason and flag map to `install.log`.

The marker is independent of `TRACE_EFFECTS` and of stage On/Off; both stage
On and stage Off are observed. With the marker absent, the stage hooks keep
their previous early return and do not read the live stage id for these
counters.

### Model-draw calls (118)

A cumulative count of **observed model-draw hook calls** on the eleven normal
stages only. It is not a rendered frame, a completed pass, or an FPS figure.

| Field | Encoding |
| --- | --- |
| `a` | `(stage_id << 32) \| reason` |
| `b` | Cumulative count |

The background logger scans every 5 seconds and writes changed nonzero
counters, at most 32 per 500 ms wake, so one scan cannot fill the 128-event
ring. A count can therefore reach `events.log` a few seconds after the calls.
Stage ids outside those eleven, including `-1` while no stage is active, are
not stored.

Every call that reaches the registry read gets one **lookup** reason:

| Reason | Meaning |
| --- | --- |
| 1 `found` | The registry lookup matched |
| 2 `unregistered` | No registry entry |
| 3 `ambiguous` | The stored path is 0 |
| 4 `contended` | `try_read` failed on that bucket; the registry is not disabled |
| 5 `disabled` | Null renderer, unhealthy registry, or poisoned lock |

A `found` call also gets exactly one **result** reason:

| Reason | Meaning |
| --- | --- |
| 6 `requested-native-hidden` | This call left the draw-record argument alone |
| 7 `native-visible` | This call left the draw-record argument alone |
| 8 `suppressed` | This call cleared the draw-record argument |
| 9 `wrong-vtable` | The call failed the vtable check |
| 10 `wrong-stage` | The registered path is not a static target for the live stage id |

`found` equals the sum of reasons 6–10. Adding every reason together
double-counts found calls. None of these reasons identify which pixels the
display showed.

### Particle predicate (119)

The first time the existing stage particle hook sees an allowlisted effect
name **or** an allowlisted stage caller, recorded before the conjunction that
decides whether to null the request. The request register is unchanged by
this record.

| Field | Encoding |
| --- | --- |
| `a` | Hash40 |
| `b` | `(signed stage16 << 48) \| (flags16 << 32) \| caller32` |
| Flags | `name-hit` 1, `caller-allowed` 2, `stage-match` 4, `row-off` 8 |

Row-off is part of the key, so On and Off do not collapse into one record; the
key also keeps the caller and the live stage. The table holds 512 keys,
separate from the fighter `TRACE_EFFECTS` table, and is reserved at install
when the marker is present. Only this observer takes that lock, and only with
`try_lock`.

A missing 119 does **not** prove the predicate did not occur:

- repeats are not logged;
- a busy or poisoned dedupe lock skips that call and does not retry;
- after 512 distinct keys, new keys are dropped;
- a request that misses both the name list and the caller allowlist is not
  recorded;
- a return address that is missing or does not fit in `u32` is stored as
  `0xffffffff`, and every such caller collapses to that one sentinel;
- a stage id outside signed 16 bits is not recorded.

Code 119 records a request observation, not a rendered frame.

### What a stage capture can support

| Supports | Does not support |
| --- | --- |
| Which registry outcome dominated on a recognized stage | That those counts caused a flicker or a whiteout |
| Whether a matched target was left native-hidden, left native-visible, or had its draw argument cleared | That a miss happened on an unrecognized stage id |
| Which predicate bit was clear the first time an allowlisted particle name or caller was seen | That every call was logged, or any frame-rate claim |

## Critical-screen probe (96, 97)

`PROBE_CRITICAL_SCREEN` is a diagnostic-only startup marker, described with
its comparison procedure in
[Critical-zoom reproduction](MEASUREMENT_REFERENCE.md#critical-zoom-reproduction).
Code 97 records the latched marker state at startup. Code 96 records the first
32 matched requests (this plugin has no finite capture session to extend
that):

| Field | Encoding |
| --- | --- |
| `a` | Original Hash40 |
| `b` bits 0–7 | Request path: 1 screen, 2 screen-system, 3 ordinary |
| `b` bit 8 | Actual diagnostic suppression |

A missing event or an unchanged screenshot does not prove that the effect was
suppressed.
