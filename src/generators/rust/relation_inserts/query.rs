use super::{Names, RelationInsert};
use crate::database::introspection::Schema;
use crate::generators::rust::{naming::sql, types};

/// Emits INSERT SELECT with a scalar lowest-ID subquery and NOT EXISTS guard.
/// No ON CONFLICT clause hides a concurrent constraint failure.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    report: &RelationInsert,
) -> String {
    let target = schema
        .tables
        .iter()
        .find(|t| t.name == report.table)
        .unwrap();
    let source = schema
        .tables
        .iter()
        .find(|t| t.name == report.source_table)
        .unwrap();
    let mut input = target.column(&report.input_column).unwrap().clone();
    input.nullable = false;
    let mut source_id = source.column(&report.source_key).unwrap().clone();
    source_id.nullable = false;
    let input_type = types::map_column(target, &input, schema, names).rust;
    let source_type = types::map_column(source, &source_id, schema, names).rust;
    let table = sql(&report.table);
    let input_column = sql(&report.input_column);
    let source_key = sql(&report.source_key);
    let source_target = sql(&report.source_target);
    let optional_key = sql(&report.optional_key);
    let eligible = if report.eligible_value {
        "TRUE"
    } else {
        "FALSE"
    };
    let query = format!(
        "INSERT INTO {table} ({input_column}, {}, {source_target}) \
         SELECT $1, (SELECT {optional_key} FROM {} AS optional_row \
         WHERE optional_row.{} = source_row.{source_key} ORDER BY {optional_key} ASC LIMIT 1), source_row.{source_key} \
         FROM {} AS source_row WHERE source_row.{source_key} = $2 AND source_row.{} = {eligible} \
         AND NOT EXISTS (SELECT 1 FROM {table} AS existing WHERE existing.{input_column} = $1 \
         AND existing.{source_target} = source_row.{source_key})",
        sql(&report.optional_target), sql(&report.optional_table), sql(&report.optional_source),
        sql(&report.source_table), sql(&report.eligible_column),
    );
    format!(
        "/// Inserts an eligible relationship only when the same input/source pair is absent.\n\
         /// Returns false for absent/ineligible/already-linked source rows; database errors propagate.\n\
         /// Example: `{function}(pool, input_id, source_id).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, input_id: {input_type}, source_id: {source_type}) -> Result<bool, sqlx::Error> {{\n\
             sqlx::query({query:?}).bind(input_id).bind(source_id).execute(executor).await\n\
                 .map(|result| result.rows_affected() > 0)\n\
         }}\n\n"
    )
}
