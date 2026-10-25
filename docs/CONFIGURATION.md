# Configuration

All files live in `sd:/ultimate/ssbu_fx_control/`: `config.ini`, optional
[marker files](#markers) and [logs](#diagnostics). Every exact rule is in the
[configuration reference](CONFIGURATION_REFERENCE.md).

[config.ini](#config-ini) · [Keys](#keys) · [Examples](#examples) ·
[Load rules](#load-rules) · [Who writes the file](#who-writes-the-file) ·
[Scope and online detection](#scope-and-online-detection) ·
[Markers](#markers) · [Diagnostics](#diagnostics)

## config.ini

First start writes the defaults from
[`config/default.ini`](../config/default.ini): Scope Online, every fighter and
stage On, variant 1 where a variant exists, and no cull list. Hand edits take
effect on the next game start; an unreadable file is left untouched and the
defaults are used for that run.

For offline testing, copy [`config/eden-test.ini`](../config/eden-test.ini)
(`scope=all`, every row On) while the emulator is closed, then set the rows
under test.

## Keys

| Key | Values | Default |
| --- | --- | --- |
| `scope` | `online` / `all` | `online` |
| `all` | `on` / `off` / `reduced` | `on` |
| One per fighter, e.g. `kirby` | `on` / `off` / `reduced` | `on` |
| `variant.<move>` | `0` to the move's variant count | `1` where a variant exists (not written) |
| `cull` | Up to 16 comma-separated names | Empty |
| `stages` | `on` / `off` | `on` |
| `stage.<key>`, e.g. `stage.end` | `on` / `off` | `on` |

All fighter, move and stage key names are in the
[reference](CONFIGURATION_REFERENCE.md#keys). Moves and their variant counts
come from the ledger in [`src/moves.rs`](../src/moves.rs).

## Examples

When applying a snippet, remove conflicting per-fighter or per-stage overrides.

Fighter rows in offline matches and Training too, every fighter Reduced
(Inkling, which has no variant, stays On):

```ini
scope=all
all=reduced
```

Captain Falcon Reduced, but Falcon Punch left original:

```ini
captain_falcon=reduced
variant.falcon_punch=0
```

Smashville's targeted scenery Off, every other stage On; this applies offline
and online whatever `scope=` says:

```ini
stages=on
stage.animal_village=off
```

## Load rules

![Fighter rows are gated by Scope and set by all= then each fighter line; stage rows have no Scope gate and are set by stages= then each stage line. Deluxe host persistence writes one line for a Scope or single fighter row edit; the plugin saver writes the whole file, including all=, variant, stages= and stage lines.](media/config-two-groups.svg)

- A fighter line always beats `all=`, and a stage line beats `stages=`,
  whatever the line order.
- Unknown keys, comments and bad lines are ignored on load and kept on
  rewrite. A bad value keeps that row's default.
- `reduced` on Inkling is ignored (the row stays On), and stage rows ignore
  `reduced`.

Details for `all=`, `variant.`, `cull=` and removed fighters:
[load rules](CONFIGURATION_REFERENCE.md#load-rules).

## Who writes the file

With Online Deluxe 0.5.3 or later, Deluxe writes a single fighter-page edit as
one line. The plugin also rewrites the whole file after any change, once the
table has been quiet for two seconds; that is how **All**, variant and Stages
edits are saved. Both keep the previous file as `config.previous.ini`.
[Writers](CONFIGURATION_REFERENCE.md#writers)

## Scope and online detection

`scope=all` applies fighter rows everywhere; `scope=online` (default) only in
online play outside Training. Stage rows ignore Scope.

- **Deluxe 0.5.0 or later** supplies the online signal.
- **No Deluxe** (and no legacy SsbuSync host): the plugin's own hooks mark a
  match online about three seconds after its stage loads, so those first
  seconds keep full fighter effects. If those hooks are refused, Scope
  Online never applies.
- **Deluxe older than 0.5.0, or a legacy SsbuSync host**: no signal, so Scope
  Online never applies; use `scope=all`.

[Online detection](CONFIGURATION_REFERENCE.md#online-detection)

## Markers

Marker files in the same folder:

| File | Use |
| --- | --- |
| `DISABLE` | Skips every hook and the menu; the INI and logs still work |
| `PROBE_CRITICAL_SCREEN` | Diagnostic only; keep absent |
| `TRACE_EFFECTS` | Harness tracing; keep absent from captures and normal installs |
| `TRACE_STAGE_DIAGNOSTICS` | Stage harness only |

[Marker details](CONFIGURATION_REFERENCE.md#markers)

## Diagnostics

`install.log` records startup and each hook's result, `events.log` holds the
event ring, and `panic.log` appears only after a panic.
[Log formats](CONFIGURATION_REFERENCE.md#logs) ·
[Event codes](CONFIGURATION_REFERENCE.md#event-codes)
