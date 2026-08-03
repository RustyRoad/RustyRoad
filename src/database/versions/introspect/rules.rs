//! Filtering and renaming rules for introspected columns.

use super::super::naming::DELETION_PREFIX;
use super::super::ops::trigger::NEEDS_BACKFILL_COLUMN;
use super::build::Row;

/// Prefix marking RustyRoad's own bookkeeping tables.
const INTERNAL_TABLE_PREFIX: &str = "_rustyroad_";

/// Renames to apply when projecting: `(table, from, to)`.
pub(super) type Renames = [(String, String, String)];

/// Returns `true` when a row belongs in a client-facing version.
///
/// RustyRoad's own tables and the backfill marker are implementation detail; a
/// column pending removal is absent from the current version by definition.
pub(super) fn is_visible(row: &Row) -> bool {
    !row.table.is_empty()
        && !row.column.is_empty()
        && !row.table.starts_with(INTERNAL_TABLE_PREFIX)
        && row.column != NEEDS_BACKFILL_COLUMN
        && !row.column.starts_with(DELETION_PREFIX)
}

/// Applies a declared rename to a logical column name.
///
/// The new version's views must expose the new name, otherwise clients would have
/// to keep using the old one until completion, which defeats the rename.
pub(super) fn renamed(table: &str, column: &str, renames: &Renames) -> String {
    renames
        .iter()
        .find(|(rename_table, from, _)| rename_table == table && from == column)
        .map(|(_, _, to)| to.clone())
        .unwrap_or_else(|| column.to_string())
}
