//! Trigger naming, expression handling, and teardown.

use super::trigger_test::up_trigger;
use crate::database::versions::ops::trigger::{
    drop_marker, function_name, Trigger, NEEDS_BACKFILL_COLUMN,
};

#[test]
fn expressions_are_parenthesized_once() {
    let already = Trigger {
        expression: "(NEW.a + NEW.b)",
        ..up_trigger()
    };
    let sql = already.install().join("\n");

    // Precedence must not leak, but parentheses must not be doubled either.
    assert!(sql.contains("= (NEW.a + NEW.b)"));
    assert!(!sql.contains("((NEW.a + NEW.b))"));
}

#[test]
fn bare_expressions_are_wrapped() {
    let sql = up_trigger().install().join("\n");
    assert!(sql.contains("= (NEW.zone_id::text::uuid)"));
}

#[test]
fn removal_drops_trigger_and_function() {
    let statements = up_trigger().remove();

    assert!(statements[0].starts_with("DROP TRIGGER IF EXISTS"));
    assert!(statements[1].starts_with("DROP FUNCTION IF EXISTS"));
}

#[test]
fn function_names_are_scoped_to_table_and_column() {
    let name = function_name("users", "_rustyroad_new_email");

    assert!(name.contains("users"));
    assert!(name.contains("_rustyroad_new_email"));
    // Two columns on one table must not collide.
    assert_ne!(name, function_name("users", "_rustyroad_new_name"));
}

#[test]
fn marker_column_is_dropped_by_table() {
    assert!(drop_marker("users").contains(NEEDS_BACKFILL_COLUMN));
}
