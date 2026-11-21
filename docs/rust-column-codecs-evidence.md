# Scoped codec source handoff

Evidence level: static/local only. Native build, tests, generation and runtime not-run.
No Cargo/rustc/build/install/formatter/Git/DB commands or Spotless edits were performed.
Patch-tool LSP preflight was incomplete; it is not compilation evidence.

## Exact changed source inventory (43 files)

Paths relative to src/generators/rust unless beginning src/. Counts: awk FNR.

| File | Lines |
|---|---:|
| src/database/introspection/cli/mod.rs | 79 |
| src/database/introspection/cli/rust_models.rs | 57 |
| src/generators/model_paths/settings.rs | 74 |
| aggregates/query.rs | 37 |
| allocation.rs | 69 |
| allocation/configured.rs | 21 |
| allocation/scoped.rs | 48 |
| api.rs | 68 |
| api/scoped.rs | 43 |
| group_metrics/expressions.rs | 52 |
| group_metrics/query.rs | 58 |
| json_inserts/query.rs | 48 |
| models/fields.rs | 54 |
| models/inputs.rs | 53 |
| models/table.rs | 64 |
| mod.rs | 30 |
| procedures.rs | 34 |
| procedures/handlers.rs | 62 |
| procedures/preamble.rs | 57 |
| procedures/routes.rs | 56 |
| procedures/scaffolds.rs | 36 |
| projections/casts.rs | 75 |
| projections/insert/input.rs | 61 |
| projections/joined/fields.rs | 41 |
| projections/selection.rs | 26 |
| related_writes/columns.rs | 53 |
| related_writes/validation.rs | 71 |
| relation_inserts/query.rs | 46 |
| relation_inserts/validation.rs | 56 |
| repositories/atomic.rs | 77 |
| repositories/atomic/clocks/input.rs | 33 |
| repositories/atomic/increment.rs | 55 |
| repositories/equality/filters/comparisons/fields.rs | 44 |
| repositories/equality/filters/fields.rs | 50 |
| repositories/selector.rs | 72 |
| repositories/spatial.rs | 44 |
| repositories/table.rs | 54 |
| spatial_reports/columns.rs | 28 |
| types/column_override.rs | 32 |
| types/column_validation.rs | 43 |
| types/mod.rs | 55 |
| url_stats/query.rs | 40 |
| url_stats/validation.rs | 50 |

New docs: this file and rust-column-codecs.md (configuration/API examples).
The procedures split was necessary to keep the edited 203-line source <=80 lines.

## Static/local findings and remaining gates

Read-only rg found zero remaining map_named references: consumers carry table identity.
cat inspection traced namespace -> scoped validation -> configured rendering -> output.
Empty filtered catalogs reject configured targets instead of silently skipping them.
All listed sources are <=80 lines. Serde/default/null wrappers were retained.
Prior projection-insert evidence remains historical: its input renderer fingerprint
changed because scoped identity now flows into its normal codec lookup.

Compatible executable unavailable in checked installed/debug/release locations.
Installed ~/.cargo/bin/rustyroad and target/release/rustyroad share SHA-256:
745a475d5290392e84f68e1b9fc5d5c1422338fd39fc78f017ab00646adf57fd
Their Oct17 timestamps predate this repair; debug is Oct16. A read-only strings scan
found no rust_column_types/openapi_type/insert_input markers. Binaries were not executed.

Deferred: native and generated compile, focused drift/array/namespace/wrapper checks,
install provenance, actual scoped pull, external geography codec/API compatibility,
application adoption and runtime SQLx/Serde/OpenAPI proof. No executable proof is claimed.