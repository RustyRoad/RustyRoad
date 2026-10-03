//! The `UPDATE` statement itself.

use crate::database::introspection::Schema;
use crate::generators::rust::model::{Field, Model};
use crate::generators::rust::render::{read, sql};

/// Renders the update for a table that supports one.
pub(super) fn render(model: &Model, key: &Field, writable: &[&Field], schema: &Schema) -> String {
    let binds = writable
        .iter()
        .map(|field| format!("\t\t\t\t.bind(row.{})", field.ident))
        .collect::<Vec<_>>()
        .join("\n");

    let signature = format!(
        "pub async fn update({}: {}, row: {}) -> Result<Self, sqlx::Error>",
        key.ident,
        read::borrowed(key.key_type()),
        model.name
    );

    let statement = format!(
        "\t\t\tUPDATE {}\n\t\t\tSET\n{}{}\n\t\t\tWHERE {} = ${}\n\t\t\tRETURNING *",
        model.table,
        sql::assignments(writable, 1, 4),
        touch(model),
        sql::quote(&key.column),
        writable.len() + 1
    );

    format!(
        "{import}\n\
         impl {name} {{\n\
         \t/// Updates the row identified by `{ident}` and returns it as stored.\n\
         \t{signature} {{\n\
         \t\tlet sql = r#\"\n{statement}\n\t\t\"#;\n\n\
         \t\tcrate::db_pool::fetch_one(\n\
         \t\t\tsqlx::query_as::<_, Self>(sql)\n{binds}\n\
         \t\t\t\t.bind({ident}),\n\
         \t\t)\n\
         \t\t.await\n\
         \t}}\n\
         }}\n",
        name = model.name,
        ident = key.ident,
        import = super::super::imports::with_key(model, schema)
    )
}

/// Returns the `updated_at` assignment, when the table has that column.
///
/// Set by the statement rather than bound, so the server clock stays authoritative and
/// callers cannot backdate a row.
fn touch(model: &Model) -> String {
    if model.fields.iter().any(|f| f.column == "updated_at") {
        return ",\n\t\t\t\tupdated_at = CURRENT_TIMESTAMP".to_string();
    }
    String::new()
}
