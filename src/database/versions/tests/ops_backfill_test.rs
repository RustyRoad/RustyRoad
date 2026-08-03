//! Which operations require rewriting existing rows.

use crate::database::versions::ops::model::Migration;

#[test]
fn backfill_is_required_only_when_values_change() {
    let json = r#"{"operations":[
        {"add_column":{"table":"users","column":"a","type":"text"}},
        {"add_column":{"table":"users","column":"b","type":"text","up":"'x'"}},
        {"sql":{"up":"SELECT 1"}}]}"#;

    let migration: Migration = serde_json::from_str(json).expect("should parse");

    // A plain new column has no per-row value to compute.
    assert!(!migration.operations[0].needs_backfill());
    // An `up` expression must be evaluated for every existing row.
    assert!(migration.operations[1].needs_backfill());
    // Raw SQL is applied as written.
    assert!(!migration.operations[2].needs_backfill());
}

#[test]
fn altering_a_column_always_requires_backfill() {
    let json = r#"{"operations":[
        {"alter_column":{"table":"users","column":"email","type":"citext"}}]}"#;

    let migration: Migration = serde_json::from_str(json).expect("should parse");
    // The shadow column starts empty regardless of what changed.
    assert!(migration.operations[0].needs_backfill());
}

#[test]
fn dropping_a_column_backfills_only_to_keep_it_readable() {
    let without = r#"{"operations":[
        {"drop_column":{"table":"users","column":"legacy"}}]}"#;
    let with = r#"{"operations":[
        {"drop_column":{"table":"users","column":"legacy","down":"'unknown'"}}]}"#;

    let plain: Migration = serde_json::from_str(without).expect("should parse");
    let compensating: Migration = serde_json::from_str(with).expect("should parse");

    assert!(!plain.operations[0].needs_backfill());
    // A `down` expression keeps old readers supplied until completion.
    assert!(compensating.operations[0].needs_backfill());
}
