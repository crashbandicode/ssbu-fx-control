# Provenance

What this repository took from elsewhere, and what it does not include.

## Imported from Online Deluxe

Copied byte-for-byte from Online Deluxe (`intpa/ssbu-online-deluxe-rewrite`)
at commit `f3fe436f75aa2c2f150059c932c327b2ce1985f1` (first commit of this
repository; git blob ids match): the FX policy, tables, EFF-sibling
transform, hooks, host tests, generator and the two policy ledgers. Deluxe
`f571a30` differs only by dropping the `online::install()` call from
`src/runtime/effects.rs`, which this plugin does not port either.

| Item | Status in this tree |
| --- | --- |
| `fx-policy.json` | Kept as imported: initial Off targets with source file and line |
| `fx-expanded-policy.json` | Kept as imported: expanded targets, move qualifications and the 41 excluded cues |
| EFF-sibling transform, host tests, generator | Removed in 0.2.0; on branch `archive/0.1.x-appearance-variants` |

Not ported from Deluxe: the scene tracker and its hooks, the input-buffer
write in the online-latency hook, the render profiles, the overlay and the
timing capture (`api_trace`, `CAPTURE_TIMING`).

## Other sources

- **User-supplied 2026-05-14 peer sources:** named effects and observations
  behind the FX selection. The peer plugins and compiled binaries are not bundled.
- **skyline-smash `bed642e`, `src/cpp.rs`:** the C ABI declarations in
  `src/runtime/effects.rs` match it.
- **WuBoytH/SSBU-Dumped-Scripts,** commit
  `59004685238a3f6f93bae905d3a4e079701c40b4`: move layers and variant cull
  names in `src/moves.rs` are parent requests at this pin (`moves::PIN`).
  The 0.1.x keep/cull families and their generator are on branch
  `archive/0.1.x-appearance-variants`.

`effect_names.txt` is the generated name list for `tools/resolve_trace.py`.

## Not included

No Nintendo game assets, decompiled game code or closed host NROs are
included. The repository has no license file, and `Cargo.toml` declares no
`license` field.
