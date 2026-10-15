# Configuration

All files live in `sd:/ultimate/ssbu_fx_control/`.

## config.ini

```ini
scope=online        # online | all
all=on              # on | off | reduced
kazuya=on           # one line per covered fighter: on | off | reduced
captain_falcon=on
ness=on
lucas=on
ike=on
chrom=on
roy=on
palutena=on
sephiroth=on
pyra=on
mythra=on
byleth=on
inkling=on
ganondorf=on
kirby=on
variant.kirby_hammer=2       # Reduced variant per move, 0 = original; default 1 is not written
cull=eflame_sword_open       # harness: extra names nulled while that fighter is Reduced
```

- The file is created with these defaults on first start
  ([`config/default.ini`](../config/default.ini)). An unreadable file is left
  untouched and the defaults are used for that run.
- **Load order:** `all=` is applied first, then every fighter line, whatever
  the line order. A fighter line always wins over `all=`.
- `all=` is written only while every fighter has the same value; while the
  rows differ the plugin removes the line rather than leave a wrong one.
- Unknown keys, comments and lines that do not parse are ignored on load and
  kept when the plugin rewrites the file. A bad value keeps that row's default.
- `reduced` on a fighter without a Reduced variant is ignored (the row stays
  On): Kazuya, Ike, Roy and Ganondorf have Off tables only in 0.2.
- `variant.<move>=N`: the Reduced variant for one move of `src/moves.rs`
  (`falcon_punch`, `pyra_side_b`, `pyra_neutral_b`, `pyra_up_b`,
  `mythra_specials`, `palutena_explosive_flame`, `ness_pk_thunder`,
  `ness_pk_fire`, `lucas_pk_thunder`, `kirby_hammer`, `byleth_specials`,
  `inkling_roller`, `sephiroth_gigaflare`). 0 keeps that move original while
  the row is Reduced; a number above the move's variant count keeps the
  default (1). The plugin writes the line only when the value is not the
  default. See [HARNESS](HARNESS.md).
- `cull=<name>,<name>`: up to 16 request names, each with a covered
  fighter's prefix, nulled while that fighter's row is Reduced and the scope
  allows. A name without a known prefix is skipped. Read at start, kept
  verbatim on rewrite; the menu cannot change it.
- Edits by hand take effect on the next game start.

[`config/eden-test.ini`](../config/eden-test.ini) is the offline test override
(`scope=all`, every row On): copy it while the emulator is closed and set the
rows under test.

### Who writes the file

With Online Deluxe 0.5.0 or later, a single-row edit on the FX Control page is
written by Deluxe's host persistence (`config_path`, one `key=value` line, on
Deluxe's background thread). Editing **All** changes every row, which the host
cannot express, so the plugin's own background thread rewrites the whole file
after any change once the table has been quiet for two seconds. The delay
keeps the two writers from overlapping; the plugin's write always contains the
complete state. Both writers replace the file atomically (temporary file, then
rename) and keep the previous version as `config.previous.ini`.

## Scope and online detection

`scope=all` applies the per-fighter modes everywhere. `scope=online` (default)
applies them only in online play outside Training. Where the online signal
comes from depends on what else is installed:

