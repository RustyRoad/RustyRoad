//! Which tables a plan schedules for backfill.

use super::plan_test::rename_migration;
use crate::database::versions::ops::model::Migration;
use crate::database::versions::ops::plan;

#[test]
fn rename_schedules_a_backfill() {
    let plan = plan(&rename_migration(), "public_02_rename");

    // Rows never written would otherwise keep an empty shadow column.
    assert_eq!(plan.backfill_tables, vec!["platform_trash_zone_mappings"]);
}

#[test]
fn drop_column_keeps_it_readable_until_completion() {
    let migration: Migration = serde_json::from_str(
        r#"{"operations":[{"drop_column":{"table":"users","column":"legacy",
            "down":"'unknown'"}}]}"#,
    )
    .expect("should parse");

    let plan = plan(&migration, "public_v");
    let start = plan.statements.join("\n");

    // Start installs a trigger keeping the column populated for old readers.
    assert!(start.contains("CREATE OR REPLACE FUNCTION"));
    assert!(!start.contains("DROP COLUMN"));
    // The drop itself waits for completion.
    assert!(plan
        .completion_statements()
        .join("\n")
        .contains(r#"DROP COLUMN IF EXISTS "legacy""#));
}
