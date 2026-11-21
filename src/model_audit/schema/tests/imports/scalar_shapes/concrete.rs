//! Extracted fixtures retain their original assertions and inputs.
use super::audits;

/// Example: concrete aliases and legitimate generic containers retain codecs.
#[test]
fn concrete_aliases_and_containers_retain_known_codecs() {
    let result = audits(
        r#"
        use uuid::Uuid as Key;
        use chrono::{DateTime as Stamp, Utc as Zone};
        struct Row {
            id: i32,
            key: Key,
            keys: Vec<Key>,
            at: Option<Stamp<Zone>>,
            bytes: std::vec::Vec<u8>,
            text: ::std::string::String,
            body: sqlx::types::Json<serde_json::Value>,
        }
        "#,
    );
    let row = &result[0];
    assert!(row.blockers.is_empty(), "{:?}", row.blockers);
    for (name, sql, nullable) in [
        ("key", "uuid", false),
        ("keys", "uuid[]", false),
        ("at", "timestamp with time zone", true),
        ("bytes", "bytea", false),
        ("text", "text", false),
        ("body", "jsonb", false),
    ] {
        let column = row.desired_columns.iter().find(|c| c.name == name).unwrap();
        assert_eq!(column.sql_type, sql, "{name}");
        assert_eq!(column.nullable, nullable, "{name}");
    }
    assert!(row.proposal.is_some());
}
