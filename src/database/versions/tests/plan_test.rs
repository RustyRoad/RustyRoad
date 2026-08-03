//! Planning the zone_id -> trash_zone_id breaking rename.

use crate::database::versions::ops::model::Migration;
use crate::database::versions::ops::plan;

/// The rename that motivated this work: retype int4 to uuid and rename.
pub(super) fn rename_migration() -> Migration {
    serde_json::from_str(
        r#"{
            "name": "02_rename_zone",
            "operations": [{"alter_column": {
                "table": "platform_trash_zone_mappings",
                "column": "zone_id",
                "name": "trash_zone_id",
                "type": "uuid",
                "nullable": false,
                "up": "NEW.zone_id::text::uuid",
                "down": "NEW.trash_zone_id::text::int4"
            }}]
        }"#,
    )
    .expect("should parse")
}

#[test]
fn rename_creates_shadow_and_both_triggers() {
    let sql = plan(&rename_migration(), "public_02_rename")
        .statements
        .join("\n");

    // The original column is untouched during start, so old readers keep working.
    assert!(sql.contains(r#"ADD COLUMN IF NOT EXISTS "_rustyroad_new_zone_id" uuid"#));
    assert!(!sql.contains(r#"DROP COLUMN IF EXISTS "zone_id""#));

    // Writes through either version populate both columns.
    assert!(sql.contains("IF search_path <> 'public_02_rename'"));
    assert!(sql.contains("IF search_path = 'public_02_rename'"));
}

#[test]
fn destructive_work_is_deferred_to_completion() {
    let completion = plan(&rename_migration(), "public_02_rename")
        .completion_statements()
        .join("\n");

    // Only now is the old column removed and the shadow promoted.
    assert!(completion.contains(r#"DROP COLUMN IF EXISTS "zone_id""#));
    assert!(completion.contains(r#"RENAME COLUMN "_rustyroad_new_zone_id" TO "zone_id""#));
    assert!(completion.contains(r#"RENAME COLUMN "zone_id" TO "trash_zone_id""#));

    // Triggers must go before the columns they reference.
    let trigger_at = completion.find("DROP TRIGGER").expect("trigger dropped");
    let column_at = completion.find("DROP COLUMN").expect("column dropped");
    assert!(trigger_at < column_at);
}

#[test]
fn rollback_removes_only_what_start_added() {
    let rollback = plan(&rename_migration(), "public_02_rename")
        .rollback_statements()
        .join("\n");

    assert!(rollback.contains(r#"DROP COLUMN IF EXISTS "_rustyroad_new_zone_id""#));
    assert!(rollback.contains("DROP TRIGGER IF EXISTS"));
    // The original column must survive.
    assert!(!rollback.contains(r#"DROP COLUMN IF EXISTS "zone_id""#));
}
