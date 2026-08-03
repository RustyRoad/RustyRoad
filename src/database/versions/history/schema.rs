//! DDL for the migration history table and its invariants.

/// Table holding migration history.
pub const HISTORY_TABLE: &str = "_rustyroad_history";

/// Recorded for a migration applied normally.
pub const TYPE_MIGRATION: &str = "migration";
/// Recorded for a migration adopted by `baseline` without executing SQL.
pub const TYPE_BASELINE: &str = "baseline";

/// Returns the `CREATE TABLE` statement for the history table.
pub(super) fn create_table() -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {HISTORY_TABLE} (
            name TEXT NOT NULL PRIMARY KEY,
            parent TEXT REFERENCES {HISTORY_TABLE} (name),
            done BOOLEAN NOT NULL DEFAULT FALSE,
            migration_type TEXT NOT NULL DEFAULT '{TYPE_MIGRATION}',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"
    )
}

/// Returns the indexes enforcing the history invariants.
///
/// These encode the rules in the database rather than in application logic: one
/// migration in progress, one root, and no forks.
pub(super) fn create_indexes() -> Vec<String> {
    vec![
        // At most one migration may be in progress.
        format!(
            "CREATE UNIQUE INDEX IF NOT EXISTS {HISTORY_TABLE}_only_one_active \
             ON {HISTORY_TABLE} ((done)) WHERE done = FALSE"
        ),
        // Only the first migration may lack a parent.
        format!(
            "CREATE UNIQUE INDEX IF NOT EXISTS {HISTORY_TABLE}_single_root \
             ON {HISTORY_TABLE} ((parent IS NULL)) WHERE parent IS NULL"
        ),
        // History is linear: a migration may have at most one child.
        format!(
            "CREATE UNIQUE INDEX IF NOT EXISTS {HISTORY_TABLE}_linear \
             ON {HISTORY_TABLE} (parent)"
        ),
    ]
}
