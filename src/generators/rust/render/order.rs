//! The `ORDER BY` a listing carries.

use crate::generators::rust::model::Model;

/// Returns an `ORDER BY` clause when the table has a column worth ordering on.
///
/// An unordered `SELECT` may return rows in any order, which makes a listing appear to
/// shuffle between calls. Newest-first matches the hand-written models.
pub(super) fn clause(model: &Model, indent: &str) -> String {
    if model.fields.iter().any(|f| f.column == "created_at") {
        return format!("{indent}ORDER BY created_at DESC");
    }

    match model.key() {
        Some(key) => format!("{indent}ORDER BY {} ASC", super::sql::quote(&key.column)),
        None => String::new(),
    }
}
