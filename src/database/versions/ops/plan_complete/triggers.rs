//! Trigger teardown statements.

use super::super::trigger::{drop_marker, function_name, Direction, Trigger};
use super::super::Plan;

impl Plan {
    /// Drops every trigger and function this plan installed.
    pub(super) fn remove_triggers(&self) -> Vec<String> {
        let mut statements = Vec::new();
        for table in self.touched_tables() {
            for column in self.trigger_columns(&table) {
                statements.extend(
                    Trigger {
                        table: &table,
                        physical_column: &column,
                        direction: Direction::Up,
                        expression: "NULL",
                        latest_schema: "",
                    }
                    .remove(),
                );
            }
        }
        statements
    }

    /// Removes the backfill marker column from every touched table.
    pub(super) fn drop_markers(&self) -> Vec<String> {
        self.touched_tables()
            .iter()
            .map(|table| drop_marker(table))
            .collect()
    }

    /// Returns every table this plan installed a trigger on.
    pub(super) fn touched_tables(&self) -> Vec<String> {
        let mut tables = self.backfill_tables.clone();
        tables.sort();
        tables.dedup();
        tables
    }

    /// Returns the physical columns carrying triggers on `table`.
    ///
    /// Derived from the generated statements rather than tracked separately, so the
    /// two cannot drift out of sync.
    fn trigger_columns(&self, table: &str) -> Vec<String> {
        let prefix = function_name(table, "");
        let mut columns: Vec<String> = self
            .statements
            .iter()
            .filter_map(|statement| statement.split(&prefix).nth(1))
            .filter_map(|rest| rest.split('"').next())
            .filter(|column| !column.is_empty())
            .map(str::to_string)
            .collect();
        columns.sort();
        columns.dedup();
        columns
    }
}
