# Provenance

The FX policy, tables, EFF-sibling transform, hooks, host tests, generator and
the two policy ledgers were copied byte-for-byte from Online Deluxe
(`intpa/ssbu-online-deluxe-rewrite`) at commit
`f3fe436f75aa2c2f150059c932c327b2ce1985f1` (first commit of this repository;
git blob ids match). Deluxe `f571a30` differs only by dropping the
`online::install()` call from `src/runtime/effects.rs`, which this plugin does
not port either. The EFF-sibling transform, the host tests and the generator
are gone from the 0.2.0 tree and survive on the branch
`archive/0.1.x-appearance-variants`; the two ledgers remain here as imported.

- `fx-policy.json`: initial Off targets with source file and line.
- `fx-expanded-policy.json`: expanded targets, move qualifications and the
  41 excluded cues.

FX selection uses named effects and observations from the user-supplied
2026-05-14 peer sources; the peer plugins and their compiled binaries are not
bundled. C ABI declarations in `src/runtime/effects.rs` match skyline-smash
bed642e `src/cpp.rs`. The move layers and variant cull names in
`src/moves.rs` are parent requests in WuBoytH/SSBU-Dumped-Scripts
`59004685238a3f6f93bae905d3a4e079701c40b4` (the 0.1.x keep/cull families and
their generator are on branch `archive/0.1.x-appearance-variants`).
`effect_names.txt` is the generated name list for `tools/resolve_trace.py`.
No Nintendo game assets, decompiled game code or closed host NROs are
included.

Not ported from Deluxe: the scene tracker and its hooks, the input-buffer
write in the online-latency hook, the render profiles, the overlay and the
timing capture (`api_trace`, `CAPTURE_TIMING`).
