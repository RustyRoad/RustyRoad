//! The struct's fields and the imports they need.

use crate::generators::rust::model::{Field, Model};
use std::fmt::Write as _;

/// Renders the field declarations, attributes included.
pub(super) fn render(model: &Model) -> String {
    let mut body = String::new();

    for field in &model.fields {
        if let Some(column) = &field.rename {
            // The field name diverged from the column, so both serde and sqlx need
            // telling; without the sqlx rename, FromRow looks for the wrong column.
            let _ = writeln!(body, "\t#[serde(rename = \"{column}\")]");
            let _ = writeln!(body, "\t#[sqlx(rename = \"{column}\")]");
        }
        if let Some(ts) = field.mapping.ts {
            let _ = writeln!(body, "\t#[ts(type = \"{ts}\")]");
        }
        let _ = writeln!(body, "\tpub {}: {},", field.ident, field.mapping.rust);
    }

    body
}

/// Renders the imports the struct and its constructors need.
///
/// Only `chrono::Utc` is conditional: the constructors reach for it to seed the audit
/// timestamps, and an unused import is a warning on every build. The condition matches
/// what `zero` actually emits — a `created_at` that is a `date` gets no clock value, so
/// importing `Utc` for it would warn.
pub(super) fn imports(model: &Model) -> String {
    let mut file = String::new();

    // A view has no constructors, so nothing in its root reaches for the clock.
    if !model.view && model.fields.iter().any(seeds_from_clock) {
        file.push_str("use chrono::Utc;\n");
    }
    file.push_str(
        "use serde::{Deserialize, Serialize};\n\
         use sqlx::FromRow;\n\
         use ts_rs::TS;\n\
         use utoipa::ToSchema;\n\n",
    );

    file
}

/// Returns `true` when the field is seeded from the clock, so `Utc` is needed.
fn seeds_from_clock(field: &Field) -> bool {
    if !is_timestamp(field) {
        return false;
    }

    matches!(
        inner_type(&field.mapping.rust),
        "chrono::DateTime<chrono::Utc>" | "chrono::NaiveDateTime"
    )
}

/// Returns the type inside an `Option`, or the type itself.
///
/// Unwrapped by matching the prefix rather than trimming brackets: `trim_end_matches('>')` also
/// strips the closing bracket of a generic type, turning `DateTime<Utc>` into `DateTime<Utc`
/// and defeating every comparison against it.
pub(super) fn inner_type(rust: &str) -> &str {
    match rust
        .strip_prefix("Option<")
        .and_then(|rest| rest.strip_suffix('>'))
    {
        Some(inner) => inner,
        None => rust,
    }
}

/// Returns `true` when the field is an audit timestamp the database maintains.
pub(super) fn is_timestamp(field: &Field) -> bool {
    matches!(field.column.as_str(), "created_at" | "updated_at")
}
