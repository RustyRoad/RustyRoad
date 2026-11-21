use super::{expressions, JsonInsert, Names};
use crate::database::introspection::Schema;
use crate::generators::rust::{naming::sql, types};

/// Emits one atomic INSERT SELECT with conflict deduplication and database-side casts.
/// Returns affected rows without decoding a wider row or committing the executor.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    insert: &JsonInsert,
) -> String {
    let table = schema.table(&insert.table).unwrap();
    let mut parameters = Vec::new();
    let mut inputs = Vec::new();
    let mut binds = String::new();
    let mut args = Vec::new();
    for (index, name) in insert.columns.iter().enumerate() {
        let column = table.column(name).unwrap();
        let value = format!("value_{index}");
        parameters.push(format!(
            "{value}: {}",
            types::map_column(table, column, schema, names).rust
        ));
        inputs.push(format!(
            "${}::{} AS {}",
            index + 1,
            expressions::cast(column).unwrap(),
            sql(name)
        ));
        binds.push_str(&format!(".bind({value})"));
        args.push(value);
    }
    let assigned = expressions::assigned(table, insert);
    let columns = assigned
        .iter()
        .map(|(name, _)| sql(name))
        .collect::<Vec<_>>()
        .join(", ");
    let values = assigned
        .iter()
        .map(|(_, value)| value.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let conflict = insert
        .conflict
        .iter()
        .map(|name| sql(name))
        .collect::<Vec<_>>()
        .join(", ");
    let query = format!("WITH input AS (SELECT {}) INSERT INTO {} ({columns}) SELECT {values} FROM input ON CONFLICT ({conflict}) DO NOTHING",
        inputs.join(", "), sql(&table.name));
    let parameters = parameters.join(", ");
    let args = args.join(", ");
    format!(
        "/// Inserts with guarded JSON integers and atomic conflict deduplication.\n\
         /// Missing/non-number JSON keys yield NULL; PostgreSQL cast errors propagate.\n\
         /// Existing conflict keys return zero affected rows; no implicit commit occurs.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, {parameters}) -> Result<(), sqlx::Error> {{\n\
         /// let affected = {function}(pool, {args}).await?;\n\
         /// println!(\"affected: {{}}\", affected);\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {parameters}) -> Result<u64, sqlx::Error> {{\n\
             sqlx::query({query:?}){binds}.execute(executor).await.map(|result| result.rows_affected())\n\
         }}\n\n"
    )
}