| Installed | Online signal | Scope row |
| --- | --- | --- |
| Online Deluxe 0.5.0 or later | Deluxe's `ssbu_online_deluxe_online_context_v1` export: its `ONLINE` bit, and `is_training_mode()` false | Online / All |
| No Deluxe, no legacy SsbuSync host | The plugin's own mark-only hooks: the online-latency setup (`0x16ccab8`) marks a match online; every stage load (`0x25d9288`) clears the mark | (no Deluxe window) |
| Deluxe older than 0.5.0, or a legacy SsbuSync host | None. The plugin does not hook the latency site the host already hooks. **Scope Online never applies**: effects stay original until `scope=all` is set in the INI. | `Unavailable` (would be; an old Deluxe refuses the registration) |
| No host, but the own hooks were refused (unexpected instruction words) | None. Scope Online never applies (Deluxe's conservative choice). Scope All still works. | (no Deluxe window) |

Without Deluxe, the latency hook fires about three seconds after an online
Singles/Doubles stage loads (Deluxe's recorded console matches: 3.1 to 3.3 s),
so the first seconds of each online match run with full effects. Clearing the
mark at every stage load means an offline match after an online one is
offline again. Neither hook writes game memory; the stage id is not read.

The plugin needs nothing from Deluxe beyond the plugin-API table and
`ssbu_online_deluxe_online_context_v1`.

## Markers

| File | Effect |
| --- | --- |
| `DISABLE` | Skips every hook and the menu registration. The INI is still loaded and the logs still written. |
| `PROBE_CRITICAL_SCREEN` | Diagnostic only, latched at startup: while **every** fighter row is Off and the scope allows, the eighteen exact `bg_criticalhit` / `sys_bg_criticalhit` Special Zoom background names are suppressed. Keep it absent from normal installs. |
| `TRACE_EFFECTS` | Harness only, latched at startup: every request hook records the first request of each (name, actor category, kind, status) as event code 110, up to 512 per boot. Decode with `tools/resolve_trace.py`. Reads the actor's context for every request; keep it absent from measurement captures and normal installs. |

## Diagnostics

- `install.log`: build id, startup phases, each hook's entry word and install
  result (`<code> 0x<id> 0x<detail>` lines), the online source, the loaded
  settings and the menu registration result. The previous run is kept as
  `install.previous.log`.
- `events.log`: a bounded 128-event ring flushed every 500 ms by the plugin's
  background thread (`sequence tick code arg0 arg1`), rotated at 4 MiB.
  Hooks never write files; a contended ring counts a drop instead of waiting.
- `panic.log`: written once if the plugin panics.

Event codes keep their Online Deluxe numbers:

| Code | Meaning |
| --- | --- |
| 1 | Boot (`a` plugin text start, `b` game text base) |
| 2 | Not Smash 13.0.5; hooks skipped |
| 3 / 4 / 5 | Hook installing / installed / rejected (`a` id or offset) |
| 6 | INI loaded (`a` online source, `b` common mode or `u64::MAX` when mixed) |
| 8 | Menu edit applied (`a` option, `b` choice) |
| 11 | Off substituted `null` (`a` hash, `b` sampled total) |
| 15 | Heartbeat every 5 s (`a` table generation) |
| 16 | Coexistence: 1 SsbuSync host, 2 DISABLE marker, 3 Deluxe without online export, 4 own online hooks refused |
| 17 | Hook entry instruction word |
| 24 | INI rewritten (`b` 1 ok, 0 error) |
| 27 / 28 / 29 | ACMD effect seen / Off substituted / argument rejected |
| 30 | Online source (`a` 1 Deluxe, 2 own hooks, 3 unavailable, 4 undetected); `b` 1 own mark set, 2 cleared by a stage load |
| 84 / 85 / 86 / 87 | Reduced cull / keep / restore (scope blocked) / preserve (name on another move) |
| 91 / 92 / 93 | EFF sibling unavailable / ready / used |
| 94 / 95 | Sibling cleanup / cleanup route |
| 96 / 97 | Special Zoom probe request / marker latched |
| 99 | Menu registration (`a` handle or negative error, `-100` no host; `b` descriptor version) |
| 110 | `TRACE_EFFECTS`: first request of a (name, actor) pair (`a` Hash40; `b` bits 0–7 category, 8–23 kind, 24–39 status, 40–55 ordinal). At startup `a` = 0, `b` = 1 when the marker was latched |

Hook ids: 256 to 265 are the EffectModule request, screen, `kill_kind` and
`detach_kind` hooks; `0x110` to `0x11b` the ACMD commands; `0x16ccab8` and
`0x25d9288` the own online hooks.
