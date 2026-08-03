//! Declarative operations: safe column changes via shadow columns and backfill.
//!
//! A raw-SQL migration that renames or retypes a column breaks readers of the old
//! schema the moment it runs. A declared operation instead:
//!
//! 1. creates a shadow physical column beside the original,
//! 2. installs triggers so a write through either schema version populates both,
//! 3. backfills existing rows in batches, and
//! 4. at completion, promotes the shadow and drops the original.
//!
//! Both versions therefore read and write correctly for the whole rollout.

pub mod backfill;
pub mod duplicate;
pub mod model;
mod plan_complete;
mod plan_start;
pub mod trigger;

use super::quote::quote_ident;
use model::Migration;

/// Statements and follow-up work for one migration's operations.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plan {
    /// Statements applied during `start`.
    pub statements: Vec<String>,
    /// Tables needing a batched backfill after `start`.
    pub backfill_tables: Vec<String>,
    /// Shadow columns to promote at completion: `(table, column)`.
    pub promotions: Vec<(String, String)>,
    /// Columns to drop at completion: `(table, column)`.
    pub drops: Vec<(String, String)>,
    /// Columns to rename at completion: `(table, from, to)`.
    pub renames: Vec<(String, String, String)>,
    /// NOT NULL constraints deferred until the backfill finished.
    pub deferred_not_null: Vec<(String, String)>,
}

/// Builds the plan for every operation in `migration`.
pub fn plan(migration: &Migration, latest_schema: &str) -> Plan {
    let mut combined = Plan::default();

    for operation in &migration.operations {
        let plan = plan_start::plan(operation, latest_schema);
        combined.statements.extend(plan.statements);
        combined.backfill_tables.extend(plan.backfill_tables);
        combined.promotions.extend(plan.promotions);
        combined.drops.extend(plan.drops);
        combined.renames.extend(plan.renames);
        combined.deferred_not_null.extend(plan.deferred_not_null);
    }

    combined.backfill_tables.sort();
    combined.backfill_tables.dedup();
    combined
}

/// Returns a `NEW.<column>` reference, the default forward expression.
fn new_ref(column: &str) -> String {
    format!("NEW.{}", quote_ident(column))
}
