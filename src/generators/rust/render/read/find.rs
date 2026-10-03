//! The keyed lookup.

use crate::generators::rust::model::{Field, Model};
use crate::generators::rust::render::sql;

/// Renders the keyed lookup.
///
/// Returns `Option` rather than erroring on a missing row: a lookup that finds nothing is an
/// ordinary outcome, and `RowNotFound` forces every caller to destructure an error to discover
/// that nothing was wrong.
pub(super) fn render(model: &Model, key: &Field) -> String {
    format!(
        "\t/// Returns the row with `{ident}`, or `None` when it does not exist.\n\
         \tpub async fn find({ident}: {parameter}) -> Result<Option<Self>, sqlx::Error> {{\n\
         \t\tlet sql = r#\"\n\
         \t\t\tSELECT *\n\
         \t\t\tFROM {table}\n\
         \t\t\tWHERE {column} = $1\n\
         \t\t\"#;\n\n\
         \t\tcrate::db_pool::fetch_optional(sqlx::query_as::<_, Self>(sql).bind({ident})).await\n\
         \t}}\n",
        ident = key.ident,
        parameter = super::borrowed(key.key_type()),
        table = model.table,
        column = sql::quote(&key.column)
    )
}
