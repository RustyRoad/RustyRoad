# Native projection input handoff evidence

Level: static/local source inspection only. No executable behavior is established.
User approval lifted the dedicated-input design hold, not execution restrictions.
No Cargo/rustc/build/test/install/formatter/Git/DB commands were invoked explicitly.
No Spotless files or migrations were changed. Actual app generation remains not-run.
The patch tool reported unavailable automatic LSP preflight; no diagnostic success
or compilation claim is made from its output.

## Files changed for the dedicated-input implementation

Paths below src/generators/rust/ unless prefixed docs/. Counts use awk FNR.

| File | Lines |
|---|---:|
| allocation.rs | 66 |
| allocation/configured.rs | 20 |
| projections.rs | 63 |
| projections/configuration.rs | 79 |
| projections/casts.rs | 75 |
| projections/insert.rs | 74 |
| projections/insert/configuration.rs | 24 |
| projections/insert/codecs.rs | 49 |
| projections/insert/fields.rs | 36 |
| projections/insert/input.rs | 61 |
| projections/insert/policy.rs | 43 |
| docs/rust-projection-inserts.md | 79 |
| docs/rust-projection-inserts.toml | 31 |
| docs/rust-projection-input-examples.md | 76 |

This evidence file is also new. The prior create_clock/policy.rs repair is retained.
No existing canonical input model or read renderer was edited in this implementation.

## Observed static/local checks

Read-only cat/rg traced configuration -> validation -> input/field rendering ->
single INSERT RETURNING fetch_one. awk ENDFILE reported every edited file <=80 lines.
Source inspection covered nested Option types, outer presence guards, typed binds,
fixed cast suffixes, clock fallback precedence, and the unchanged canonical branch.
Validation rejects read-only insert options, unknown/duplicate/incompatible input
casts, identity casts, invalid clocks, and generated input type-name collisions.
The generated input doc example visits all nullable writable catalog columns,
assigning Some(None), independent of selected RETURNING columns.

sha256sum fingerprints observed for the core generator sources:

```text
insert.rs 9bdabe16f14e7feba72bf07c790f1d88cf02f7da7898d7c3ecec06116c9d3e15
insert/input.rs 26807c61409c11b1811a2997ac69c7f6ace1dae9dcdb9e0800d27a42af3b89ce
insert/fields.rs 2a7dfe5c05ce0fc3b571988d9dbe91cf8548bdc21abffd390515744bfb1902c1
insert/codecs.rs 2ab964ec269ec82801615850538b763ab4f85508f208407c0a9280f5c2f24877
```

## Remaining gates: not-run, authorization deferred

Native compile and focused regression checks; generated-code compile; install with
source/executable provenance; authorized catalog pull into app crates/db/src;
inspect actual generated fields/codecs; then caller adoption and app checks.
Separately authorized runtime checks must cover omission vs NULL vs value, defaults,
only consent_timestamp clocking, numeric precision/errors, inet parsing/HOST output,
caller transaction rollback, ordinary read compatibility and database error propagation.
There is no remaining source-design hold; executable proof is still unavailable.