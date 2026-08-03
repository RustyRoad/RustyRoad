//! Planning the completion and rollback phases.

mod columns;
mod shadows;
mod triggers;

use super::Plan;

impl Plan {
    /// Returns the statements finalizing this plan.
    ///
    /// Order matters: triggers are removed before the columns they reference, shadows
    /// are promoted before constraints are applied, and the backfill marker is dropped
    /// last once nothing depends on it.
    pub fn completion_statements(&self) -> Vec<String> {
        let mut statements = self.remove_triggers();
        statements.extend(self.promote_shadows());
        statements.extend(self.drop_columns());
        statements.extend(self.rename_columns());
        statements.extend(self.apply_not_null());
        statements.extend(self.drop_markers());
        statements
    }

    /// Returns the statements undoing this plan's start phase.
    ///
    /// Drops the triggers, shadow columns, and marker column so the table returns to
    /// the shape it had before `start`. Best effort by design: the caller ignores
    /// individual failures, since a partially applied start may not have created
    /// everything this would remove.
    pub fn rollback_statements(&self) -> Vec<String> {
        let mut statements = self.remove_triggers();
        statements.extend(self.discard_shadows());
        statements.extend(self.drop_markers());
        statements
    }
}
