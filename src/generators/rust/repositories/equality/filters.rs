//! Typed multi-column conjunctions independent of indexes and uniqueness.
mod clock;
mod clock_delete;
mod comparisons;
mod delete;
mod distinct;
pub(in crate::generators::rust::repositories::equality) mod fields;
mod ordered;
mod ordered_columns;
mod update;
pub(in crate::generators::rust) use comparisons::comparison_bindings;
mod any;
mod locked;
pub(super) mod related;
use super::{keys, sql, Names, Schema, Table};

/// Generate typed equality filters and matching read/delete/update operations.
/// Omitted fields add no predicate; nullable Some(None) adds IS NULL. Empty
/// filters are explicit unfiltered reads. All supplied values remain bound.
/// Generated reads reuse the supplied pool or transaction connection and never commit it.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(
    table: &Table,
    schema: &Schema,
    names: &Names,
    relationship_reads: bool,
) -> String {
    let columns: Vec<_> = keys(table)
        .iter()
        .map(|key| {
            table
                .column(&key[0])
                .expect("supported column belongs to table")
        })
        .collect();
    if columns.is_empty() {
        return String::new();
    }
    let filter = names.filter(table);
    let model = names.row(table);
    let declarations = fields::declarations(table, schema, &columns, names);
    let bindings = fields::bindings(table, &columns, names);
    let mut output = format!(
        "/// Typed equality filter. None omits a field; Some(None) matches nullable SQL NULL.\n\
         #[derive(Debug, Clone, Default)]\n\
         pub struct {filter} {{\n{declarations}\n}}\n\n"
    );
    let row_builder = format!("build_query_as::<{model}>()");
    for (verb, result, fetch, limit, projection, builder) in [
        (
            "filter_first",
            format!("Option<{model}>"),
            "fetch_optional",
            "query.push(\" LIMIT 1\");",
            "*",
            row_builder.clone(),
        ),
        (
            "filter_list",
            format!("Vec<{model}>"),
            "fetch_all",
            "",
            "*",
            row_builder,
        ),
        (
            "filter_count",
            "i64".into(),
            "fetch_one",
            "",
            "COUNT(*)",
            "build_query_scalar::<i64>()".into(),
        ),
    ] {
        let function = names.function(table, verb);
        let prefix = format!("SELECT {projection} FROM {} WHERE TRUE", sql(&table.name));
        output.push_str(&format!(
            "/// Apply typed equality predicates; e.g. `{function}(pool, {filter}::default()).await`.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}) -> Result<{result}, sqlx::Error> {{\n\
                 let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
                 {bindings}\n{limit}\n\
                 query.{builder}.{fetch}(executor).await\n\
             }}\n\n"
        ));
    }
    output.push_str(&ordered::render(table, names, &bindings));
    output.push_str(&ordered_columns::render(table, names, &bindings));
    output.push_str(&distinct::render(table, names, &bindings));
    output.push_str(&clock::render(table, names, &columns, &bindings));
    output.push_str(&delete::render(table, names, &columns, &bindings));
    output.push_str(&clock_delete::render(table, names, &columns, &bindings));
    output.push_str(&update::render(table, names, &columns, &bindings));
    output.push_str(&comparisons::render(
        table, schema, names, &columns, &bindings,
    ));
    output.push_str(&any::render(table, names, &bindings));
    output.push_str(&locked::render(table, names, &bindings));
    if relationship_reads {
        output.push_str(&related::render(table, schema, names));
    }
    output
}
