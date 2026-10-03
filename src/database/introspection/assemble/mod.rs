//! Assembling introspected rows into the schema model.

pub(crate) mod attach;
mod parts;

use super::model::{Column, Table};

pub(crate) use parts::{action, FkParts, Grouped, IndexParts};

/// Builds tables from column rows, in catalog order.
pub(crate) fn tables(rows: Vec<(String, Column)>) -> Vec<Table> {
    let mut tables: Vec<Table> = Vec::new();

    for (table_name, column) in rows {
        match tables.iter_mut().find(|table| table.name == table_name) {
            Some(table) => table.columns.push(column),
            None => tables.push(Table {
                name: table_name,
                columns: vec![column],
                ..Table::default()
            }),
        }
    }

    tables
}
