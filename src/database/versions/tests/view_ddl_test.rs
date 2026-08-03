//! View DDL details: replacement, quoting, RLS, and teardown.

use super::view_test::renamed_table;
use crate::database::versions::model::{Column, Table};
use crate::database::versions::view::{create_view, drop_schema};

#[test]
fn view_is_replaced_not_appended() {
    let statements = create_view("public", "02_rename", &renamed_table(), 16);
    assert!(statements[0].starts_with("DROP VIEW IF EXISTS"));
}

#[test]
fn security_invoker_depends_on_server_version() {
    // Respects row-level security on the underlying table.
    let modern = create_view("public", "v", &renamed_table(), 16).join("\n");
    assert!(modern.contains("security_invoker = true"));

    // Unsupported before Postgres 15.
    let legacy = create_view("public", "v", &renamed_table(), 14).join("\n");
    assert!(!legacy.contains("security_invoker"));
}

#[test]
fn identifiers_are_quoted_against_injection() {
    let table = Table::new(r#"od"d"#, vec![Column::new("id")]);
    let sql = create_view("public", "v", &table, 16).join("\n");

    // The embedded quote is doubled, not left to terminate the identifier.
    assert!(sql.contains(r#""od""d""#));
}

#[test]
fn dropping_a_version_cascades() {
    assert_eq!(
        drop_schema("public", "01_init"),
        r#"DROP SCHEMA IF EXISTS "public_01_init" CASCADE"#
    );
}
