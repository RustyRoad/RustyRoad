//! Trigger function body generation.

use super::naming::{function_name, Direction, NEEDS_BACKFILL_COLUMN};
use super::Trigger;
use crate::database::versions::quote::{quote_ident, quote_literal};

/// Returns the `CREATE OR REPLACE FUNCTION` statement for `trigger`.
///
/// The function fires on insert and update. It compares the current search path
/// against the version being served so that a write arriving through the old schema
/// is rewritten into the new column, and a write arriving through the new schema is
/// rewritten back into the old one. Comparing the search path is what distinguishes
/// the two without the application knowing anything about the migration.
pub(super) fn function(trigger: &Trigger<'_>) -> String {
    let name = quote_ident(&function_name(trigger.table, trigger.physical_column));
    let comparison = match trigger.direction {
        Direction::Up => "<>",
        Direction::Down => "=",
    };

    format!(
        "CREATE OR REPLACE FUNCTION {name}()\n\
         RETURNS TRIGGER\n\
         LANGUAGE PLPGSQL\n\
         AS $$\n\
         DECLARE\n  \
           search_path TEXT;\n\
         BEGIN\n  \
           SELECT btrim(split_part(current_setting('search_path'), ',', 1), ' \"')\n    \
             INTO search_path;\n  \
           IF search_path {comparison} {latest} THEN\n    \
             NEW.{target} = {expression};\n    \
             NEW.{marker} = false;\n  \
           END IF;\n  \
           RETURN NEW;\n\
         END; $$",
        latest = quote_literal(trigger.latest_schema),
        target = quote_ident(trigger.physical_column),
        expression = parenthesize(&trigger.resolved_expression()),
        marker = quote_ident(NEEDS_BACKFILL_COLUMN),
    )
}

/// Wraps an expression in parentheses unless it already is.
///
/// The expression is interpolated into an assignment, so its precedence must not
/// leak into the surrounding statement.
fn parenthesize(expression: &str) -> String {
    let trimmed = expression.trim();
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        return trimmed.to_string();
    }
    format!("({trimmed})")
}
