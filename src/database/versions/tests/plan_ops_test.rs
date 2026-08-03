//! Planning add, drop, and raw SQL operations.

use crate::database::versions::ops::model::Migration;
use crate::database::versions::ops::plan;

#[test]
fn add_column_defers_not_null_until_backfilled() {
    let migration: Migration = serde_json::from_str(
        r#"{"operations":[{"add_column":{"table":"users","column":"status",
            "type":"text","nullable":false,"up":"'active'"}}]}"#,
    )
    .expect("should parse");

    let plan = plan(&migration, "public_v");

    // Applying NOT NULL before the backfill would fail on existing rows.
    assert!(!plan.statements.join("\n").contains("SET NOT NULL"));
    assert!(plan
        .completion_statements()
        .join("\n")
        .contains(r#"ALTER COLUMN "status" SET NOT NULL"#));
}

#[test]
fn raw_sql_passes_through_untouched() {
    let migration: Migration = serde_json::from_str(
        r#"{"operations":[{"sql":{"up":"CREATE INDEX idx ON users (status)"}}]}"#,
    )
    .expect("should parse");

    let plan = plan(&migration, "public_v");

    assert_eq!(plan.statements, vec!["CREATE INDEX idx ON users (status)"]);
    // No shadow, no trigger, no backfill.
    assert!(plan.backfill_tables.is_empty());
    assert!(plan.promotions.is_empty());
}

#[test]
fn multiple_operations_merge_into_one_plan() {
    let migration: Migration = serde_json::from_str(
        r#"{"operations":[
            {"add_column":{"table":"users","column":"a","type":"text","up":"'x'"}},
            {"add_column":{"table":"orders","column":"b","type":"text","up":"'y'"}}]}"#,
    )
    .expect("should parse");

    let plan = plan(&migration, "public_v");

    // Each table is backfilled once, in stable order.
    assert_eq!(plan.backfill_tables, vec!["orders", "users"]);
}
