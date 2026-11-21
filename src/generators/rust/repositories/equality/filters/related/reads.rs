//! Bound first/list/count reads over one verified foreign-key relationship.
use super::{rows, Names, Table};
use crate::database::introspection::ForeignKey;

/// Emits one-statement relationship operations without imposing an ordering.
/// Source and parent predicates share the same PostgreSQL statement snapshot.
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
    let joined = names.joined(table, foreign);
    let source_filter = names.filter(table);
    let target_filter = names.filter(target);
    let projection = format!(
        "{}, {}",
        rows::projection(table, "source_row"),
        rows::projection(target, "related_row")
    );
    let decode = format!(
        "{joined} {{ row: {}, related: {} }}",
        rows::decode(table, names, 0),
        rows::decode(target, names, table.columns.len())
    );
    let list_finish = format!(
        "query.build().fetch_all(executor).await?.into_iter().map(|record| Ok({decode})).collect()"
    );
    let first_finish = format!(
        "query.build().fetch_optional(executor).await?.map(|record| Ok({decode})).transpose()"
    );
    let mut output = String::new();
    for (verb, result, select, limit, finish) in [
        (
            "related_first",
            format!("Option<{joined}>"),
            projection.as_str(),
            "query.push(\" LIMIT 1\");",
            first_finish.as_str(),
        ),
        (
            "related_list",
            format!("Vec<{joined}>"),
            projection.as_str(),
            "",
            list_finish.as_str(),
        ),
        (
            "related_count",
            "i64".into(),
            "COUNT(*)",
            "",
            "query.build_query_scalar::<i64>().fetch_one(executor).await",
        ),
    ] {
        let function = names.selector(table, verb, std::slice::from_ref(&foreign.name));
        let prefix = format!("SELECT {select}{from}");
        output.push_str(&format!(
            "/// Applies typed source and related filters to an introspected inner join.\n\
             /// Example: `{function}(pool, {source_filter}::default(), {target_filter}::default()).await?`.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, row_filter: {source_filter}, related_filter: {target_filter}) -> Result<{result}, sqlx::Error> {{\n\
                 let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
                 {bindings}\n{limit}\n\
                 {finish}\n\
             }}\n\n"
        ));
    }
    output
}
