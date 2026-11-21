//! Atomic boolean inversion derived from catalog columns.
use super::super::super::{allocation::Names, naming::sql};
use super::super::policy;
use crate::database::introspection::{Column, Table};

/// Finds mutable boolean columns on tables with a simple primary key.
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
            matches!(
                column.sql_type.trim().to_ascii_lowercase().as_str(),
                "bool" | "boolean"
            ) && !column.auto_increment
                && !table.primary_key.contains(&column.name)
        })
        .collect()
}

/// Renders one atomic toggle per eligible column; NOT preserves SQL NULL.
/// Timestamp touching is explicit and requires an eligible updated_at column.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn render(table: &Table, key_type: &str, names: &Names) -> String {
    let model = names.row(table);
    columns(table).into_iter().map(|column| {
        let function = names.selector(table, "toggle", std::slice::from_ref(&column.name));
        let target = sql(&column.name);
        let prefix = format!("UPDATE {} SET {target} = NOT {target}", sql(&table.name));
        let suffix = format!(" WHERE {} = $1 RETURNING *", sql(&table.primary_key[0]));
        let plain = format!("{prefix}{suffix}");
        let touch = policy::clock(table).map(|clock| {
            format!("{:?}", format!("{prefix}, {} = CURRENT_TIMESTAMP{suffix}", sql(&clock.name)))
        }).unwrap_or_else(|| {
            "return Err(sqlx::Error::Protocol(\"Table has no eligible updated_at column\".into()))".into()
        });
        format!(
            "/// Inverts `{column_name}` in one statement; NULL remains NULL.\n\
             /// Example: `{function}(pool, id, false).await?`; true also touches updated_at.\n\
             pub async fn {function}(pool: &PgPool, id: {key_type}, touch_updated_at: bool) -> Result<Option<{model}>, sqlx::Error> {{\n\
                 let statement = if touch_updated_at {{ {touch} }} else {{ {plain:?} }};\n\
                 sqlx::query_as::<_, {model}>(statement).bind(id).fetch_optional(pool).await\n\
             }}\n\n",
            column_name = column.name,
        )
    }).collect()
}
