//! Shared typed equality selectors for read, update and delete operations.

use super::super::allocation::Names;
use super::super::{naming::sql, types};
use crate::database::introspection::{Schema, Table};

/// Bound fragments; e.g. a text token produces `"token" = $1` and `.bind(value_1)`.
pub(super) struct Selector {
    pub parameters: String,
    pub predicate: String,
    pub binds: String,
    pub arguments: String,
}

/// Derives read/delete arguments; e.g. `render(table, schema, &["token".into()])`.
pub(super) fn render(table: &Table, schema: &Schema, key: &[String], names: &Names) -> Selector {
    build(table, schema, key, names, |position| {
        format!("value_{position}")
    })
}

/// Names update selectors; e.g. one text column yields `key: String`.
pub(super) fn update(table: &Table, schema: &Schema, key: &[String], names: &Names) -> Selector {
    build(table, schema, key, names, |position| {
        key_argument(position, key.len())
    })
}

/// Names a positional key; e.g. `key_argument(1, 1)` is `key`, not `key_1`.
pub(super) fn key_argument(position: usize, count: usize) -> String {
    if count == 1 {
        "key".to_string()
    } else {
        format!("key_{position}")
    }
}

/// Builds typed bindings; e.g. `build(table, schema, key, |i| format!("key_{i}"))`.
fn build(
    table: &Table,
    schema: &Schema,
    key: &[String],
    names: &Names,
    name: impl Fn(usize) -> String,
) -> Selector {
    let mut parameters = Vec::new();
    let mut predicates = Vec::new();
    let mut arguments = Vec::new();
    let mut binds = String::new();
    for (index, column_name) in key.iter().enumerate() {
        let mut column = table
            .column(column_name)
            .expect("known selector column")
            .clone();
        column.nullable = false;
        let position = index + 1;
        let argument = name(position);
        parameters.push(format!(
            "{argument}: {}",
            types::map_column(table, &column, schema, names).rust
        ));
        predicates.push(format!("{} = ${position}", sql(column_name)));
        binds.push_str(&format!(".bind({argument})"));
        arguments.push(argument);
    }
    Selector {
        parameters: parameters.join(", "),
        predicate: predicates.join(" AND "),
        binds,
        arguments: arguments.join(", "),
    }
}
