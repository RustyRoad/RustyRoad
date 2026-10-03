//! Ordered field fragments; e.g. omitted optional inputs emit neither names nor binds.

use super::super::naming::{snake, sql};
use super::inputs::optional;
use crate::database::introspection::Column;

/// Pairs insert fragments; e.g. `insert(&[name])` returns matching name/bind blocks.
pub(super) fn insert(columns: &[&Column]) -> (String, String) {
    let (names, values): (Vec<_>, Vec<_>) = columns
        .iter()
        .map(|column| (column_push(column), value_push(column)))
        .unzip();
    (names.join("\n"), values.join("\n"))
}

/// Emits an insert name; e.g. `column_push(name)` guards optional `input.name`.
fn column_push(column: &Column) -> String {
    let field = snake(&column.name);
    let push = format!("columns.push({:?});", sql(&column.name));
    if optional(column) {
        format!("        if input.{field}.is_some() {{\n            {push}\n        }}")
    } else {
        format!("        {push}")
    }
}

/// Emits a matching bind; e.g. `value_push(name)` borrows `input.name` without moving it.
fn value_push(column: &Column) -> String {
    let field = snake(&column.name);
    if optional(column) {
        format!(
            "        if let Some(value) = input.{field}.as_ref() {{\n            values.push_bind(value);\n        }}"
        )
    } else {
        format!("        values.push_bind(&input.{field});")
    }
}

/// Emits a patch assignment; e.g. `assignment(name)` binds only `Some` inputs.
pub(super) fn assignment(column: &Column) -> String {
    assignment_from(column, "input")
}

/// Names the Patch binding; e.g. `assignment_from(name, "patch")` reads `patch.name`.
pub(super) fn assignment_from(column: &Column, input: &str) -> String {
    let field = snake(&column.name);
    let assignment = format!("{} = ", sql(&column.name));
    format!(
        "        if let Some(value) = {input}.{field}.as_ref() {{\n\
             set.push({assignment:?}).push_bind_unseparated(value);\n\
         }}"
    )
}