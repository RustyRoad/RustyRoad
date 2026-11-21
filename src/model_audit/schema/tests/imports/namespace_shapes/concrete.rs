//! Extracted fixtures retain their original assertions and inputs.
use super::audits;

/// Example: `super::Key` and `db::types::Uuid` retain legitimate codec identity.
#[test]
fn concrete_alias_namespaces_preserve_codecs() {
    let result = audits(
        r#"
        use uuid::Uuid as Key;
        mod nested {
            use sqlx as db;
            use super::Key;
            struct Row {
                id: i32,
                parent: super::Key,
                local: self::Key,
                keys: Vec<db::types::Uuid>,
            }
        }
        "#,
    );
    let row = &result[0];
    assert!(row.blockers.is_empty(), "{:?}", row.blockers);
    for (name, sql) in [("parent", "uuid"), ("local", "uuid"), ("keys", "uuid[]")] {
        let column = row.desired_columns.iter().find(|c| c.name == name).unwrap();
        assert_eq!(column.sql_type, sql, "{name}");
        assert!(!column.nullable, "{name}");
    }
    assert!(row.proposal.is_some());
}
