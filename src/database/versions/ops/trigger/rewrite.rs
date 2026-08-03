//! Rewriting user expressions from logical to physical column names.
//!
//! A trigger's `NEW` record carries the table's physical columns. While a migration
//! is in flight the new logical name is served only by a view, so an expression
//! written against the logical name must be rewritten to the column that actually
//! exists on the row.

use crate::database::versions::quote::quote_ident;

/// Rewrites `NEW.<logical>` references to their physical column.
///
/// Both bare and quoted forms are handled, since either is valid SQL.
pub(super) fn to_physical(expression: &str, logical: &str, physical: &str) -> String {
    if logical == physical {
        return expression.to_string();
    }

    let target = format!("NEW.{}", quote_ident(physical));
    expression
        .replace(&format!("NEW.{}", quote_ident(logical)), &target)
        .replace(&format!("NEW.{logical}"), &target)
}
