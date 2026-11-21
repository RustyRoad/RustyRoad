//! Ownership evidence never substitutes for key, codec, or namespace contracts.
use super::{audit, empty};
use fixture::planned;

/// `SELECT *` proves the relation but cannot invent a primary key or keylessness.
#[test]
fn missing_query_owned_table_requires_a_reviewed_key_contract() {
    let source = r#"struct Row { id: i32 } fn load() {
        sqlx::query_as::<_, Row>("SELECT * FROM sessions");
    }"#;
    let unresolved = planned(source, false, &[], false);
    assert_eq!(unresolved.mapping, "source_row_query_contract");
    assert!(unresolved.proposal.is_none());
    assert!(unresolved.blockers.iter().any(|b| b.contains("not keys")));
    let keyed = planned(source, false, &["id".into()], false);
    assert!(keyed.blockers.is_empty(), "{:?}", keyed.blockers);
    assert!(keyed.proposal.unwrap().up.contains("PRIMARY KEY (\"id\")"));
    let keyless = planned(source, true, &[], false);
    assert!(!keyless.proposal.unwrap().up.contains("PRIMARY KEY"));
}

/// A query target cannot bypass occupied names, nullable keys, or unknown codecs.
#[test]
fn query_ownership_retains_independent_schema_blockers() {
    for (fields, occupied, expected) in [
        ("id: i32", true, "occupied"),
        ("id: Option<i32>", false, "non-nullable"),
        ("id: i32, opaque: UnknownCodec", false, "storage"),
    ] {
        let source = format!(
            r#"struct Row {{ {fields} }} fn load() {{
            sqlx::query_as::<_, Row>("SELECT * FROM sessions");
        }}"#
        );
        let result = planned(&source, false, &["id".into()], occupied);
        assert!(result.proposal.is_none(), "{source}");
        assert!(
            result.blockers.iter().any(|b| b.contains(expected)),
            "{expected}: {:?}",
            result.blockers
        );
    }
}

/// Explicit mappings remain authoritative, including when queries conflict.
#[test]
fn explicit_mapping_overrides_conflicting_source_targets() {
    let source = r#"struct Row { id: i32 } fn load() {
        sqlx::query_as::<_, Row>("SELECT * FROM sessions");
        sqlx::query_as::<_, Row>("SELECT * FROM archives");
    }"#;
    let result = audit(source, Some("reviewed_rows"));
    assert_eq!(result.mapping, "explicit_persistence_contract");
    assert_eq!(result.table.as_deref(), Some("reviewed_rows"));
    assert!(!result.blockers.iter().any(|b| b.contains("Conflicting")));
}

mod fixture;
