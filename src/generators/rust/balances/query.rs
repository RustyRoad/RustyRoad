use super::{Balance, Names};
use crate::database::introspection::Schema;
use crate::generators::rust::{naming::sql, repositories::equality};

/// Emits one signed SUM with zero for no matching rows and transaction-clock expiry.
/// Negation stays inside the CASE, preserving PostgreSQL integer error behavior.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(schema: &Schema, names: &Names, function: &str, balance: &Balance) -> String {
    let table = schema.table(&balance.table).unwrap();
    let filter = names.filter(table);
    let bindings = equality::aggregate_bindings(table, names);
    let polarity = sql(&balance.polarity);
    let amount = sql(&balance.amount);
    let start = format!("SELECT COALESCE(SUM(CASE WHEN {polarity} = ");
    let middle = format!(" THEN {amount} WHEN {polarity} = ");
    let end = format!(
        " THEN -{amount} ELSE 0 END), 0) FROM {} WHERE TRUE",
        sql(&table.name)
    );
    let expiry = balance
        .expires
        .as_ref()
        .map(|column| {
            let column = sql(column);
            format!(
                "query.push({:?});",
                format!(" AND ({column} IS NULL OR {column} > CURRENT_TIMESTAMP)")
            )
        })
        .unwrap_or_default();
    format!(
        "/// Sums positive entries minus negative entries in one database statement.\n\
         /// Other labels contribute zero; no rows return zero. Database errors propagate.\n\
         /// Configured expiration retains NULL or strictly future timestamps.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, filter: {filter}) -> Result<(), sqlx::Error> {{\n\
         /// let balance = {function}(pool, filter).await?;\n\
         /// println!(\"balance: {{}}\", balance);\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}) -> Result<i64, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({start:?});\n\
             query.push_bind({positive:?}).push({middle:?}).push_bind({negative:?}).push({end:?});\n\
             {bindings}\n{expiry}\n\
             query.build_query_scalar::<i64>().fetch_one(executor).await\n\
         }}\n\n",
        positive = balance.positive, negative = balance.negative,
    )
}
