//! `create.tether` generation: the insert.

use super::sql;
use crate::generators::tetherscript::model::Model;

/// Renders the create submodule.
pub fn render(model: &Model) -> String {
    let supplied = model.insertable();

    if supplied.is_empty() {
        return empty(model);
    }

    format!(
        "// Insert for the `{table}` table.\n\
         //\n\
         // Values are bound as a parameter list, never spliced into the SQL, so a\n\
         // hostile value cannot change the shape of the statement.\n\n\
         fn create(row) {{\n\
         \x20   let sql = \"INSERT INTO {table} (\n{columns}\n    ) VALUES ({placeholders}) RETURNING *\"\n\n\
         \x20   let rows = db.query(sql, [{binds}])?\n\
         \x20   return Ok(rows[0])\n\
         }}\n\n\
         export create\n",
        table = model.table,
        columns = sql::indented_columns(&supplied, 8),
        placeholders = sql::placeholders(supplied.len()),
        binds = sql::binds(&supplied)
    )
}

/// Renders the insert for a table with no caller-supplied columns.
fn empty(model: &Model) -> String {
    format!(
        "// Insert for the `{table}` table.\n\
         //\n\
         // Every column is database-assigned, so the statement names none.\n\n\
         fn create(row) {{\n\
         \x20   let sql = \"INSERT INTO {table} DEFAULT VALUES RETURNING *\"\n\n\
         \x20   let rows = db.query(sql, [])?\n\
         \x20   return Ok(rows[0])\n\
         }}\n\n\
         export create\n",
        table = model.table
    )
}
