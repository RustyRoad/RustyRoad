//! Per-column type mapping: scalars, sizes, and nullability.
//!
//! The defect the migration-scraping generator had was giving every column one shared
//! type, so these assert the opposite.

use super::support::{from_tables, required};
use crate::generators::rust::types;

/// Maps one column against a schema with no enums.
pub(super) fn map(name: &str, sql_type: &str, nullable: bool) -> String {
    let mut column = required(name, sql_type);
    column.nullable = nullable;

    types::map(&column, &from_tables(Vec::new())).rust
}

#[test]
fn each_column_gets_its_own_type() {
    // The whole point: a table mixing these must not collapse to one Rust type.
    assert_eq!(map("id", "integer", false), "i32");
    assert_eq!(map("big", "bigint", false), "i64");
    assert_eq!(map("small", "smallint", false), "i16");
    assert_eq!(map("name", "text", false), "String");
    assert_eq!(map("active", "boolean", false), "bool");
    assert_eq!(map("price", "double precision", false), "f64");
    assert_eq!(map("ratio", "real", false), "f32");
    assert_eq!(map("payload", "jsonb", false), "serde_json::Value");
    assert_eq!(map("raw", "bytea", false), "Vec<u8>");
}

#[test]
fn a_sized_type_keeps_its_base_type() {
    // The modifier must not defeat the lookup and fall through to the default.
    assert_eq!(map("email", "character varying(255)", false), "String");
    // utoipa's ToSchema derive recognises Decimal, not BigDecimal, and the struct derives it.
    assert_eq!(map("total", "numeric(12,2)", false), "sqlx::types::Decimal");
}

#[test]
fn a_nullable_column_is_wrapped_in_option() {
    // Decoding a NULL into a bare String fails at runtime, so this is not cosmetic.
    assert_eq!(map("description", "text", true), "Option<String>");
    assert_eq!(map("id", "integer", true), "Option<i32>");
}

#[test]
fn an_unknown_type_degrades_to_string_rather_than_failing() {
    // A pull that fails on one exotic column is worse than one usable field.
    assert_eq!(map("location", "geography(Point,4326)", false), "String");
}
