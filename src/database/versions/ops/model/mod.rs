//! Declarative migration operations.
//!
//! A migration may be authored as JSON instead of raw SQL, declaring what changes
//! rather than how to apply them. This is what makes a breaking column change safe:
//! the runner knows a column is being replaced, so it can create a shadow column,
//! install triggers to keep both in sync, and backfill existing rows while both
//! schema versions are being served.

mod kinds;

use serde::{Deserialize, Serialize};

pub use kinds::{AddColumn, AlterColumn, DropColumn, RawSql};

/// A declarative migration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Migration {
    /// Version name; defaults to the migration directory name.
    #[serde(default)]
    pub name: String,
    pub operations: Vec<Operation>,
}

/// One declared change.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    /// Adds a column, backfilling existing rows from `up`.
    AddColumn(AddColumn),
    /// Changes a column's type, name, or nullability without breaking readers.
    AlterColumn(AlterColumn),
    /// Removes a column from the new version while old readers still see it.
    DropColumn(DropColumn),
    /// Raw SQL applied verbatim.
    Sql(RawSql),
}

impl Operation {
    /// Returns the table the operation affects, if any.
    pub fn table(&self) -> Option<&str> {
        match self {
            Self::AddColumn(op) => Some(&op.table),
            Self::AlterColumn(op) => Some(&op.table),
            Self::DropColumn(op) => Some(&op.table),
            Self::Sql(_) => None,
        }
    }

    /// Returns `true` when existing rows must be rewritten.
    ///
    /// Raw SQL is applied directly, and a new column with no `up` expression has no
    /// per-row value to compute.
    pub fn needs_backfill(&self) -> bool {
        match self {
            Self::AddColumn(op) => op.up.is_some(),
            Self::AlterColumn(_) => true,
            Self::DropColumn(op) => op.down.is_some(),
            Self::Sql(_) => false,
        }
    }
}
