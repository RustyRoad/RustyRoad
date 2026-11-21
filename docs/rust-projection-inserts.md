# Configured Rust projection inserts

Evidence level: **static/local source inspection only**. Build, tests, formatting,
installation, pull, app adoption, Git operations, and database changes are not-run.

Existing wiring: CLI `rust_models::write` forwards `models.rust_projections` to
`api::configured::render`, then `projections::render`. Omitted/false `insert`
retains filtered reads; true selects the existing atomic INSERT RETURNING renderer.
Omitting insert_input retains canonical New inputs. Setting insert_input = {} opts
into a separate <PascalCaseFunction>Input type with tri-state nullable fields.
The operation neither commits nor performs a follow-up SELECT; DB errors propagate.

## Configuration examples

These selections are illustrative; configure the application's entire result tuple.

```toml
[models.rust_projections.insert_consent]
table = "sms_consent_audit"
columns = ["id", "consent_timestamp", "ip_address"]
insert = true
database_timestamps = ["consent_timestamp"]
insert_input = { network_text = ["ip_address"] }
network_host = ["ip_address"]

[models.rust_projections.insert_route_snapshot]
table = "route_optimization_snapshots"
columns = ["id", "planned_fuel_cost", "planned_total_cost"]
insert = true
numeric_float = ["planned_fuel_cost", "planned_total_cost"]
insert_input = { numeric_float = ["planned_fuel_cost", "planned_total_cost"] }
```

HOST produces String and ::float8 produces f64 (Option for nullable columns).
Configured tuple order is preserved. Input and output cast settings are independent.
Input numeric_float binds f64 with ::float8::numeric; network_text binds String
with ::text::inet (or ::text::cidr). PostgreSQL applies target precision/constraints.
Omitted selected clocks use CURRENT_TIMESTAMP; supplied values take precedence.
Every nullable dedicated input is Option<Option<T>>: None omits, Some(None) binds
SQL NULL, Some(Some(value)) binds value. Non-null defaulted fields are Option<T>;
required fields are T. Auto-increment fields are excluded. JSON null stays a value,
distinct from SQL NULL. Unlisted inputs retain catalog/project codecs.

## Source repair and deferred gates

The shared clock policy now rejects timestamp-prefixed domains and arrays using
exact timestamp spellings (precision 0..6). Insert QueryBuilder is fully qualified.
Generated docs now specify configured tuple order and nested explicit NULLs.
See the continuation checklist below before adopting any generated operation.

1. Obtain authorization for native build and focused tests: ordinary read
   compatibility; view/join/comparison/order rejection; clock unknown/duplicate/
   ineligible rejection; casts/nullability/order; required/optional/empty New
   shapes; omitted/default/explicit/nested-NULL precedence; generated compilation.
2. Build/install this inspected source and record executable provenance. An older
   installed binary is not evidence for these changes.
3. With authorized catalog access, run actual generation in the app workspace:

   ```sh
   rustyroad pull --language rust --out crates/db/src
   ```

   Inspect canonical New inputs and generated operations. Do not write or apply
   migrations; this gate is catalog introspection and generated output only.
4. Then replace SMS consent and route snapshot handwritten INSERT callers with
   generated operations, preserving their caller-owned connection/transaction.
5. Compile/check the app and obtain separately authorized runtime evidence for
   rollback, timestamp precedence, HOST formatting, numeric casts and DB errors.

No generated app files or callers were changed by this source-only handoff.
No runtime or generated-code behavior is claimed from source inspection.

## Authorized dedicated-input design

The user approved this opt-in input, lifting the earlier canonical-New-only hold.
Ordinary New/read APIs remain unchanged. Insert order/join/comparisons are rejected.
See rust-projection-inserts.toml for full proposed caller configs and
rust-projection-input-examples.md for explicit NULL and transaction usage.
No Spotless files were inspected or edited; executable gates remain not-run.