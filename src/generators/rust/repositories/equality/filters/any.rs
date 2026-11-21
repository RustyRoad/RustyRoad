//! Disjunctions of typed equality groups preserve single-statement identity reads.
use super::{sql, Names, Table};

/// Emits OR-group reads without introducing uniqueness or implicit ordering.
/// An empty group list matches no rows; an empty individual filter matches all.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names, bindings: &str) -> String {
    let model = names.row(table);
    let filter = names.filter(table);
    let mut output = String::new();
    for (verb, result, fetch, limit, projection, builder) in [
        (
            "filter_any_first",
            format!("Option<{model}>"),
            "fetch_optional",
            "query.push(\" LIMIT 1\");",
            "*",
            format!("build_query_as::<{model}>()"),
        ),
        (
            "filter_any_list",
            format!("Vec<{model}>"),
            "fetch_all",
            "",
            "*",
            format!("build_query_as::<{model}>()"),
        ),
        (
            "filter_any_count",
            "i64".into(),
            "fetch_one",
            "",
            "COUNT(*)",
            "build_query_scalar::<i64>()".into(),
        ),
    ] {
        let function = names.function(table, verb);
        let prefix = format!("SELECT {projection} FROM {} WHERE FALSE", sql(&table.name));
        output.push_str(&format!(
            "/// Reads typed OR groups; fields within each filter are joined with AND.\n\
             /// Empty groups match no rows; an empty filter matches all rows. Order is unspecified.\n\
             /// Example: `{function}(pool, Vec::<{filter}>::new()).await?`.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, groups: Vec<{filter}>) -> Result<{result}, sqlx::Error> {{\n\
                 let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
                 for filter in groups {{\n\
                     query.push(\" OR (TRUE\");\n\
                     {bindings}\n\
                     query.push(\")\");\n\
                 }}\n\
                 {limit}\n\
                 query.{builder}.{fetch}(executor).await\n\
             }}\n\n"
        ));
    }
    output
}
