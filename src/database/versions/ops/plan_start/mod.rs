//! Planning the start phase of a declarative operation.

mod add;
mod alter;

use super::model::{DropColumn, Operation};
use super::trigger::{Direction, Trigger};
use super::Plan;

/// Builds the start-phase plan for `operation`.
pub(super) fn plan(operation: &Operation, latest_schema: &str) -> Plan {
    match operation {
        Operation::AddColumn(op) => add::plan(op, latest_schema),
        Operation::AlterColumn(op) => alter::plan(op, latest_schema),
        Operation::DropColumn(op) => drop_column(op, latest_schema),
        Operation::Sql(op) => Plan {
            statements: vec![op.up.clone()],
            ..Plan::default()
        },
    }
}

/// Drops a column at completion, keeping it populated until then.
fn drop_column(op: &DropColumn, latest_schema: &str) -> Plan {
    let mut plan = Plan {
        drops: vec![(op.table.clone(), op.column.clone())],
        ..Plan::default()
    };

    // Old readers still select this column, so new writes must keep filling it.
    if let Some(down) = &op.down {
        plan.statements.extend(
            Trigger {
                table: &op.table,
                physical_column: &op.column,
                direction: Direction::Down,
                expression: down,
                latest_schema,
            }
            .install(),
        );
        plan.backfill_tables.push(op.table.clone());
    }

    plan
}
