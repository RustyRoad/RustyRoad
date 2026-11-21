//! Partial-index predicates remain attached to their own conflict operation.
use super::{inputs, insert, Names};
use crate::database::introspection::{PartialUniqueIndex, Schema};
use crate::generators::rust::naming::sql;
use std::io;

/// Emits insert-ignore only for validated partial metadata and nonempty New inputs.
/// Concurrent conflicts on unrelated constraints remain database errors.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn render(
    schema: &Schema,
    names: &Names,
    indexes: &[PartialUniqueIndex],
) -> io::Result<String> {
    let mut output = String::new();
    for index in indexes {
        let invalid = || {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid partial-index metadata for {:?}", index.name),
            )
        };
        let table = schema.table(&index.table).ok_or_else(|| invalid())?;
        if table.view
            || index.columns.is_empty()
            || index.predicate.trim().is_empty()
            || index
                .columns
                .iter()
                .any(|name| table.column(name).is_none())
        {
            return Err(invalid());
        }
        if inputs::new_columns(table).is_empty() {
            continue;
        }
        let function = names.selector(table, "partial_insert", std::slice::from_ref(&index.name));
        let create = names.create(table);
        let target = index
            .columns
            .iter()
            .map(|name| sql(name))
            .collect::<Vec<_>>()
            .join(", ");
        let conflict = format!(
            " ON CONFLICT ({target}) WHERE ({}) DO NOTHING",
            index.predicate
        );
        let insert = insert::render(table, names);
        output.push_str(&format!(
            "/// Inserts a row, ignoring only conflicts matching partial index {:?}.\n\
             /// Example: `{function}(pool, input).await?` returns whether a row was inserted.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, input: {create}) -> Result<bool, sqlx::Error> {{\n\
                 {insert}\n\
                 query.push({conflict:?});\n\
                 query.build().execute(executor).await.map(|result| result.rows_affected() > 0)\n\
             }}\n\n", index.name,
        ));
    }
    Ok(output)
}
