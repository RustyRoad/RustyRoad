//! Nullable related rows preserve source rows with no matching foreign-key target.
use super::{rows, sql, Names, Table};
use crate::database::introspection::ForeignKey;
use crate::generators::rust::repositories::equality::keys;

/// Emits one bounded left-join read ordered by a validated source-table column.
/// Related predicates apply in WHERE, so callers explicitly choose NULL behavior.
/// The join-key presence bit prevents decoding a missing target as a required row.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(
    table: &Table,
    target: &Table,
    foreign: &ForeignKey,
    names: &Names,
    from: &str,
    bindings: &str,
) -> String {
    let function = names.selector(
        table,
        "related_left_ordered",
        std::slice::from_ref(&foreign.name),
    );
    let source_filter = names.filter(table);
    let target_filter = names.filter(target);
    let source_row = names.row(table);
    let target_row = names.row(target);
    let marker = format!(
        "\"related_row\".{} IS NOT NULL",
        sql(&foreign.foreign_columns[0])
    );
    let prefix = format!(
        "SELECT {}, {}, {marker}{from}",
        rows::projection(table, "source_row"),
        rows::projection(target, "related_row")
    );
    let presence_index = table.columns.len() + target.columns.len();
    let source_decode = rows::decode(table, names, 0);
    let target_decode = rows::decode(target, names, table.columns.len());
    let order_columns = keys(table)
        .iter()
        .map(|columns| {
            let column = &columns[0];
            let qualified = format!("\"source_row\".{}", sql(column));
            format!("{column:?} => {qualified:?},")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let example_column = &keys(table)[0][0];
    format!(
        "/// Left-joins typed rows, retaining sources whose related row is absent.\n\
         /// Related predicates apply after the join; order columns belong to the source.\n\
         /// Example: `{function}(pool, {source_filter}::default(), {target_filter}::default(), {example_column:?}, true, None, None).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, row_filter: {source_filter}, related_filter: {target_filter}, order_by: &str, ascending: bool, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<({source_row}, Option<{target_row}>)>, sqlx::Error> {{\n\
             let order = match order_by {{\n{order_columns}\n\
                 _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported order column: {{order_by}}\"))),\n\
             }};\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             query.push(\" ORDER BY \").push(order);\n\
             query.push(if ascending {{ \" ASC NULLS LAST\" }} else {{ \" DESC NULLS FIRST\" }});\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}\n\
             query.build().fetch_all(executor).await?.into_iter().map(|record| {{\n\
                 let related = if sqlx::Row::try_get::<bool, _>(&record, {presence_index}usize)? {{\n\
                     Some({target_decode})\n\
                 }} else {{ None }};\n\
                 Ok(({source_decode}, related))\n\
             }}).collect()\n\
         }}\n\n"
    )
}
