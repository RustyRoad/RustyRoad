//! Planning `alter_column`, the breaking-change case.

mod sync;

use crate::database::versions::ops::duplicate::Duplicate;
use crate::database::versions::ops::model::AlterColumn;
use crate::database::versions::ops::Plan;

/// Alters a column via a shadow copy kept in sync in both directions.
///
/// The original column is left untouched during `start`, so readers of the old
/// schema version continue to work. The shadow is promoted at completion.
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
        backfill_tables: vec![op.table.clone()],
        promotions: vec![(op.table.clone(), op.column.clone())],
        ..Plan::default()
    };
    plan.statements
        .extend(sync::install(op, &duplicate, latest_schema));

    if let Some(name) = &op.name {
        plan.renames
            .push((op.table.clone(), op.column.clone(), name.clone()));
    }

    // The shadow is created nullable so the backfill can run; the declared
    // constraint is applied once every row has a value. It targets the final
    // logical name, since the rename happens first at completion.
    if op.nullable == Some(false) {
        let final_name = op.name.clone().unwrap_or_else(|| op.column.clone());
        plan.deferred_not_null.push((op.table.clone(), final_name));
    }

    plan
}
