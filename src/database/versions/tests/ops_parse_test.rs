//! Parsing declarative migration definitions.

use crate::database::versions::ops::model::{Migration, Operation};

#[test]
fn alter_column_rename_parses() {
    let json = r#"{
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
    }"#;

    let migration: Migration = serde_json::from_str(json).expect("should parse");
    assert_eq!(migration.name, "02_rename_zone");

    let Operation::AlterColumn(op) = &migration.operations[0] else {
        panic!("expected alter_column");
    };
    assert_eq!(op.name.as_deref(), Some("trash_zone_id"));
    assert_eq!(op.nullable, Some(false));
    // Both directions are required for a breaking rename to stay readable.
    assert!(op.up.is_some() && op.down.is_some());
}

#[test]
fn add_column_parses_with_conservative_defaults() {
    let json = r#"{"operations":[
        {"add_column":{"table":"users","column":"status","type":"text"}}]}"#;

    let migration: Migration = serde_json::from_str(json).expect("should parse");
    let Operation::AddColumn(op) = &migration.operations[0] else {
        panic!("expected add_column");
    };

    assert!(!op.nullable);
    assert_eq!(op.default, None);
    assert_eq!(op.up, None);
}

#[test]
fn drop_column_and_raw_sql_parse() {
    let json = r#"{"operations":[
        {"drop_column":{"table":"users","column":"legacy","down":"'unknown'"}},
        {"sql":{"up":"CREATE INDEX idx ON users (status)"}}]}"#;

    let migration: Migration = serde_json::from_str(json).expect("should parse");
    assert!(matches!(migration.operations[0], Operation::DropColumn(_)));
    assert_eq!(migration.operations[0].table(), Some("users"));
    // Raw SQL is opaque, so no table can be attributed to it.
    assert_eq!(migration.operations[1].table(), None);
}
