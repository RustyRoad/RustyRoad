//! Attaching constraints to their tables.

use super::super::model::{ForeignKey, Index, Schema, Unique};
use super::parts::{FkParts, Grouped, IndexParts};
use std::collections::BTreeMap;

/// Attaches primary key columns to their tables.
pub(crate) fn primary_keys(schema: &mut Schema, rows: Vec<(String, String)>) {
    for (table_name, column) in rows {
        if let Some(table) = schema.tables.iter_mut().find(|t| t.name == table_name) {
            table.primary_key.push(column);
        }
    }
}

/// Attaches unique constraints, grouped by constraint name.
pub(crate) fn uniques(schema: &mut Schema, grouped: Grouped) {
    for (name, (table_name, columns)) in grouped {
        if let Some(table) = schema.tables.iter_mut().find(|t| t.name == table_name) {
            table.uniques.push(Unique { name, columns });
        }
    }
}

/// Attaches indexes, grouped by index name.
pub(crate) fn indexes(schema: &mut Schema, grouped: BTreeMap<String, IndexParts>) {
    for (name, parts) in grouped {
        if let Some(table) = schema.tables.iter_mut().find(|t| t.name == parts.table) {
            table.indexes.push(Index {
                name,
                columns: parts.columns,
                unique: parts.unique,
            });
        }
    }
}

/// Attaches foreign keys, grouped by constraint name.
pub(crate) fn foreign_keys(schema: &mut Schema, grouped: BTreeMap<String, FkParts>) {
    for (name, parts) in grouped {
        if let Some(table) = schema.tables.iter_mut().find(|t| t.name == parts.table) {
            table.foreign_keys.push(ForeignKey {
                name,
                columns: parts.columns,
                foreign_table: parts.foreign_table,
                foreign_columns: parts.foreign_columns,
                on_delete: parts.on_delete,
                on_update: parts.on_update,
            });
        }
    }
}
