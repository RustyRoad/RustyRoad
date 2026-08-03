//! Builds the logical schema model from raw Postgres column rows.

use super::super::model::{Column, Schema, Table};
use super::super::naming::{is_internal, logical_name, DELETION_PREFIX};

/// One raw column row read from `information_schema`.
pub(super) struct Row {
    pub table: String,
    pub column: String,
    pub default: Option<String>,
}

/// Assembles rows into the logical view of each table.
pub(super) fn build(rows: Vec<Row>) -> Schema {
    let mut tables: Vec<Table> = Vec::new();

    for row in rows {
        if row.table.is_empty() || row.column.is_empty() {
            continue;
        }
        // A column pending removal is absent from the current version.
        if row.column.starts_with(DELETION_PREFIX) {
            continue;
        }

        let mut column = Column::new(logical_name(&row.column)).backed_by(&row.column);
        if let Some(default) = row.default {
            column = column.with_default(default);
        }

        match tables.iter_mut().find(|t| t.name == row.table) {
            Some(table) => table.columns.push(column),
            None => tables.push(Table::new(row.table, vec![column])),
        }
    }

    for table in &mut tables {
        drop_shadowed(table);
    }

    Schema { tables }
}

/// Drops originals shadowed by an in-flight replacement of the same logical name.
fn drop_shadowed(table: &mut Table) {
    let shadowed: Vec<String> = table
        .columns
        .iter()
        .filter(|c| is_internal(&c.physical_name))
        .map(|c| c.name.clone())
        .collect();

    table
        .columns
        .retain(|c| is_internal(&c.physical_name) || !shadowed.contains(&c.name));
}
