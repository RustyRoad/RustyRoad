//! Planning `alter_column`, the breaking-change case.

use super::super::duplicate::Duplicate;
use super::super::model::AlterColumn;
use super::super::trigger::{Direction, Trigger};
use super::super::{new_ref, Plan};

/// Alters a column via a shadow copy kept in sync in both directions.
///
/// The original column is left untouched during `start`, so readers of the old
/// schema version continue to work. Both are promoted at completion.
pub(super) fn plan(op: &AlterColumn, latest_schema: &str) -> Plan {
    let duplicate = Duplicate {
        table: &op.table,
        column: &op.column,
        column_type: op.column_type.as_deref(),
        nullable: op.nullable.unwrap_or(true),
        default: None,
    };

    let mut plan = Plan {
        statements: vec![duplicate.add_column()],
        ..Plan::default()
    };

    // Forward: old column to shadow, for readers of the new version.
    let shadow = duplicate.shadow_name();
    let default_up = new_ref(&op.column);
    plan.statements.extend(
        Trigger {
            table: &op.table,
            physical_column: &shadow,
            direction: Direction::Up,
            expression: op.up.as_deref().unwrap_or(&default_up),
            latest_schema,
        }
        .install(),
    );

    // Reverse: shadow back to old column, so readers of the old version keep working.
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
    }

    plan.backfill_tables.push(op.table.clone());
    plan.promotions.push((op.table.clone(), op.column.clone()));
    if let Some(name) = &op.name {
        plan.renames
            .push((op.table.clone(), op.column.clone(), name.clone()));
    }

    plan
}
