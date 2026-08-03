//! Decoding array and network types.

use serde_json::Value;
use sqlx::Row;

/// Decodes the array types Postgres sends in a binary form.
///
/// Only the element types a query is likely to select are attempted; an unusual
/// element type still reports its size rather than misrendering.
pub(super) fn pg_array(row: &sqlx::postgres::PgRow, name: &str) -> Option<Value> {
    if let Ok(values) = row.try_get::<Vec<String>, _>(name) {
        return Some(Value::Array(
            values.into_iter().map(Value::String).collect(),
        ));
    }
    if let Ok(values) = row.try_get::<Vec<i16>, _>(name) {
        return Some(serde_json::json!(values));
    }
    if let Ok(values) = row.try_get::<Vec<i32>, _>(name) {
        return Some(serde_json::json!(values));
    }
    if let Ok(values) = row.try_get::<Vec<i64>, _>(name) {
        return Some(serde_json::json!(values));
    }
    if let Ok(values) = row.try_get::<Vec<f64>, _>(name) {
        return Some(serde_json::json!(values));
    }
    if let Ok(values) = row.try_get::<Vec<bool>, _>(name) {
        return Some(serde_json::json!(values));
    }
    if let Ok(values) = row.try_get::<Vec<sqlx::types::Uuid>, _>(name) {
        return Some(Value::Array(
            values
                .into_iter()
                .map(|value| Value::String(value.to_string()))
                .collect(),
        ));
    }
    None
}

/// Decodes `bytea`, which has no meaningful JSON form.
///
/// Reported as a size rather than dumped, since a blob would flood the output.
pub(super) fn pg_bytes(row: &sqlx::postgres::PgRow, name: &str) -> Option<Value> {
    row.try_get::<Vec<u8>, _>(name)
        .ok()
        .map(|bytes| Value::String(format!("<{} bytes>", bytes.len())))
}
