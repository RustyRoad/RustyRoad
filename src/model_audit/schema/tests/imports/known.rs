//! Extracted fixtures retain their original assertions and inputs.
use super::audits;

#[test]
fn imported_aliases_and_containers_produce_known_postgres_contracts() {
    let audits = audits(
        r#"
        use chrono::{DateTime, Utc};
        use uuid::Uuid as Identifier;
        use serde_json::Value as JsonValue;
        struct Event { id: Identifier, at: Option<DateTime<Utc>>, body: JsonValue,
                       ids: Vec<Identifier> }
    "#,
    );
    let audit = &audits[0];
    assert!(audit.blockers.is_empty(), "{:?}", audit.blockers);
    let columns = &audit.desired_columns;
    for (name, sql, nullable) in [
        ("id", "uuid", false),
        ("at", "timestamp with time zone", true),
        ("body", "jsonb", false),
        ("ids", "uuid[]", false),
    ] {
        let column = columns.iter().find(|column| column.name == name).unwrap();
        assert_eq!(column.sql_type, sql);
        assert_eq!(column.nullable, nullable);
    }
    assert!(audit.proposal.is_some());
}

#[test]
fn nested_module_resolves_only_explicit_parent_imports() {
    let audits = audits(
        r#"
        use uuid::Uuid;
        mod inner { use super::Uuid as Key; struct Event { id: Key } }
    "#,
    );
    assert_eq!(audits[0].desired_columns[0].sql_type, "uuid");
}
