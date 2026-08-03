//! Planning `add_column`.

use super::super::model::AddColumn;
use super::super::quote_ident;
use super::super::trigger::{Direction, Trigger};
use super::super::Plan;

/// Adds a column, backfilling from `up` when given.
///
/// A new column is written directly rather than shadowed: no existing reader knows
/// about it, so there is no old value to preserve.
pub(super) fn plan(op: &AddColumn, latest_schema: &str) -> Plan {
    let mut sql = format!(
        "ALTER TABLE {} ADD COLUMN IF NOT EXISTS {} {}",
        quote_ident(&op.table),
        quote_ident(&op.column),
        op.column_type
    );
    if let Some(default) = &op.default {
        sql.push_str(&format!(" DEFAULT {default}"));
    }

    let mut plan = Plan {
        statements: vec![sql],
        ..Plan::default()
    };

    // Existing rows need a value computed from the row itself.
    if let Some(up) = &op.up {
        plan.statements.extend(
            Trigger {
                table: &op.table,
                physical_column: &op.column,
                direction: Direction::Up,
                expression: up,
                latest_schema,
            }
            .install(),
        );
        plan.backfill_tables.push(op.table.clone());
    }

    // NOT NULL is applied at completion, once every row has a value.
    if !op.nullable {
        plan.deferred_not_null
            .push((op.table.clone(), op.column.clone()));
    }

    plan
}
