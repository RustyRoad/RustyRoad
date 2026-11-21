use super::{columns, Names, RelatedWrite};
use crate::database::introspection::Schema;
use crate::generators::rust::{naming::sql, repositories::equality};

/// Returns both schema-owned rows from a single mutation/snapshot statement.
/// Missing identities return an empty collection; SQL and decode errors propagate.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    write: &RelatedWrite,
) -> String {
    let table = schema.table(&write.table).unwrap();
    let foreign = table
        .foreign_keys
        .iter()
        .find(|f| f.name == write.foreign_key)
        .unwrap();
    let related = schema.table(&foreign.foreign_table).unwrap();
    let (parameters, binds, predicates) = columns::arguments(table, related, schema, names, write);
    let mutation = columns::mutation(table, related, write, &predicates);
    let projection = format!(
        "{}, {}",
        equality::projection(table, "source_row"),
        equality::projection(related, "related_row")
    );
    let query = format!("WITH changed AS ({mutation}) SELECT {projection} FROM changed AS source_row JOIN {} AS related_row ON source_row.{} = related_row.{}",
        sql(&related.name), sql(&foreign.columns[0]), sql(&foreign.foreign_columns[0]));
    let arguments = (0..write.columns.len() + 1 + write.predicates.len())
        .map(|index| format!("value_{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let row = names.row(table);
    let parent = names.row(related);
    let decode = format!(
        "({}, {})",
        equality::decode(table, names, 0),
        equality::decode(related, names, table.columns.len())
    );
    format!(
        "/// Writes through a unique related identity and returns both canonical rows.\n\
         /// One statement preserves the lookup/write snapshot; missing identities return no rows.\n\
         /// Database/default/constraint/decode errors propagate. Caller retains transaction ownership.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(executor: &sqlx::PgPool, {parameters}) -> Result<(), sqlx::Error> {{\n\
         /// let rows = {function}(executor, {arguments}).await?;\n\
         /// println!(\"changed rows: {{}}\", rows.len());\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {parameters}) -> Result<Vec<({row}, {parent})>, sqlx::Error> {{\n\
             sqlx::query({query:?}){binds}.fetch_all(executor).await?.into_iter()\n\
                 .map(|record| Ok({decode})).collect()\n\
         }}\n\n"
    )
}
