//! Builds the logical schema model from raw Postgres column rows.

use super::super::model::{Column, Schema, Table};
use super::super::naming::{is_internal, logical_name};
use super::rules::{self, Renames};

/// One raw column row read from `information_schema`.
pub(super) struct Row {
    pub table: String,
    pub column: String,
    pub default: Option<String>,
}

/// Assembles rows into the logical view of each table.
pub(super) fn build(rows: Vec<Row>, renames: &Renames) -> Schema {
    let mut tables: Vec<Table> = Vec::new();

    for row in rows {
        if !rules::is_visible(&row) {
            continue;
        }

        let logical = rules::renamed(&row.table, logical_name(&row.column), renames);
        let mut column = Column::new(logical).backed_by(&row.column);
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
