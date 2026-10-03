//! Atomic selective updates through verified unique selectors.

use super::super::naming::{pascal, snake, sql};
use super::{patch, selector, targets, where_key};
use crate::database::introspection::{Schema, Table};

/// Emits `update_sessions_by_session_token(pool, key, input)` with a generated Patch.
/// Missing rows return SQLx RowNotFound; an empty patch only looks up the row.
pub(super) fn render(table: &Table, schema: &Schema, key: &[String]) -> (String, bool) {
    let function = format!("update_{}_by_{}", snake(&table.name), targets::suffix(key));
    let model = pascal(&table.name);
    let patch::Patch { parameter, changed, assignments } = patch::render(table, "input");
    let selector::Selector { parameters, predicate, binds, arguments } = selector::update(table, schema, key);
    let select = format!("SELECT * FROM {} WHERE {predicate} LIMIT 1", sql(&table.name));
    let lookup = format!("sqlx::query_as::<_, {model}>({select:?}){binds}.fetch_one(pool).await");
    let builder = !assignments.is_empty();
    let body = if !builder {
        lookup
    } else {
        let prefix = format!("UPDATE {} SET ", sql(&table.name));
        let filter = where_key::render(key);
        format!(
            "    if !({changed}) {{ return {lookup}; }}\n\
                 let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
                 {{\n\
                     let mut set = query.separated(\", \");\n\
             {assignments}\n\
                 }}\n\
             {filter}\
                 query.push(\" RETURNING *\");\n\
                 query.build_query_as::<{model}>().fetch_one(pool).await"
        )
    };
    let output = format!(
        "/// Selective update; e.g. `{function}(pool, {arguments}, input).await`.\n\
         /// Unspecified fields remain unchanged; missing rows return SQLx RowNotFound.\n\
         pub async fn {function}(pool: &PgPool, {parameters}, {parameter}) -> Result<{model}, sqlx::Error> {{\n\
         {body}\n\
         }}\n\n"
    );
    (output, builder)
}