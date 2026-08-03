//! Triggers keeping an altered column in sync with its shadow.

use crate::database::versions::ops::duplicate::Duplicate;
use crate::database::versions::ops::model::AlterColumn;
use crate::database::versions::ops::new_ref;
use crate::database::versions::ops::trigger::{Direction, Trigger};

/// Returns the statements installing both sync triggers.
///
/// Forward keeps the shadow populated for readers of the new version; reverse keeps
/// the original populated so readers of the old version keep working.
pub(super) fn install(
    op: &AlterColumn,
    duplicate: &Duplicate<'_>,
    latest_schema: &str,
) -> Vec<String> {
    let shadow = duplicate.shadow_name();
    let default_up = new_ref(&op.column);

    let mut statements = Trigger {
        table: &op.table,
        physical_column: &shadow,
        direction: Direction::Up,
        expression: op.up.as_deref().unwrap_or(&default_up),
        latest_schema,
        rewrite: None,
    }
    .install();

    // The `down` expression is written against the new logical name, which only the
    // view exposes; on the row it is the shadow column, so it must be rewritten.
    if let Some(down) = &op.down {
        let logical = op.name.as_deref().unwrap_or(&op.column);
        statements.extend(
            Trigger {
                table: &op.table,
                physical_column: &op.column,
                direction: Direction::Down,
                expression: down,
                latest_schema,
                rewrite: Some((logical, &shadow)),
            }
            .install(),
        );
    }

    statements
}
