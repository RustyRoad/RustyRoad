//! Decoding Postgres query result values for display.
//!
//! A row arrives as opaque bytes plus a type OID, so each value must be decoded
//! against a concrete Rust type. Trying only a handful of types leaves common
//! columns — `smallint`, `jsonb`, `uuid`, `numeric`, timestamps — rendering as
//! `<unprintable>`, which forces callers to cast in SQL just to read their data.

use super::ladder;
use serde_json::Value;
use sqlx::{Column, Row, ValueRef};

/// Decodes every column of a Postgres row into JSON.
pub fn pg_row(row: &sqlx::postgres::PgRow) -> serde_json::Map<String, Value> {
    let mut map = serde_json::Map::new();

    for column in row.columns() {
        let name = column.name();
        map.insert(name.to_string(), pg_value(row, name));
    }

    map
}

/// Renders one Postgres column as display text.
///
/// Used by the table renderer, which needs a plain string rather than JSON. It
/// shares the decoding so both output formats agree on every type.
pub fn pg_display(row: &sqlx::postgres::PgRow, name: &str) -> String {
    match pg_value(row, name) {
        Value::Null => "NULL".to_string(),
        Value::String(text) => text,
        other => other.to_string(),
    }
}

/// Decodes one Postgres column, treating a null as null before anything else.
fn pg_value(row: &sqlx::postgres::PgRow, name: &str) -> Value {
    match row.try_get_raw(name) {
        Ok(raw) if raw.is_null() => Value::Null,
        Ok(_) => ladder::decode(row, name),
        Err(_) => Value::String("<error>".to_string()),
    }
}
