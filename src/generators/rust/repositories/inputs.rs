//! Input shapes; e.g. auto-ID-only tables have empty New and Patch inputs.

use super::super::allocation::Names;
use crate::database::introspection::{Column, Table};

/// Mirrors New fields, excluding auto-increment identities.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn new_columns(table: &Table) -> Vec<&Column> {
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

/// Recognizes omittable nullable or defaulted inputs.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn optional(column: &Column) -> bool {
    column.nullable || column.default.is_some()
}

/// Joins input-field presence checks for insertion branches.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn present(
    table: &Table,
    columns: &[&Column],
    names: &Names,
) -> String {
    present_from(table, columns, "input", names)
}

/// Names the input; e.g. `present_from(&[name], "patch")` checks `patch.name`.
pub(super) fn present_from(
    table: &Table,
    columns: &[&Column],
    input: &str,
    names: &Names,
) -> String {
    columns
        .iter()
        .map(|column| format!("{input}.{}.is_some()", names.field(table, &column.name)))
        .collect::<Vec<_>>()
        .join(" || ")
}
