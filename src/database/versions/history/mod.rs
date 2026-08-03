//! Migration history: linear parent chain, active migration, and baselines.
//!
//! The history table records one row per migration with a `parent` pointer forming
//! a linear chain, a `done` flag distinguishing started from completed migrations,
//! and a `migration_type` marking baselines.

mod schema;

use super::exec;
use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;

pub use schema::{HISTORY_TABLE, TYPE_BASELINE, TYPE_MIGRATION};

/// A history row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub parent: Option<String>,
    pub done: bool,
    pub migration_type: String,
}

impl Entry {
    /// Returns `true` when this entry was created by `baseline`.
    pub fn is_baseline(&self) -> bool {
        self.migration_type == TYPE_BASELINE
    }
}

/// Creates the history table and its invariants when absent.
pub async fn ensure_table(connection: &DatabaseConnection) -> Result<(), CustomMigrationError> {
    exec::run(connection, &schema::create_table(), &[]).await?;

    for statement in schema::create_indexes() {
        // Index creation is best effort: a legacy table may hold rows that violate
        // the invariants, and the lifecycle checks do not depend on the indexes.
        let _ = exec::run(connection, &statement, &[]).await;
    }

    Ok(())
}
