//! Grouping constraint rows by their constraint or index name.

use super::super::assemble::{FkParts, Grouped, IndexParts};
use super::rows::{action, text};
use sqlx::postgres::PgRow;
use std::collections::BTreeMap;

/// Groups enum rows into their types, preserving declaration order.
pub(super) fn enums(rows: Vec<PgRow>) -> Vec<crate::database::introspection::Enum> {
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in rows {
        grouped
            .entry(text(&row, "name"))
            .or_default()
            .push(text(&row, "value"));
    }

    grouped
        .into_iter()
        .map(|(name, values)| crate::database::introspection::Enum { name, values })
        .collect()
}

/// Groups constraint rows by name, preserving column order.
pub(super) fn constraints(rows: Vec<PgRow>) -> Grouped {
    let mut grouped: Grouped = BTreeMap::new();
    for row in rows {
        let entry = grouped
            .entry(text(&row, "name"))
            .or_insert_with(|| (text(&row, "table_name"), Vec::new()));
        entry.1.push(text(&row, "column_name"));
    }
    grouped
}

/// Groups index rows by index name.
pub(super) fn indexes(rows: Vec<PgRow>) -> BTreeMap<String, IndexParts> {
    let mut grouped: BTreeMap<String, IndexParts> = BTreeMap::new();
    for row in rows {
        let entry = grouped.entry(text(&row, "name")).or_default();
        entry.table = text(&row, "table_name");
        entry.unique = sqlx::Row::try_get(&row, "unique").unwrap_or(false);
        entry.columns.push(text(&row, "column_name"));
    }
    grouped
}

/// Groups foreign key rows by constraint name.
pub(super) fn foreign_keys(rows: Vec<PgRow>) -> BTreeMap<String, FkParts> {
    let mut grouped: BTreeMap<String, FkParts> = BTreeMap::new();
    for row in rows {
        let entry = grouped.entry(text(&row, "name")).or_default();
        entry.table = text(&row, "table_name");
        entry.foreign_table = text(&row, "foreign_table");
        entry.columns.push(text(&row, "column_name"));
        entry.foreign_columns.push(text(&row, "foreign_column"));
        entry.on_delete = action(&row, "on_delete");
        entry.on_update = action(&row, "on_update");
    }
    grouped
}
