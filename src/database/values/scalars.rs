//! Decoding scalar Postgres types.

use serde_json::Value;
use sqlx::Row;

/// Decodes the integer widths, narrowest first.
///
/// Order matters: a `smallint` decoded as `i32` would be silently widened, which is
/// why `i16` is attempted first.
pub(super) fn integer(row: &sqlx::postgres::PgRow, name: &str) -> Option<Value> {
    if let Ok(value) = row.try_get::<i16, _>(name) {
        return Some(serde_json::json!(value));
    }
    if let Ok(value) = row.try_get::<i32, _>(name) {
        return Some(serde_json::json!(value));
    }
    if let Ok(value) = row.try_get::<i64, _>(name) {
        return Some(serde_json::json!(value));
    }
    None
}

/// Decodes the floating-point widths.
pub(super) fn float(row: &sqlx::postgres::PgRow, name: &str) -> Option<Value> {
    if let Ok(value) = row.try_get::<f32, _>(name) {
        return Some(serde_json::json!(value));
    }
    if let Ok(value) = row.try_get::<f64, _>(name) {
        return Some(serde_json::json!(value));
    }
    None
}

/// Decodes types best rendered as their text form.
///
/// `numeric` decodes through `BigDecimal` and is rendered as a string: routing it
/// through `f64` would lose the precision the type exists to preserve, and Postgres
/// sends it in a binary form that is not readable as text.
pub(super) fn stringly(row: &sqlx::postgres::PgRow, name: &str) -> Option<Value> {
    if let Ok(value) = row.try_get::<sqlx::types::Uuid, _>(name) {
        return Some(Value::String(value.to_string()));
    }
    if let Ok(value) = row.try_get::<sqlx::types::BigDecimal, _>(name) {
        return Some(Value::String(value.to_string()));
    }
    temporal(row, name)
}

/// Decodes the date and time types.
fn temporal(row: &sqlx::postgres::PgRow, name: &str) -> Option<Value> {
    if let Ok(value) = row.try_get::<chrono::DateTime<chrono::Utc>, _>(name) {
        return Some(Value::String(value.to_rfc3339()));
    }
    if let Ok(value) = row.try_get::<chrono::NaiveDateTime, _>(name) {
        return Some(Value::String(value.to_string()));
    }
    if let Ok(value) = row.try_get::<chrono::NaiveDate, _>(name) {
        return Some(Value::String(value.to_string()));
    }
    if let Ok(value) = row.try_get::<chrono::NaiveTime, _>(name) {
        return Some(Value::String(value.to_string()));
    }
    None
}
