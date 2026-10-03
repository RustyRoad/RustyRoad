//! Zero values for the fields a caller does not supply.

use crate::generators::rust::model::Field;
use crate::generators::rust::render::root::fields::{inner_type, is_timestamp};

/// The offset-aware timestamp type, seeded from the clock.
const OFFSET_AWARE: &str = "chrono::DateTime<chrono::Utc>";

/// The wall-clock timestamp type, seeded from the clock.
const NAIVE: &str = "chrono::NaiveDateTime";

/// Returns the zero value for a field.
pub(super) fn value(field: &Field) -> String {
    // Seeded from the clock rather than left empty so a row built in code carries the same
    // timestamps the database would have written. Keyed on the resolved type, not the column
    // name: a `timestamptz` audit column is `DateTime<Utc>`, and a naive seed would not
    // type-check against it.
    if is_timestamp(field) {
        if let Some(seed) = clock(field) {
            return if field.nullable {
                format!("Some({seed})")
            } else {
                seed.to_string()
            };
        }
    }

    if field.nullable {
        return "None".to_string();
    }

    zero(&field.mapping.rust)
}

/// Returns the clock expression for a timestamp field, if its type takes one.
///
/// A column named `created_at` that is actually a `date` or a string gets no clock value: the
/// name is a convention, and the type is what has to compile.
fn clock(field: &Field) -> Option<&'static str> {
    // Compared with `if`, not `match`: a `match` arm naming a const binds a fresh variable and
    // matches everything, which silently reinstates the bug this guards.
    let inner = inner_type(&field.mapping.rust);

    if inner == OFFSET_AWARE {
        return Some("Utc::now()");
    }
    if inner == NAIVE {
        return Some("Utc::now().naive_utc()");
    }
    None
}

/// Returns the zero value for a non-nullable Rust type.
fn zero(rust: &str) -> String {
    if rust.starts_with("Vec<") {
        return "vec![]".to_string();
    }

    match rust {
        "bool" => "false".to_string(),
        "i16" | "i32" | "i64" => "0".to_string(),
        "f32" | "f64" => "0.0".to_string(),
        "String" => "\"\".to_string()".to_string(),
        "serde_json::Value" => "serde_json::json!({})".to_string(),
        "chrono::NaiveDateTime" | "chrono::NaiveDate" | "chrono::NaiveTime" => {
            format!("{rust}::default()")
        }
        // A type without a literal zero still implements Default, which covers Decimal, Uuid,
        // DateTime<Utc>, and any generated enum.
        _ => "Default::default()".to_string(),
    }
}
