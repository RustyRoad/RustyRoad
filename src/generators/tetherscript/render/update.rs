//! `update.tether` generation: the update.

use super::sql;
use crate::generators::tetherscript::model::Model;

/// Renders the update submodule.
pub fn render(model: &Model) -> String {
    let Some(key) = model.key() else {
        return unsupported(
            model,
            "a single-column primary key is needed to address the row to update",
        );
    };

    let writable = model.updatable();
    if writable.is_empty() {
        return unsupported(model, "every column is either the key or database-assigned");
    }

    let touch = if model
        .fields
        .iter()
        .any(|field| field.column == "updated_at")
    {
        // Set by the statement rather than bound, so the server clock stays
        // authoritative and a caller cannot backdate a row.
        ",\n        updated_at = CURRENT_TIMESTAMP".to_string()
    } else {
        String::new()
    };

    format!(
        "// Update for the `{table}` table.\n\n\
         fn update(key, row) {{\n\
         \x20   let sql = \"UPDATE {table} SET\n{assignments}{touch}\n    WHERE {key_column} = ${key_index} RETURNING *\"\n\n\
         \x20   let rows = db.query(sql, [{binds}, key])?\n\
         \x20   if rows.len() == 0 {{\n\
         \x20       return Err(\"no {table} row with key \" + str(key))\n\
         \x20   }}\n\
         \x20   return Ok(rows[0])\n\
         }}\n\n\
         export update\n",
        table = model.table,
        assignments = sql::assignments(&writable, 1, 8),
        touch = touch,
        key_column = sql::quote(&key.column),
        key_index = writable.len() + 1,
        binds = sql::binds(&writable)
    )
}

/// Renders a placeholder explaining why no update was generated.
///
/// An empty file would look like an oversight, so the reason is stated where the
/// developer is already looking.
fn unsupported(model: &Model, reason: &str) -> String {
    format!(
        "// No update was generated for `{}`: {reason}.\n\
         //\n\
         // Add one here if the table needs it; this file is regenerated, so prefer a\n\
         // sibling module for code you intend to keep.\n",
        model.table
    )
}
