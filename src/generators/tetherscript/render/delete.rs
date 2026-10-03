//! `delete.tether` generation: the delete.

use super::sql;
use crate::generators::tetherscript::model::Model;

/// Renders the delete submodule.
pub fn render(model: &Model) -> String {
    let Some(key) = model.key() else {
        return format!(
            "// No delete was generated for `{}`: a single-column primary key is needed\n\
             // to address the row to delete.\n",
            model.table
        );
    };

    format!(
        "// Delete for the `{table}` table.\n\n\
         // Deletes the row with `key`, reporting whether one was removed.\n\
         //\n\
         // `RETURNING` is what makes the outcome visible: deleting a key that is not\n\
         // present is not an error, but it is rarely what the caller intended.\n\
         fn delete(key) {{\n\
         \x20   let sql = \"DELETE FROM {table} WHERE {column} = $1 RETURNING {column}\"\n\n\
         \x20   let rows = db.query(sql, [key])?\n\
         \x20   return Ok(rows.len() > 0)\n\
         }}\n\n\
         export delete\n",
        table = model.table,
        column = sql::quote(&key.column)
    )
}
