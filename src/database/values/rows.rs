//! Rendering MySQL and SQLite result values as display text.
//!
//! Postgres has its own richer ladder in [`super::postgres`] because it sends a
//! type OID per value. MySQL and SQLite share this simpler ladder, which both the
//! table and JSON renderers use so the two output formats agree.

use serde_json::Value;
use sqlx::{ColumnIndex, Decode, Row, Type, ValueRef};

/// Returns the first type that decodes, or `<unprintable>`.
///
/// Order matters: `String` comes first so text is not misread by a looser
/// decoder, then the numeric widths from narrowest to widest.
macro_rules! first_decoded {
    ($row:expr, $name:expr, $($ty:ty),+ $(,)?) => {{
        $(
            if let Ok(value) = $row.try_get::<$ty, _>($name) {
                return serde_json::json!(value);
            }
        )+
        Value::String("<unprintable>".to_string())
    }};
}

/// Decoding a MySQL or SQLite column without knowing its type ahead of time.
///
/// The bounds live on the blanket impl below, so callers need only this trait.
pub trait DecodableRow {
    /// Decodes one column into JSON, treating a null as null before anything else.
    fn decode_value(&self, name: &str) -> Value;

    /// Renders one column as display text for the table renderer.
    fn decode_display(&self, name: &str) -> String {
        match self.decode_value(name) {
            Value::Null => "NULL".to_string(),
            Value::String(text) => text,
            other => other.to_string(),
        }
    }
}

impl<R> DecodableRow for R
where
    R: Row,
    for<'a> &'a str: ColumnIndex<R>,
    for<'a> String: Decode<'a, R::Database> + Type<R::Database>,
    for<'a> i32: Decode<'a, R::Database> + Type<R::Database>,
    for<'a> i64: Decode<'a, R::Database> + Type<R::Database>,
    for<'a> f64: Decode<'a, R::Database> + Type<R::Database>,
    for<'a> bool: Decode<'a, R::Database> + Type<R::Database>,
{
    fn decode_value(&self, name: &str) -> Value {
        match self.try_get_raw(name) {
            Ok(raw) if raw.is_null() => return Value::Null,
            Ok(_) => {}
            Err(_) => return Value::String("<error>".to_string()),
        }
        first_decoded!(self, name, String, bool, i32, i64, f64)
    }
}
