//! View projection: logical names, defaults, and omitted objects.

use crate::database::versions::model::{Column, Schema, Table};
use crate::database::versions::view::{create_version, create_view};

/// A table whose `zone_id` was replaced by an in-flight `trash_zone_id`.
pub(super) fn renamed_table() -> Table {
    Table::new(
        "platform_trash_zone_mappings",
        vec![
            Column::new("id"),
            Column::new("trash_zone_id").backed_by("_rustyroad_new_trash_zone_id"),
        ],
    )
}

#[test]
fn view_projects_physical_columns_under_logical_names() {
    let sql = create_view("public", "02_rename", &renamed_table(), 16).join("\n");

    // The logical name is what clients see; the physical column backs it.
    assert!(sql.contains(r#""_rustyroad_new_trash_zone_id" AS "trash_zone_id""#));
    assert!(sql.contains(r#"CREATE VIEW "public_02_rename"."platform_trash_zone_mappings""#));
    assert!(sql.contains(r#"FROM "public"."platform_trash_zone_mappings""#));
}

#[test]
fn deleted_columns_are_omitted() {
    let table = Table::new(
        "users",
        vec![Column::new("id"), Column::new("legacy").deleted()],
    );
    let sql = create_view("public", "v", &table, 16).join("\n");

    assert!(sql.contains(r#""id" AS "id""#));
    assert!(!sql.contains("legacy"));
}

#[test]
fn column_defaults_are_restated_on_the_view() {
    let table = Table::new("users", vec![Column::new("status").with_default("'active'")]);
    let sql = create_view("public", "v", &table, 16).join("\n");

    // A view does not inherit its table's defaults, so they must be set explicitly.
    assert!(sql.contains(r#"ALTER VIEW "public_v"."users" ALTER "status" SET DEFAULT 'active'"#));
}

#[test]
fn deleted_tables_are_omitted_from_the_version() {
    let mut hidden = Table::new("gone", vec![Column::new("id")]);
    hidden.deleted = true;
    let schema = Schema {
        tables: vec![renamed_table(), hidden],
    };

    let sql = create_version("public", "v", &schema, 16).join("\n");
    assert!(sql.contains(r#"CREATE SCHEMA IF NOT EXISTS "public_v""#));
    assert!(!sql.contains("gone"));
}
