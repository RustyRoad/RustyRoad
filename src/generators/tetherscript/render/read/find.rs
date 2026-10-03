//! The keyed lookup.

use crate::generators::tetherscript::model::Model;
use crate::generators::tetherscript::render::sql;

/// Renders the keyed lookup.
///
/// Returns nil for a missing row rather than an error: a lookup that finds nothing is an
/// ordinary outcome, and an error would force every caller to inspect it to discover that
/// nothing was wrong.
pub(super) fn render(model: &Model, column: &str) -> String {
    format!(
        "// Returns the row with `key`, or nil when no row matches.\n\
         fn find(key) {{\n\
         \x20   let sql = \"SELECT * FROM {table} WHERE {column} = $1\"\n\n\
         \x20   let rows = db.query(sql, [key])?\n\
         \x20   if rows.len() == 0 {{\n\
         \x20       return Ok(nil)\n\
         \x20   }}\n\
         \x20   return Ok(rows[0])\n\
         }}\n\n",
        table = model.table,
        column = sql::quote(column)
    )
}

/// Explains why no keyed lookup was generated.
pub(super) fn unsupported(model: &Model) -> String {
    format!(
        "// No keyed lookup was generated for `{}`: a single-column primary key is\n\
         // needed to address one row.\n\n",
        model.table
    )
}
