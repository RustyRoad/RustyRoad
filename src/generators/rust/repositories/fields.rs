//! Ordered field fragments; e.g. omitted optional inputs emit neither names nor binds.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::inputs::optional;
use crate::database::introspection::{Column, Table};

/// Pairs insert fragments; e.g. `insert(&[name])` returns matching name/bind blocks.
pub(super) fn insert(table: &Table, columns: &[&Column], names: &Names) -> (String, String) {
    let (names, values): (Vec<_>, Vec<_>) = columns
        .iter()
        .map(|column| {
            (
                column_push(column, names.field(table, &column.name)),
                value_push(column, names.field(table, &column.name)),
            )
        })
        .unzip();
    (names.join("\n"), values.join("\n"))
}

/// Emits an insert name; e.g. `column_push(name)` guards optional `input.name`.
fn column_push(column: &Column, field: &str) -> String {
    let push = format!("columns.push({:?});", sql(&column.name));
    if optional(column) {
        format!("        if input.{field}.is_some() {{\n            {push}\n        }}")
    } else {
        format!("        {push}")
    }
}

/// Emits a matching bind; e.g. `value_push(name)` borrows `input.name` without moving it.
fn value_push(column: &Column, field: &str) -> String {
    if optional(column) {
        format!(
            "        if let Some(value) = input.{field}.as_ref() {{\n            values.push_bind(value);\n        }}"
        )
    } else {
        format!("        values.push_bind(&input.{field});")
    }
}

/// Emits a patch assignment; e.g. `assignment(name)` binds only `Some` inputs.
pub(super) fn assignment(table: &Table, column: &Column, names: &Names) -> String {
    assignment_from(table, column, "input", names)
}

/// Names the Patch binding; e.g. `assignment_from(name, "patch")` reads `patch.name`.
pub(super) fn assignment_from(
    table: &Table,
    column: &Column,
    input: &str,
    names: &Names,
) -> String {
    let field = names.field(table, &column.name);
    let assignment = format!("{} = ", sql(&column.name));
    format!(
        "        if let Some(value) = {input}.{field}.as_ref() {{\n\
             set.push({assignment:?}).push_bind_unseparated(value);\n\
         }}"
    )
}
