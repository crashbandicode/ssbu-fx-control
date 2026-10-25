# Configuration reference

Exact keys, parse rules, writers, online detection, markers, logs, event codes
and hook ids. The short guide is [CONFIGURATION](CONFIGURATION.md).

[Keys](#keys) · [Load rules](#load-rules) · [Writers](#writers) ·
[Online detection](#online-detection) · [Markers](#markers) · [Logs](#logs) ·
[Event codes](#event-codes) · [Hook ids](#hook-ids)

## Keys

Example settings; the variant and cull lines illustrate optional overrides,
not first-start defaults:

```ini
scope=online
all=on
captain_falcon=on
ness=on
lucas=on
palutena=on
sephiroth=on
pyra=on
mythra=on
byleth=on
inkling=on
kirby=on
variant.kirby_hammer=2
cull=eflame_sword_open
stages=on
stage.end=on
```

| Key | Values | Default |
| --- | --- | --- |
| `scope` | `online` / `all` | `online` |
| `all` | `on` / `off` / `reduced` | `on` |
| One per page fighter (below) | `on` / `off` / `reduced` | `on` |
| `variant.<move>` | `0` to the move's variant count | `1` where a variant exists (not written) |
| `cull` | Up to 16 comma-separated names | Empty |
| `stages` | `on` / `off` | `on` |
| `stage.<key>` (below) | `on` / `off` | `on` |

- **Fighters** (`fighters::ALL`, also the page order): `captain_falcon`, `ness`,
  `lucas`, `palutena`, `sephiroth`, `pyra`, `mythra`, `byleth`, `inkling`,
  `kirby`.
- **Moves** (`src/moves.rs`): `falcon_punch`, `pyra_side_b`, `pyra_neutral_b`,
  `pyra_up_b`, `mythra_specials`, `palutena_explosive_flame`,
  `ness_pk_thunder`, `ness_pk_fire`, `lucas_pk_thunder`, `kirby_hammer`,
  `byleth_specials`, `inkling_roller`, `sephiroth_gigaflare`.
- **Stages**: `battle_field`, `battle_field_s`, `end`, `yoshi_cartboard`,
  `fox_lylatcruise`, `poke_stadium2`, `animal_village`, `poke_kalos`,
  `animal_city`, `ff_cave`, `trail_castle`
  ([which stage is which](STAGES.md#what-off-hides)).

## Load rules

- **Load order:** `all=` is applied first, then every fighter line, whatever
  the line order. A fighter line always wins over `all=`.
- `all=` applies to the ten page fighters only. It is written only while
  every page fighter has the same value; while the rows differ the plugin
  removes the line rather than leave a wrong one.
- Unknown keys, comments and lines that do not parse are ignored on load and
  kept when the plugin rewrites the file. A bad value keeps that row's default.
- Fighter lines exist only for the ten fighters that have a move on the ledger
  and a row on the Deluxe page (`fighters::ALL`). A line for another fighter
  (for example `kazuya=off` from an older file) is ignored on load and kept
  verbatim on rewrite. Kazuya, Ike, Chrom, Roy and Ganondorf are removed from
  the code (no Off tables, no move profiles, no name prefixes); their lines
  (`kazuya=`, `ike=`, `chrom=`, `roy=`, `ganondorf=`) are unknown keys and
  their effect requests pass through untouched.
- `reduced` on a fighter without a Reduced variant is ignored (the row stays
  On): that is Inkling, whose roller has no variant.
- `variant.<move>=N` selects the Reduced variant for one move of
  `src/moves.rs`. 0 keeps that move original while the row is Reduced; a
  number above the move's variant count keeps the default (1). The plugin
  writes the line only when the value is not the default. The same value is
  the move's row on the Deluxe page (Original / Variant 1 .. N); a change there
  is saved to this line by the plugin's own saver, like `all=`. See
  [HARNESS](HARNESS.md).
- `cull=<name>,<name>`: up to 16 request names, each with a covered
  fighter's prefix, nulled while that fighter's row is Reduced and the scope
  allows. A name without a known prefix is skipped. Read at start, kept
  verbatim on rewrite; the menu cannot change it.
- `stages=on|off` is applied before every `stage.<key>=on|off`, whatever the
  line order. A stage line wins over `stages=`. These keys apply offline and
  online and ignore fighter All, Reduced, and Scope. All eleven stages
  default On. A stage value of `reduced`, or any other unknown value, is
  ignored. Reload the stage after a menu change for a consistent result. A
  rejected stage hook disables its rows and **All backgrounds**. Keys and
  coverage are in [STAGES](STAGES.md).

An unreadable file is left untouched and the defaults are used for that run.
Edits by hand take effect on the next game start.

## Writers

With Online Deluxe 0.5.3 or later, a single-row edit on the FX Control page is
written by Deluxe's host persistence (`config_path`, one `key=value` line, on
Deluxe's background thread). Editing **All** changes every row, which the host
cannot express, so the plugin's own background thread rewrites the whole file
after any change once the table has been quiet for two seconds. The delay
coalesces edits; the plugin's write contains the complete state. Both writers
replace the file atomically (temporary file, then rename) and keep the
previous version as `config.previous.ini`.

The **Stages** page sets every row `no_persist` and registers no host config
path. Its edits are saved only by the plugin's quiet-period saver, including
`stages=` and each `stage.<key>=`. Fighter host persistence does not write
those keys.

## Online detection

`scope=all` applies the per-fighter modes everywhere. `scope=online` (default)
applies them only in online play outside Training. The online signal depends on
what else is installed:

| Installed | Online signal | Scope row |
| --- | --- | --- |
| Online Deluxe 0.5.0 or later | Deluxe's `ssbu_online_deluxe_online_context_v1` export: its `ONLINE` bit, and `is_training_mode()` false | Online / All (the page, and so the row, exists on 0.5.3 or later; 0.5.0 to 0.5.2 refuse the registration) |
| No Deluxe, no legacy SsbuSync host | The plugin's own mark-only hooks: the online-latency setup (`0x16ccab8`) marks a match online; every stage load (`0x25d9288`) clears the mark | (no Deluxe window) |
| Deluxe older than 0.5.0, or a legacy SsbuSync host | None. The plugin does not hook the latency site the host already hooks. **Scope Online never applies**: effects stay original until `scope=all` is set in the INI. | (no page: an old Deluxe refuses the registration) |
| No host, but the own hooks were refused (unexpected instruction words) | None. Scope Online never applies (Deluxe's conservative choice). Scope All still works. | (no Deluxe window) |

Without Deluxe, the latency hook fires about three seconds after an online
Singles/Doubles stage loads (Deluxe's recorded console matches: 3.1 to 3.3 s),
so the first seconds of each online match run with full fighter effects.
Clearing the mark at every stage load means an offline match after an online
one is offline again. Neither online-tracking hook writes game memory or
reads the stage id. The separate stage-background filter queries the guarded
stage API and does not consult fighter Scope. That is why stage rows still
apply when the online mark is late.

The plugin needs nothing from Deluxe beyond the plugin-API table and
`ssbu_online_deluxe_online_context_v1`.

## Markers

Marker files in `sd:/ultimate/ssbu_fx_control/`:

| File | Effect |
| --- | --- |
| `DISABLE` | Skips every hook and the menu registration. The INI is still loaded and the logs still written. |
| `PROBE_CRITICAL_SCREEN` | Diagnostic only, latched at startup: while **every** fighter row is Off and the scope allows, the eighteen exact `bg_criticalhit` / `sys_bg_criticalhit` Special Zoom background names are suppressed. Keep it absent from normal installs. |
| `TRACE_EFFECTS` | Harness only, latched at startup: every EffectModule `req*` hook records the first request of each (name, actor category, kind, status) as event code 110, up to 512 per boot. The same marker also records stage particle code 114 and scenery-group code 116. Decode with `tools/resolve_trace.py`. Reads the actor's context for every request; keep it absent from measurement captures and normal installs. |
| `TRACE_STAGE_DIAGNOSTICS` | Stage harness only, latched once when the stage hooks install. Codes 117, 118 and 119. Independent of `TRACE_EFFECTS` and of stage On/Off. Absent until the next process start if created mid-session. See [HARNESS](HARNESS.md#stage-diagnostics). |

## Logs

- `install.log`: build id, startup phases, each hook's entry word and install
  result (`<code> 0x<id> 0x<detail>` lines), the online source, the loaded
  settings and the menu registration result. The previous run is kept as
  `install.previous.log`.
- `events.log`: a bounded 128-event ring flushed every 500 ms by the plugin's
  background thread (`sequence tick code arg0 arg1`), rotated at 4 MiB.
  Hooks never write files; a contended ring counts a drop instead of waiting.
- `panic.log`: written once if the plugin panics.

## Event codes

Each `events.log` line is `sequence tick code arg0 arg1`. Event codes keep
their Online Deluxe numbers:

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
| 91 to 95 | Retired 0.1.x sibling codes, not in the code (numbers reserved) |
| 96 / 97 | Special Zoom probe request / marker latched |
| 99 | Menu registration (`a` handle or negative error, `-100` no host; `b` 4, the descriptor version SDK 5 sends, or 0 on failure) |
| 110 | `TRACE_EFFECTS`: first request of a (name, actor) pair (`a` Hash40; `b` bits 0–7 category, 8–23 kind, 24–39 status, 40–55 ordinal). At startup `a` = 0, `b` = 1 when the marker was latched |
| 111 | Stages page registration, same handle/error and descriptor fields as 99 |
| 112 | Stage particle request replaced with `null` (`a` original hash; `b` stage id in the high 32 bits, sampled count in the low 32) |
| 113 | Stages menu edit (`a` option index, `b` choice), separate from fighter code 8 |
| 114 | `TRACE_EFFECTS`: first qualifying background name on a stage (`a` hash; `b` stage id in the high 32 bits, ordinal in the low 32) |
| 115 | Scenery draw argument forced off (`a` full folder-path hash; `b` stage id and sampled count). Stage-object visibility stays native |
| 116 | `TRACE_EFFECTS`: first targeted scenery group (`a` full folder-path hash; `b` stage id and ordinal) |
| 117 | `TRACE_STAGE_DIAGNOSTICS` latched at stage install (`a` 1 present, 0 absent) |
| 118 | Cumulative model-hook observations for one of the eleven stages (`a` stage id and reason; `b` call count). Not a frame or FPS count |
| 119 | First particle predicate observation before the null decision (`a` hash; `b` stage, flags, and caller). Repeats and a full 512-key table are omitted |

Decode trace codes 110, 114 and 116 with `tools/resolve_trace.py`. Codes 117
to 119 are decoded field by field in the
[diagnostic event reference](DIAGNOSTICS_REFERENCE.md#stage-diagnostics-117-118-119);
the harness summary is [HARNESS](HARNESS.md#stage-diagnostics). The other trace
fields are in the [diagnostic event reference](DIAGNOSTICS_REFERENCE.md).

## Hook ids

Hook ids: 256 to 263 are the eight EffectModule request and screen hooks;
`0x110` to `0x119` the ten ACMD effect commands; `0x16ccab8` and `0x25d9288`
the own online hooks (installed only when no Deluxe host provides the online
signal). The 0.1.x `kill_kind` / `detach_kind` hooks (264, 265) and the ACMD
`EFFECT_OFF_KIND` / `EFFECT_DETACH_KIND` hooks (`0x11a`, `0x11b`) are not
installed. Stage visuals, 13.0.5 only, use the guarded effect request at
`0x3563430` and the draw-record argument at `0x35d73e0`. Caller lists are in
[STAGE_REFERENCE](STAGE_REFERENCE.md#request-boundary-13-0-5-only), preserved
geometry in [STAGE_TARGETS](STAGE_TARGETS.md#preserved-not-targeted), and the
open acceptance limits in
[STAGES](STAGES.md#remaining-runtime-acceptance).
