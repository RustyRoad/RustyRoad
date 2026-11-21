//! Extracted fixtures retain their original assertions and inputs.
use super::audits;

/// Example: imported containers accept arguments; closed aliases keep theirs.
#[test]
fn preserves_closed_alias_targets_and_imported_containers() {
    let result = audits(
        r#"
        use uuid::Uuid as Key;
        use std::vec::Vec as List;
        type Keys = List<Key>;
        type Maybe = Option<Key>;
        type Stamp = chrono::DateTime<chrono::Utc>;
        type Body = sqlx::types::Json<serde_json::Value>;
        struct Row { id: i32, keys: Keys, maybe: Maybe, at: Stamp, body: Body }
        "#,
    );
    let row = result.last().unwrap();
    assert!(row.blockers.is_empty(), "{:?}", row.blockers);
    for (name, sql, nullable) in [
        ("keys", "uuid[]", false),
        ("maybe", "uuid", true),
        ("at", "timestamp with time zone", false),
        ("body", "jsonb", false),
    ] {
        let column = row.desired_columns.iter().find(|c| c.name == name).unwrap();
        assert_eq!(column.sql_type, sql, "{name}");
        assert_eq!(column.nullable, nullable, "{name}");
    }
    assert!(row.proposal.is_some());
}
