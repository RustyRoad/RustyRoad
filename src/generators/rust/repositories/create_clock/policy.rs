//! Whitelist only omittable scalar timestamp inputs for explicit database time.

use super::super::inputs;
use crate::database::introspection::{Column, Table};

/// Accept a non-key scalar timestamp that the New input can omit.
/// Domains and arrays are excluded instead of guessing their assignment casts.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn eligible(table: &Table, column: &Column) -> bool {
    inputs::optional(column)
        && !column.auto_increment
        && !table.primary_key.contains(&column.name)
        && timestamp(&column.sql_type)
}

/// Recognize scalar PostgreSQL timestamp spellings, including precision 0 through 6.
/// Prefix lookalikes (such as a timestamp_custom domain) and arrays are not clocks.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
fn timestamp(sql_type: &str) -> bool {
    let kind = sql_type.trim().to_ascii_lowercase();
    let normalized = if let Some((base, rest)) = kind.split_once('(') {
        let Some((precision, suffix)) = rest.split_once(')') else {
            return false;
        };
        if precision.len() != 1 || !matches!(precision.as_bytes()[0], b'0'..=b'6') {
            return false;
        }
        format!("{} {}", base.trim_end(), suffix.trim())
            .trim()
            .to_string()
    } else {
        kind
    };
    matches!(
        normalized.as_str(),
        "timestamp" | "timestamptz" | "timestamp without time zone" | "timestamp with time zone"
    )
}
