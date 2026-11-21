# Exact scoped Rust column codecs

Status: static/local source implementation only; compilation/generation/runtime not-run.
This adds an opt-in setting without changing the global rust_types configuration.

## Proposed application configuration (not applied to Spotless)

```toml
[[models.rust_column_types]]
schema = "public"
table = "parcels"
column = "geometry"
sql_type = "geometry(Polygon,4326)"
nullable = false
rust = "geography::DbPolygon"

[[models.rust_column_types]]
schema = "public"
table = "zones"
column = "definition"
sql_type = "geometry(Polygon,4326)"
nullable = false
rust = "geography::DbPolygon"
```

Required fields: schema/table/column/sql_type/rust. Optional: nullable/openapi_type.
Here schema is the PostgreSQL namespace. openapi_type is a Rust schema representation,
not a database type or a serializer. Omit it to use the codec's own ToSchema support.
Do not invent a String OpenAPI shape unless that is the codec's actual serialization.

For ONLY these two columns, the application owner must remove its existing global
geometry(Polygon,4326) override. Retaining it retains its broader existing behavior.
Scoped mappings do not affect trash_zones.polygon/bbox, zone_trash_schedules.definition,
other matching columns, or array elements. No unknown geometry type is inferred.

## Validation and resolution

- Validate against the selected catalog before creating the output directory/files.
- Require exact schema/table/column identity; missing or filtered targets fail.
- Match normalized storage declarations, retaining typmods/SRID/dimensions/qualification.
- Reject bare geometry for the configured Polygon declaration, nullability drift,
  duplicate targets, array targets, unknown config fields and invalid codec paths.
- Rust codec paths have no generic arguments, matching existing global path rules.
- Precedence: exact scoped mapping, existing global override, existing default mapping.
- Scoped mismatch is an error, never permission to fall back silently.
- Scoped OpenAPI resolution cannot inherit an unrelated global OpenAPI override.

Column owner identity travels through rows, New/Patch, filters/keys, joins, aggregates,
spatial helpers, Actix keys and dedicated projection inputs, including cloned columns.
Existing null/default wrappers, Serde derives, SQLx binds, and projection casts remain.
Explicit projection input/output conversions still take precedence for those expressions.
The external crate supplies Debug/Clone/Serde/ToSchema and compatible SQLx codecs;
RustyRoad emits its path and has no geography or Spotless dependency.

## Writer APIs and usage

Old write_api/write_api_with_codecs/write_api_with_reports signatures are unchanged.
New: rustyroad::generators::rust::write_api_with_scoped_reports takes the same arguments
as write_api_with_reports, followed by namespace: &str and columns: &[types::ColumnOverride].
See src/generators/rust/api/scoped.rs for a full Rust usage example.
Offline callers must supply the namespace represented by their catalog snapshot.
CLI uses the actual --schema selection, including the same default as introspection.

After separate executable/catalog authorization, the CLI usage is:

```sh
rustyroad pull --schema public --language rust --out crates/db/src
```

This command was NOT run. Native format_type(atttypid,atttypmod) introspection is
unchanged and retains the declaration in Column.sql_type, not Column.data_type.
No migration, Spotless change, broad geometry mapping or database operation was made.
See rust-column-codecs-evidence.md for the source inventory and deferred proof gates.