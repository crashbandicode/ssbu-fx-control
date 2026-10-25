# Measurement harness

The 0.2 line of this plugin exists to answer one question per move: **which
effect layers have to go for the console to hold 59 fps while the move
plays, keeping the gameplay cue?** Nothing is decided by appearance. This
document is the protocol; `src/moves.rs` is the data it fills; the per-move
record lives in the operator's evidence root (`MOVE-LEDGER.md`), not here.

## The unit: a move with numbered variants

`src/moves.rs` lists each move under study with

- `layers`: every character-prefixed parent request its scripts make at the
  pinned dump (`moves::PIN`, WuBoytH/SSBU-Dumped-Scripts), and the `sys_*`
  requests they share (never culled);
- `variants`: 1..N cull sets, numbered. Variant *n* nulls exactly the layers
  in its set when the fighter's row is **Reduced** and `variant.<move>=n`
  (default 1). `variant.<move>=0` leaves the move original while the row
  stays Reduced for the fighter's other moves;
- a `status` per variant: `Candidate` until a console capture accepts or
  rejects it.

Candidates are ordered from the lightest cut to the heaviest; the pick is the
lowest-numbered variant that passes the acceptance rule. The 0.1.x cull sets
survive as candidates where a ledger move had one.

## Acceptance rule

Console, product Deluxe build with `CAPTURE_MATCH`, Training against a
stopped CPU, Training Modpack Input Display **off**, the Deluxe window
**hidden** while the move is performed, LLUltra (the only profile that drops;
Vanilla and LessLag held 60 on every move on 2026-10-15). Repeat the move for
about 30 s. PASS = no one-second window below 59 render callbacks
(`tools/frame_windows.py` in the Deluxe repo). The same capture, the same
rule, for On, Off and every variant; the ledger records the worst second and
the frames short of 60 for each.

Why the console and not the emulator: the Switch's LLUltra budget is CPU
update plus GPU time inside one vblank, and a move's cost there is what the
rule is about. Eden under a CPU quota ranks candidates cheaply (the 0.1.x
120 % quota table in [FX_EMULATOR_RESULTS](FX_EMULATOR_RESULTS.md)) but never
accepts one.

## Steps per move

1. **Layers.** Read the move's fighter and weapon scripts at `PIN` and list
   the `EFFECT*` request names (`layers`) and the `sys_*` ones (`shared`).
   `EFFECT_OFF_KIND` / `EFFECT_DETACH_KIND` names are not requests. When the
   scripts are incomplete or the move is not in the dump, run the **trace**
   (below) once on the console or in Eden and take the names from the log.
2. **Candidates.** Write 1..N variants: first the layers that are pure
   decoration during the move (trails, illusions, secondary fires), then the
   bodies, keeping at least the cue that tells the opponent what is coming
   (the throw, the explosion, the projectile). A variant that nulls every
   layer is Off, not Reduced; the tests refuse it.
3. **Console passes.** Set the fighter's row to Reduced and the move's
   variant row (Deluxe page, or `variant.<move>=n` in the INI), capture,
   analyse; repeat per variant without leaving Training: the row applies at
   once. In Training Deluxe 0.5.3 needs **ZR held** with Left/Right for any
   edit in its window, and 0.5.6 or later in every scene (a plain press is
   ignored; ZL + D-pad swaps the
   controlled fighter in Training). Where a candidate is close, bisect with `cull=` (below) before
   adding a new variant.
4. **Record.** Ledger row per move and variant; `status` in `moves.rs`
   follows the ledger; a VALIDATION-style entry in the Deluxe repository
   (`docs/VALIDATION.md`).
5. **Media.** On / accepted-variant GIF pair for the README, taken in Eden.

## Harness switches

