//! Age-based deletion keeps cutoff arithmetic on the PostgreSQL clock.
use super::{sql, Names, Table};
use crate::database::introspection::Column;

/// Generate a bounded timestamp deletion with optional typed equality predicates.
/// Views are excluded; the caller owns transaction commit and rollback.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, names: &Names, columns: &[&Column], bindings: &str) -> String {
    if table.view {
        return String::new();
    }
    let clocks = columns
        .iter()
        .filter(|column| timestamp(&column.sql_type))
        .map(|column| format!("{:?} => {:?},", column.name, sql(&column.name)))
        .collect::<Vec<_>>();
    if clocks.is_empty() {
        return String::new();
    }
    let clocks = clocks.join("\n");
    let function = names.function(table, "filter_delete_older_than_days");
    let filter = names.filter(table);
    let prefix = format!("DELETE FROM {} WHERE TRUE", sql(&table.name));
    format!(
        "/// Deletes rows strictly older than PostgreSQL transaction time minus the given days.\n\
         /// Equality predicates are optional; a validated timestamp cutoff is always required.\n\
         /// Negative and zero ages retain PostgreSQL interval semantics; SQL NULL does not match.\n\
         /// Returns affected rows without decoding a row or committing the executor.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, filter: {filter}, timestamp_column: &str) -> Result<(), sqlx::Error> {{\n\
         /// let deleted = {function}(pool, filter, timestamp_column, 30).await?;\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}, timestamp_column: &str, days: i32) -> Result<u64, sqlx::Error> {{\n\
             let timestamp_column = match timestamp_column {{\n{clocks}\n\
                 _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported timestamp column: {{timestamp_column}}\"))),\n\
             }};\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             query.push(\" AND \").push(timestamp_column);\n\
             query.push(\" < CURRENT_TIMESTAMP - INTERVAL '1 day' * \").push_bind(days);\n\
             Ok(query.build().execute(executor).await?.rows_affected())\n\
         }}\n\n"
    )
}

/// Accept scalar timestamps with optional precision, excluding arrays and domains.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn timestamp(sql_type: &str) -> bool {
    let value = sql_type.trim().to_ascii_lowercase();
    let value = if let Some((base, rest)) = value.split_once('(') {
        let Some((precision, suffix)) = rest.split_once(')') else {
            return false;
        };
        if precision
            .trim()
            .parse::<u8>()
            .map_or(true, |value| value > 6)
        {
            return false;
        }
        format!("{base}{suffix}")
    } else {
        value
    };
    matches!(
        value.as_str(),
        "timestamp" | "timestamptz" | "timestamp without time zone" | "timestamp with time zone"
    )
}
