//! Input shapes; e.g. auto-ID-only tables have empty New and Patch inputs.

use super::super::naming::snake;
use crate::database::introspection::{Column, Table};

/// Mirrors New fields; e.g. `new_columns(table)` omits an auto-increment `id`.
pub(super) fn new_columns(table: &Table) -> Vec<&Column> {
    table
        .columns
        .iter()
        .filter(|column| !column.auto_increment)
        .collect()
}

/// Mirrors Patch fields; e.g. `patch_columns(table)` omits even a manual primary key.
pub(super) fn patch_columns(table: &Table) -> Vec<&Column> {
    table
        .columns
        .iter()
        .filter(|column| !table.primary_key.contains(&column.name))
        .collect()
}

/// Recognizes omission; e.g. `optional(column)` is true for a defaulted timestamp.
pub(super) fn optional(column: &Column) -> bool {
    column.nullable || column.default.is_some()
}

/// Joins presence checks; e.g. `present(&[name])` emits `input.name.is_some()`.
pub(super) fn present(columns: &[&Column]) -> String {
    present_from(columns, "input")
}

/// Names the input; e.g. `present_from(&[name], "patch")` checks `patch.name`.
pub(super) fn present_from(columns: &[&Column], input: &str) -> String {
    columns
        .iter()
        .map(|column| format!("{input}.{}.is_some()", snake(&column.name)))
        .collect::<Vec<_>>()
        .join(" || ")
}