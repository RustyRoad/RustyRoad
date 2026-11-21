# Projection insert input usage (source examples; execution not-run)

Use the separately named inserts in `rust-projection-inserts.toml`. The generator
emits `InsertSmsConsentProjectedInput` and `InsertRouteSnapshotProjectedInput`
into generated repositories.rs, alongside their functions, not into models.rs.
Ordinary New inputs and existing ordered reads are unchanged.

## Explicit NULL for every absent nullable caller value

For EVERY column declared nullable in the generated input, wrap the caller's
Option<T> with Some. Do not use `value.map(Some)`: that silently omits absent values.
This standalone Rust example illustrates the binding rule (not a runtime facade):

```rust
fn main() {
    let absent: Option<String> = None;
    let bound_null: Option<Option<String>> = Some(absent);
    let omitted: Option<Option<String>> = None;
    let supplied = Some(Some("a value".to_owned()));
    assert_eq!(bound_null, Some(None));
    assert_ne!(bound_null, omitted);
    assert_eq!(supplied.as_ref().unwrap().as_deref(), Some("a value"));
}
```

Apply this mapping to each nullable member of the entire original bind list:

| Consent bound field | If catalog nullable: generated field assignment |
|---|---|
| phone_number | `Some(caller.phone_number)` |
| email | `Some(caller.email)` |
| consented | `Some(caller.consented)` |
| consent_text_version | `Some(caller.consent_text_version)` |
| source_url | `Some(caller.source_url)` |
| source_page | `Some(caller.source_page)` |
| ip_address | `Some(caller.ip_address)` (String; database casts to inet) |
| user_agent | `Some(caller.user_agent)` |
| customer_id | `Some(caller.customer_id)` |
| lead_id | `Some(caller.lead_id)` |

| Snapshot bound field | If catalog nullable: generated field assignment |
|---|---|
| route_id | `Some(caller.route_id)` |
| osrm_params | `Some(caller.osrm_params)` |
| input_coordinates | `Some(caller.input_coordinates)` |
| result_geometry | `Some(caller.result_geometry)` |
| planned_distance_miles | `Some(caller.planned_distance_miles)` |
| planned_duration_hours | `Some(caller.planned_duration_hours)` |
| planned_fuel_cost | `Some(caller.planned_fuel_cost)` (f64; database casts to numeric) |
| planned_total_cost | `Some(caller.planned_total_cost)` (f64; database casts to numeric) |
| capped | `Some(caller.capped)` |

For a required non-null field bind T directly; for a non-null defaulted field bind
Some(T) to supply it. Do not guess nullability: the actual pull establishes types.
The generated input's own Rust example assigns Some(None) to EVERY writable
nullable catalog field, including fields absent from RETURNING. It omits non-null
defaulted fields. Required values enter through its typed input argument.

## Caller-owned transaction example after authorized generation

With generated repositories imported and the input populated as above:

```rust,no_run
async fn consent(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mut input: InsertSmsConsentProjectedInput) -> Result<(), sqlx::Error> {
    input.consent_timestamp = None; // configured CURRENT_TIMESTAMP
    input.created_at = None; // ordinary database default, NOT an explicit clock
    input.updated_at = None; // ordinary database default, NOT an explicit clock
    let _row = insert_sms_consent_projected(&mut **tx, input).await?;
    Ok(()) // no commit: transaction ownership remains with the caller
}
```

Snapshot uses the same executor pattern with no clocks. Omit created_at and id
(when a defaulted id is present in the input); identity ids have no input field.
Input errors, cast/constraint errors and returned codec errors remain sqlx::Error.