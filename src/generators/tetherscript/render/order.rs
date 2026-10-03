//! The `ORDER BY` a listing carries.

use super::sql;
use crate::generators::tetherscript::model::Model;

/// Returns an `ORDER BY` clause when the table has a column worth ordering on.
///
/// An unordered `SELECT` may return rows in any order, which makes a listing appear to
/// shuffle between calls. Newest-first matches the hand-written models.
pub(super) fn clause(model: &Model) -> String {
    if model.fields.iter().any(|f| f.column == "created_at") {
        return " ORDER BY created_at DESC".to_string();
    }

    match model.key() {
        Some(key) => format!(" ORDER BY {} ASC", sql::quote(&key.column)),
        None => String::new(),
    }
}
