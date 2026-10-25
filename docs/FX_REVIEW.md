# FX expansion review disposition — 2026-10-05

**Source review of Online Deluxe's FX expansion, not runtime evidence.** It
predates ssbu-fx-control: the plugin's per-fighter mapping, menu and own
online hooks have not had an independent review yet.

Fable 5.1 xhigh independently checked the expansion against the supplied
May-14 peer source and skyline-smash bed642e.

| Finding | Primary resolution |
|---|---|
| Peer scale tables include projectile/position, reflector/counter and hitbox cues | Exclude 41 exact names from global and scoped Off; regression test and JSON ledger |
| Native effect installation dereferenced target before checking mapping | QueryMemory guard and HookRejected; skip that hook |
| Flip commands have two effect names before joint | Independently filter slots 1/2, retain joint/rest; host regression |
| Lua Int/Hash type normalization unnecessary | Preserve incoming kind and metadata |
| Tracker unavailable could look like a broken toggle | Visible status plus diagnostic Compatibility record (in this plugin: the Scope row shows `Unavailable`, code 16) |

Two further findings concerned Online Deluxe's scene tracker and input
buffer (Quickplay main-menu handling, the DISABLE_BUFFER flag); they stayed
with Deluxe and are omitted here.

**Verified:** primitive-stack bounds, 24-byte return ABI, actor
category/kind, motion/status getters, constants and weapon qualification. No
owner traversal or global victim counters are used. No accidental omissions
in the promoted peer groups apart from the explicitly deferred generic
`sys_attack_arc_d`.

**Policy, not parity.** Off intentionally removes some named layers that the
peer only shrank; cue exclusions take priority. Reduced was pending at the
time (since 0.2 it is the per-move variants in `src/moves.rs`, none accepted
yet; see [HARNESS](HARNESS.md)).

**Open at review time.** Online context had source evidence but needed the
actual transition matrix. The old `c1706bbe` runtime acceptance does not
validate new hooks.

The full review is retained locally with the audit artifacts. No peer code is
added as a runtime dependency by this review. The 41 excluded cues are listed
in `reference/fx-expanded-policy.json` ([provenance](../reference/README.md)).
