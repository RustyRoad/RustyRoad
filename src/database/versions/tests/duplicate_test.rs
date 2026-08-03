//! Shadow column duplication and promotion.

use crate::database::versions::ops::duplicate::Duplicate;

/// The zone_id -> trash_zone_id case: retype and rename without breaking readers.
fn zone_id() -> Duplicate<'static> {
    Duplicate {
        table: "platform_trash_zone_mappings",
        column: "zone_id",
        column_type: Some("uuid"),
        nullable: false,
        default: None,
    }
}

#[test]
fn shadow_column_is_prefixed() {
    assert_eq!(zone_id().shadow_name(), "_rustyroad_new_zone_id");
}

#[test]
fn shadow_column_is_added_nullable() {
    let sql = zone_id().add_column();

    assert!(sql.contains(r#"ADD COLUMN IF NOT EXISTS "_rustyroad_new_zone_id" uuid"#));
    // Existing rows have no value yet, so NOT NULL here would fail before backfill.
    assert!(!sql.contains("NOT NULL"));
}

#[test]
fn promotion_replaces_the_original_then_constrains() {
    let statements = zone_id().promote();

    // The original goes first, so the rename does not collide with it.
    assert!(statements[0].contains(r#"DROP COLUMN IF EXISTS "zone_id""#));
    assert!(statements[1].contains(r#"RENAME COLUMN "_rustyroad_new_zone_id" TO "zone_id""#));
    // NOT NULL is safe only now that every row has a value.
    assert!(statements[2].contains(r#"ALTER COLUMN "zone_id" SET NOT NULL"#));
}

#[test]
fn nullable_columns_get_no_constraint() {
    let nullable = Duplicate {
        nullable: true,
        ..zone_id()
    };
    let statements = nullable.promote();

    assert_eq!(statements.len(), 2);
    assert!(!statements.join(" ").contains("SET NOT NULL"));
}

#[test]
fn discard_removes_only_the_shadow() {
    let sql = zone_id().discard();

    assert!(sql.contains(r#"DROP COLUMN IF EXISTS "_rustyroad_new_zone_id""#));
    // The original must survive a rollback.
    assert!(!sql.contains(r#""zone_id" "#));
}
