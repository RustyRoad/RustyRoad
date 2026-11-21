//! Atomic selective updates through verified unique selectors.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::{patch, selector, update_key_body};
use crate::database::introspection::{Column, Schema, Table};

/// Emit an ordinary update, or an explicit timestamp-touch variant.
/// Missing rows return SQLx RowNotFound. Ordinary empty patches only read;
/// touch variants update the clock even when no other field is supplied.
/// Generated operations retain the supplied pool or transaction connection.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(
    table: &Table,
    schema: &Schema,
    key: &[String],
    names: &Names,
    clock: Option<&Column>,
) -> (String, bool) {
    let mode = if clock.is_some() {
        "update_touch"
    } else {
        "update"
    };
    let function = names.selector(table, mode, key);
    let model = names.row(table);
    let mut patch = patch::render(table, "input", names);
    if let Some(clock) = clock {
        let field = names.field(table, &clock.name);
        let touch = format!("{} = CURRENT_TIMESTAMP", sql(&clock.name));
        patch.assignments.push_str(&format!(
            "\n        if input.{field}.is_none() {{ set.push({touch:?}); }}"
        ));
    }
    let selector::Selector {
        parameters,
        predicate,
        binds,
        arguments,
    } = selector::update(table, schema, key, names);
    let select = format!(
        "SELECT * FROM {} WHERE {predicate} LIMIT 1",
        sql(&table.name)
    );
    let lookup =
        format!("sqlx::query_as::<_, {model}>({select:?}){binds}.fetch_one(executor).await");
    let (body, builder) =
        update_key_body::render(table, key, model, &patch, &lookup, clock.is_some());
    let clock_doc = if clock.is_some() {
        "/// Omitted updated_at uses transaction time; explicit values remain explicit.\n"
    } else {
        ""
    };
    let output = format!(
        "/// Selective update; e.g. `{function}(pool, {arguments}, input).await`.\n\
         /// Unspecified fields remain unchanged; missing rows return SQLx RowNotFound.\n\
         {clock_doc}\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {parameters}, {parameter}) -> Result<{model}, sqlx::Error> {{\n\
         {body}\n\
         }}\n\n",
        parameter = patch.parameter,
    );
    (output, builder)
}
