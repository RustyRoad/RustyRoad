//! Column drop, rename, and constraint statements at completion.

use super::super::quote_ident;
use super::super::Plan;

impl Plan {
    /// Drops columns removed by this version.
    pub(super) fn drop_columns(&self) -> Vec<String> {
        self.drops
            .iter()
            .map(|(table, column)| {
                format!(
                    "ALTER TABLE {} DROP COLUMN IF EXISTS {}",
                    quote_ident(table),
                    quote_ident(column)
                )
            })
            .collect()
    }

    /// Applies renames declared by this version.
    pub(super) fn rename_columns(&self) -> Vec<String> {
        self.renames
            .iter()
            .map(|(table, from, to)| {
                format!(
                    "ALTER TABLE {} RENAME COLUMN {} TO {}",
                    quote_ident(table),
                    quote_ident(from),
                    quote_ident(to)
                )
            })
            .collect()
    }

    /// Applies NOT NULL constraints deferred until the backfill completed.
    pub(super) fn apply_not_null(&self) -> Vec<String> {
        self.deferred_not_null
            .iter()
            .map(|(table, column)| {
                format!(
                    "ALTER TABLE {} ALTER COLUMN {} SET NOT NULL",
                    quote_ident(table),
                    quote_ident(column)
                )
            })
            .collect()
    }
}