| Switch | Where | What it does |
| --- | --- | --- |
| `variant.<move>=N` | `config.ini`, or the move's row on the Deluxe page | Selects the Reduced variant per move (0 = original). Written by the plugin only when not the default. |
| `cull=<name>,<name>` | `config.ini` | Up to 16 extra request names (with a covered fighter's prefix) nulled while that fighter's row is Reduced and the scope allows. Bisection tool: null one more layer without editing the tables. Read at start; never rewritten. |
| `TRACE_EFFECTS` | marker file in `sd:/ultimate/ssbu_fx_control/` | Latched at start. Every EffectModule `req*` hook records the first time it sees each (name, actor category, kind, status) as event code 110, up to 512 keys per boot. The same marker records stage particle code 114 and scenery-group code 116. `tools/resolve_trace.py events.log` prints the names per actor kind (names from `reference/effect_names.txt`; regenerate with `tools/export_effect_names.py`, add script names by hand under `# manual`). |
| `TRACE_STAGE_DIAGNOSTICS` | marker file in the same directory | Latched once when stage hooks install. Codes 117, 118 and 119. Independent of `TRACE_EFFECTS` and of stage On/Off. Absent file keeps the previous stage hot path. |

The trace reads the actor's kind, motion and status for every request while
the marker is present (uncovered fighters included), so keep it absent from
measurement captures.

## Stage diagnostics

Create `sd:/ultimate/ssbu_fx_control/TRACE_STAGE_DIAGNOSTICS` and fully restart
Smash before the match. The plugin reads that path once during stage-hook
install and latches it. Creating or deleting the file during a running
session does nothing until the next process start. There is no per-frame
reload. Code 117 is written at that install with `a` = 1 when the file was
present and `a` = 0 when it was absent. Install also appends the reason and
flag map to `install.log`.

The marker is independent of `TRACE_EFFECTS`. Stage On and stage Off are both
observed. With the marker absent, the stage hooks keep their previous early
return and do not read the live stage id for these counters.

Code 118 is a cumulative count of **observed model-draw hook calls** on the
eleven normal stages only. It is not a rendered frame, a completed pass, or
an FPS figure. `a` is `(stage_id << 32) | reason` and `b` is the cumulative
count. The background logger scans every 5 seconds and writes changed
nonzero counters, at most 32 per 500 ms wake, so one scan cannot fill the
128-event ring. A count can therefore reach `events.log` a few seconds after
the calls. Stage ids outside those eleven, including `-1` while no stage is
active, are not stored.

Lookup reasons, one per call that reaches the registry read: `1 found`,
`2 unregistered`, `3 ambiguous` (the stored path is 0), `4 contended`
(`try_read` failed on that bucket; the registry is not disabled), `5 disabled` (null
renderer, unhealthy registry, or poisoned lock). A found path also gets
exactly one result reason: `6 requested-native-hidden`, `7 native-visible`,
`8 suppressed`, `9 wrong-vtable`, `10 wrong-stage`. `found` equals the sum of
those result reasons. Adding every reason together double-counts found calls.
`wrong-stage` means the registered path is not a static target for the live
stage id. `suppressed` means this call cleared the draw-record argument.
`native-visible` and `requested-native-hidden` mean this call left that
argument alone. None of these reasons identify which pixels the display showed.

Code 119 is the first time the existing stage particle hook sees an
allowlisted effect name **or** an allowlisted stage caller, recorded before
the conjunction that decides whether to null the request. The request
register is unchanged by this record. `a` is the Hash40. `b` is
`(signed stage16 << 48) | (flags16 << 32) | caller32`. Flags are `name-hit`
1, `caller-allowed` 2, `stage-match` 4, and `row-off` 8. Row-off is part of
the key, so On and Off do not collapse into one record. The key also keeps
the caller and the live stage. The table holds 512 keys, separate from the
fighter `TRACE_EFFECTS` table, and it is reserved at install when the marker
is present. Only this observer takes that lock, and only with `try_lock`.

A missing 119 does **not** prove the predicate did not occur. Repeats are
not logged. A busy or poisoned dedupe lock skips that call and does not
retry. After 512 distinct keys, new keys are dropped. A request that misses
both the name list and the caller allowlist is not recorded. A return
address that is missing or does not fit in `u32` is stored as `0xffffffff`,
and every such caller collapses to that one sentinel. A stage id outside
signed 16 bits is not recorded. Code 119 records a request observation,
not a rendered frame.

`tools/resolve_trace.py` prints and `--json`-encodes 117/118/119 beside the
existing 110/114/116 groups. It does not merge them into those groups.

What a capture can support: which registry outcome dominated on a recognized
stage; whether a matched target was left native-hidden, left native-visible,
or had its draw argument cleared; and which predicate bit was clear the first
time an allowlisted particle name or caller was seen. What it cannot support:
that those counts caused a flicker or a whiteout; that a miss happened on an
unrecognized stage id; that every call was logged; or any frame-rate claim.

## What this plugin does not measure

Frame times. The capture, the per-second counts and the API trace are
Online Deluxe's (`CAPTURE_MATCH`, `tools/frame_windows.py`, `api-*.csv`).
This plugin only changes which requests reach the engine, and tells you
which ones it saw.
