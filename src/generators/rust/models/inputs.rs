use super::super::{allocation::Names, types};
use crate::database::introspection::{Column, Schema, Table};

/// Keep generated omission and explicit-null semantics for create inputs.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn create_type(
    table: &Table,
    column: &Column,
    schema: &Schema,
    names: &Names,
) -> String {
    let base = base_type(table, column, schema, names);
    if column.nullable && column.default.is_some() {
        format!("Option<Option<{base}>>")
    } else if is_optional_input(column) {
        format!("Option<{base}>")
    } else {
        base
    }
}

/// Keep generated partial-update nullability semantics.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn patch_type(table: &Table, column: &Column, schema: &Schema, names: &Names) -> String {
    let base = base_type(table, column, schema, names);
    if column.nullable {
        // Outer Option means "field omitted"; inner Option means JSON null.
        format!("Option<Option<{base}>>")
    } else {
        format!("Option<{base}>")
    }
}

/// Classify fields whose create input may be omitted.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn is_optional_input(column: &Column) -> bool {
    column.nullable || column.default.is_some() || column.auto_increment
}

/// Resolve the non-null Rust codec for a column.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn base_type(table: &Table, column: &Column, schema: &Schema, names: &Names) -> String {
    let mut required = column.clone();
    required.nullable = false;
    types::map_column(table, &required, schema, names).rust
}
