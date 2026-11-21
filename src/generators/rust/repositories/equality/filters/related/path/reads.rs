use super::{keys, rows, sql, Names, Table};
use crate::database::introspection::ForeignKey;

/// Emits bound filters and explicit multi-table ordering for a verified join path.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    tables: [&Table; 3],
    edges: [&ForeignKey; 2],
    names: &Names,
    from: &str,
    bindings: &str,
) -> String {
    let aliases = ["source_row", "related_row", "parent_row"];
    let function = names.selector(
        tables[0],
        "related_path_ordered",
        &[edges[0].name.clone(), edges[1].name.clone()],
    );
    let projection = tables
        .iter()
        .zip(aliases)
        .map(|(table, alias)| rows::projection(table, alias))
        .collect::<Vec<_>>()
        .join(", ");
    let prefix = format!("SELECT {projection}{from}");
    let mut offset = 0;
    let decoded = tables
        .iter()
        .map(|table| {
            let decode = rows::decode(table, names, offset);
            offset += table.columns.len();
            decode
        })
        .collect::<Vec<_>>()
        .join(", ");
    let result = tables
        .iter()
        .map(|table| names.row(table))
        .collect::<Vec<_>>()
        .join(", ");
    let allowed = tables
        .iter()
        .zip(aliases)
        .enumerate()
        .flat_map(|(scope, (table, alias))| {
            keys(table).into_iter().map(move |key| {
                let column = &key[0];
                let qualified = format!("{}.{}", sql(alias), sql(column));
                format!("({scope}u8, {column:?}) => {qualified:?},")
            })
        })
        .collect::<Vec<_>>()
        .join("\n");
    let filters: Vec<_> = tables.iter().map(|table| names.filter(table)).collect();
    format!(
        "/// Reads a two-edge foreign-key path; ordering scopes 0/1/2 mean source/related/parent.\n\
         /// Example: `{function}(pool, {f0}::default(), {f1}::default(), {f2}::default(), &[], Some(1), None).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, row_filter: {f0}, related_filter: {f1}, parent_filter: {f2}, order_by: &[(u8, &str, bool, bool)], limit: Option<i64>, offset: Option<i64>) -> Result<Vec<({result})>, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             for (position, &(scope, column, ascending, nulls_first)) in order_by.iter().enumerate() {{\n\
                 let column = match (scope, column) {{\n{allowed}\n\
                     _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported path order column: {{scope}}:{{column}}\"))),\n\
                 }};\n\
                 query.push(if position == 0 {{ \" ORDER BY \" }} else {{ \", \" }}).push(column);\n\
                 query.push(if ascending {{ \" ASC\" }} else {{ \" DESC\" }});\n\
                 query.push(if nulls_first {{ \" NULLS FIRST\" }} else {{ \" NULLS LAST\" }});\n\
             }}\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}\n\
             query.build().fetch_all(executor).await?.into_iter().map(|record| Ok(({decoded}))).collect()\n\
         }}\n\n", f0 = filters[0], f1 = filters[1], f2 = filters[2]
    )
}
