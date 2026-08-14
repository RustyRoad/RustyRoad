//! The `INSERT` statement itself.

use crate::generators::rust::model::{Field, Model};
use crate::generators::rust::render::sql;

/// Renders the insert for a table with caller-supplied columns.
pub(super) fn render(model: &Model, supplied: &[&Field]) -> String {
    let binds = supplied
        .iter()
        .map(|field| format!("\t\t\t\t.bind(row.{})", field.ident))
        .collect::<Vec<_>>()
        .join("\n");

    let statement = format!(
        "\t\t\tINSERT INTO {} (\n{}\n\t\t\t)\n\t\t\tVALUES ({})\n\t\t\tRETURNING *",
        model.table,
        sql::indented_columns(supplied, 4),
        sql::placeholders(supplied.len())
    );

    format!(
        "{import}\n\
         impl {name} {{\n\
         \t/// Inserts a row and returns it as the database stored it.\n\
         \t///\n\
         \t/// `RETURNING *` is what makes the generated key and the audit timestamps\n\
         \t/// visible to the caller, which a plain insert would leave unknown.\n\
         \tpub async fn create(row: {name}) -> Result<Self, sqlx::Error> {{\n\
         \t\tlet sql = r#\"\n{statement}\n\t\t\"#;\n\n\
         \t\tcrate::db_pool::fetch_one(\n\
         \t\t\tsqlx::query_as::<_, Self>(sql)\n{binds},\n\
         \t\t)\n\
         \t\t.await\n\
         \t}}\n\
         }}\n",
        name = model.name,
        import = crate::generators::rust::render::imports::struct_only(model)
    )
}
