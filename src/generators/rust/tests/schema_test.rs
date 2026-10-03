//! Every generated type must satisfy the derives the model declares.
//!
//! The struct derives `ToSchema`, so each field's type needs a utoipa schema. `BigDecimal`
//! has none — utoipa recognises `Decimal` (rust_decimal) instead — so a `numeric` column
//! produced a struct that failed on its own derive: 301 errors across the live schema, none
//! of them visible to a substring assertion.

use super::support::{from_tables, required};
use crate::generators::rust::types;

/// Types utoipa's `ToSchema` derive cannot resolve unaided.
///
/// Listed by name rather than detected, because the derive's own type table is what decides
/// and it is not introspectable from here.
const UNSUPPORTED: [&str; 1] = ["sqlx::types::BigDecimal"];

/// Maps one column against a schema with no enums.
fn map(sql_type: &str) -> types::Mapping {
    types::map(&required("value", sql_type), &from_tables(Vec::new()))
}

#[test]
fn no_generated_type_is_one_utoipa_cannot_schema() {
    // Every SQL type the generator claims to handle, so a future mapping cannot quietly
    // reintroduce an unschemable type.
    let sql_types = [
        "smallint",
        "integer",
        "bigint",
        "boolean",
        "real",
        "double precision",
        "numeric(12,2)",
        "decimal",
        "money",
        "uuid",
        "json",
        "jsonb",
        "bytea",
        "date",
        "time",
        "timestamp",
        "timestamp with time zone",
        "text",
        "character varying(255)",
        "geography(Point,4326)",
    ];

    for sql_type in sql_types {
        let mapped = map(sql_type);

        assert!(
            !UNSUPPORTED.contains(&mapped.rust.as_str()),
            "{sql_type} maps to {}, which has no utoipa schema",
            mapped.rust
        );
    }
}

#[test]
fn an_exact_decimal_still_declares_a_ts_override() {
    // Whatever the Rust type is, ts_rs cannot see through it, so the field must say what it
    // serializes as or the exported TypeScript will not compile.
    for sql_type in ["numeric(12,2)", "decimal", "money"] {
        assert_eq!(
            map(sql_type).ts,
            Some("string"),
            "{sql_type} lost its ts override"
        );
    }
}
