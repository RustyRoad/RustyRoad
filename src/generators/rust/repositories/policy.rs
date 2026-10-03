//! Reusable audit-column conventions, not application table/DTO definitions.

use crate::database::introspection::{Column, Table};

/// Selects the transaction-clock column; e.g. a timestamp `updated_at` is eligible.
pub(super) fn clock(table: &Table) -> Option<&Column> {
    table.column("updated_at").filter(|column| {
        timestamp(column) && !column.auto_increment
            && !table.primary_key.contains(&column.name)
    })
}

/// Identifies timestamp types; e.g. `timestamp(3) with time zone` is temporal.
fn timestamp(column: &Column) -> bool {
    column.sql_type.trim().starts_with("timestamp")
}

/// Snapshot payload; e.g. a key plus URL/JSONB/audit columns yields only URL/JSONB.
/// Never replace generated identities, primary/conflict keys, or audit timestamps.
pub(super) fn payload<'a>(table: &'a Table, key: &[String]) -> Vec<&'a Column> {
    table.columns.iter().filter(|column| {
        !column.auto_increment && !table.primary_key.contains(&column.name)
            && !key.contains(&column.name)
            && column.name != "created_at" && column.name != "updated_at"
            && !(timestamp(column) && column.default.is_some())
    }).collect()
}

/// Supports latest-by-text reads; e.g. a nullable `page_url: text` remains eligible.
pub(super) fn text(column: &Column) -> bool {
    let kind = column.sql_type.trim();
    kind == "text" || kind.starts_with("character varying") || kind.starts_with("varchar")
}