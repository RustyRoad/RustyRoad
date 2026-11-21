//! Atomic numeric increments preserve database arithmetic and NULL behavior.
use super::super::super::{allocation::Names, naming::sql, types};
use super::super::policy;
use crate::database::introspection::{Column, Schema, Table};

/// Finds mutable numeric columns on tables with a simple primary key.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn columns(table: &Table) -> Vec<&Column> {
    if table.view || !table.has_simple_key() {
        return Vec::new();
    }
    table
        .columns
        .iter()
        .filter(|column| {
            let declared = column.sql_type.to_ascii_lowercase();
            let base = declared.split('(').next().unwrap_or("").trim();
            !column.auto_increment
                && !table.primary_key.contains(&column.name)
                && matches!(
                    base,
                    "smallint"
                        | "int2"
                        | "integer"
                        | "int4"
                        | "bigint"
                        | "int8"
                        | "real"
                        | "float4"
                        | "double precision"
                        | "float8"
                        | "numeric"
                        | "decimal"
                )
        })
        .collect()
}

/// Renders bound numeric increments with an optional database timestamp touch.
/// SQL NULL remains NULL and PostgreSQL enforces arithmetic overflow and constraints.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn render(
    table: &Table,
    schema: &Schema,
    key_type: &str,
    names: &Names,
) -> String {
    let model = names.row(table);
    columns(table).into_iter().map(|column| {
        let function = names.selector(table, "increment", std::slice::from_ref(&column.name));
        let mut operand = (*column).clone();
        operand.nullable = false;
        let operand_type = types::map_column(table, &operand, schema, names).rust;
        let target = sql(&column.name);
        let prefix = format!("UPDATE {} SET {target} = {target} + $2", sql(&table.name));
        let suffix = format!(" WHERE {} = $1 RETURNING *", sql(&table.primary_key[0]));
        let plain = format!("{prefix}{suffix}");
        let touch = policy::clock(table).map(|clock| {
            format!("{:?}", format!("{prefix}, {} = CURRENT_TIMESTAMP{suffix}", sql(&clock.name)))
        }).unwrap_or_else(|| {
            "return Err(sqlx::Error::Protocol(\"Table has no eligible updated_at column\".into()))".into()
        });
        format!(
            "/// Atomically adds a typed delta to `{column_name}`; NULL remains NULL.\n\
             /// Example: `{function}(&mut *transaction, id, delta, false).await?`.\n\
             /// Missing rows return None; database arithmetic failures propagate unchanged.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, id: {key_type}, delta: {operand_type}, touch_updated_at: bool) -> Result<Option<{model}>, sqlx::Error> {{\n\
                 let statement = if touch_updated_at {{ {touch} }} else {{ {plain:?} }};\n\
                 sqlx::query_as::<_, {model}>(statement).bind(id).bind(delta).fetch_optional(executor).await\n\
             }}\n\n",
            column_name = column.name,
        )
    }).collect()
}
