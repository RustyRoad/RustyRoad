//! Partial updates that explicitly preserve selected non-null stored values.
mod fields;
use super::{inputs, policy, sql, Names, Table};

/// Emits database-time updates with nullable-column COALESCE assignments.
/// Unselected fields retain normal patch semantics; identifiers are validated.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, key_type: &str, names: &Names) -> String {
    let clock = policy::clock(table).expect("parent selected an update clock");
    let function = names.function(table, "update_preserving");
    let model = names.row(table);
    let patch = names.patch(table);
    let field = names.field(table, &clock.name);
    let allowed = inputs::patch_columns(table)
        .iter()
        .filter(|column| column.nullable && column.name != clock.name)
        .map(|column| format!("{:?}", column.name))
        .collect::<Vec<_>>()
        .join(" | ");
    let valid = if allowed.is_empty() {
        "false".into()
    } else {
        format!("matches!(*column, {allowed})")
    };
    let assignments = inputs::patch_columns(table)
        .iter()
        .map(|column| fields::assignment(table, column, &clock.name, names))
        .collect::<Vec<_>>()
        .join("\n");
    let clock_sql = format!("{} = CURRENT_TIMESTAMP", sql(&clock.name));
    let prefix = format!("UPDATE {} SET ", sql(&table.name));
    let predicate = format!(" WHERE {} = ", sql(&table.primary_key[0]));
    format!(
        "/// Updates a row, preserving selected non-null values with COALESCE.\n\
         /// Only nullable non-clock columns may be selected; unknown names fail before SQL.\n\
         /// Omitted updated_at uses the database clock; other omitted fields remain unchanged.\n\
         /// Example: `{function}(pool, id, patch, &[]).await?` performs a normal clock update.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, id: {key_type}, input: {patch}, preserve_existing: &[&str]) -> Result<Option<{model}>, sqlx::Error> {{\n\
             for column in preserve_existing {{\n\
                 if !({valid}) {{\n\
                     return Err(sqlx::Error::Protocol(format!(\"Unsupported preserved column: {{column}}\")));\n\
                 }}\n\
             }}\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {{\n\
                 let mut set = query.separated(\", \");\n\
                 {assignments}\n\
                 if input.{field}.is_none() {{ set.push({clock_sql:?}); }}\n\
             }}\n\
             query.push({predicate:?}).push_bind(id).push(\" RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_optional(executor).await\n\
         }}\n\n"
    )
}
