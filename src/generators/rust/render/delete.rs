//! `delete.rs` generation: the delete.

use super::{read, sql};
use crate::database::introspection::Schema;
use crate::generators::rust::model::Model;

/// Renders the delete submodule.
pub fn render(model: &Model, schema: &Schema) -> String {
    let Some(key) = model.key() else {
        return format!(
            "//! No delete was generated for `{}`: a single-column primary key is\n\
             //! needed to address the row to delete.\n",
            model.table
        );
    };

    format!(
        "{import}\n\
         impl {name} {{\n\
         \t/// Deletes the row identified by `{key_ident}`.\n\
         \t///\n\
         \t/// Reports whether a row was removed, since deleting a key that is not\n\
         \t/// present is not an error but is rarely what the caller intended.\n\
         \tpub async fn delete({key_ident}: {key_type}) -> Result<bool, sqlx::Error> {{\n\
         \t\tlet sql = \"DELETE FROM {table} WHERE {key_column} = $1\";\n\n\
         \t\tlet result = crate::db_pool::execute(sqlx::query(sql).bind({key_ident})).await?;\n\n\
         \t\tOk(result.rows_affected() > 0)\n\
         \t}}\n\
         }}\n",
        name = model.name,
        key_ident = key.ident,
        key_type = read::borrowed(key.key_type()),
        table = model.table,
        key_column = sql::quote(&key.column),
        import = super::imports::with_key(model, schema)
    )
}
