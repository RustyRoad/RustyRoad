//! The decode ladder for a Postgres value.

use super::{arrays, scalars};
use serde_json::Value;
use sqlx::{Row, ValueRef};

/// Tries each supported type in turn.
///
/// `String` comes first so text is not misread by a looser decoder, then the
/// numeric widths, then the types needing a dedicated decoder.
pub(super) fn decode(row: &sqlx::postgres::PgRow, name: &str) -> Value {
    if let Ok(value) = row.try_get::<String, _>(name) {
        return Value::String(value);
    }
    if let Ok(value) = row.try_get::<bool, _>(name) {
        return serde_json::json!(value);
    }
    if let Some(value) = scalars::integer(row, name) {
        return value;
    }
    if let Some(value) = scalars::float(row, name) {
        return value;
    }
    if let Ok(value) = row.try_get::<Value, _>(name) {
        // json and jsonb decode straight into JSON.
        return value;
    }
    if let Some(value) = scalars::stringly(row, name) {
        return value;
    }
    if let Some(value) = arrays::pg_array(row, name) {
        return value;
    }
    if let Some(value) = arrays::pg_bytes(row, name) {
        return value;
    }

    raw(row, name)
}

/// Renders a value sqlx has no Rust type for.
///
/// Postgres sends some types in a text-compatible form, so reading the bytes as
/// UTF-8 recovers those exactly — an enum, for instance. The rest, such as
/// `interval` and `inet`, report their size and SQL type, so the caller knows what
/// they are looking at and can cast to text if the value itself is needed.
fn raw(row: &sqlx::postgres::PgRow, name: &str) -> Value {
    let Ok(value) = row.try_get_raw(name) else {
        return Value::String("<unprintable>".to_string());
    };

    let type_name = value.type_info().to_string();
    let Ok(bytes) = value.as_bytes() else {
        return Value::String(format!("<{type_name}>"));
    };

    match std::str::from_utf8(bytes) {
        Ok(text) if text.chars().all(|c| !c.is_control()) => Value::String(text.to_string()),
        // Binary payload: say what it is rather than emitting mojibake.
        _ => Value::String(format!("<{type_name}, {} bytes>", bytes.len())),
    }
}
