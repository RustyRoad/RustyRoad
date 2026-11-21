//! Negative fixtures must retain unresolved ownership rather than invent tables.
use super::audit;

/// Projections, dynamic SQL, and unrelated row types are not ownership proof.
#[test]
fn rejects_queries_without_a_whole_row_literal_contract() {
    for query in [
        r#"sqlx::query_as::<_, Row>("SELECT id FROM sessions");"#,
        r#"sqlx::query_as::<_, Row>("SELECT * FROM sessions JOIN other USING (id)");"#,
        "sqlx::query_as::<_, Row>(sql);",
        r#"sqlx::query_as::<_, other::Row>("SELECT * FROM sessions");"#,
        r#"query_as::<_, Row>("SELECT * FROM sessions");"#,
    ] {
        let source = format!("struct Row {{ id: i32 }} async fn load() {{ {query} }}");
        assert_eq!(
            audit(&source, None).mapping,
            "unresolved_persistence_contract"
        );
    }
}

/// Conditional scopes and shadowed namespaces must not establish ownership.
#[test]
fn rejects_conditional_or_shadowed_source() {
    let query = r#"sqlx::query_as::<_, Row>("SELECT * FROM sessions");"#;
    for source in [
        format!(r#"#![cfg(feature = "x")] struct Row {{ id: i32 }} fn load() {{ {query} }}"#),
        format!(r#"struct Row {{ id: i32 }} #[cfg(feature = "x")] fn load() {{ {query} }}"#),
        format!("struct Row {{ id: i32 }} fn load() {{ if ready {{ {query} }} }}"),
        format!("struct Row {{ id: i32 }} fn load() {{ let f = || {{ {query} }}; }}"),
        format!("struct Row {{ id: i32 }} mod sqlx {{}} fn load() {{ {query} }}"),
        format!("use other as sqlx; struct Row {{ id: i32 }} fn load() {{ {query} }}"),
        format!("use other::*; struct Row {{ id: i32 }} fn load() {{ {query} }}"),
        format!("struct Row {{ id: i32 }} fn load() {{ type Row = Other; {query} }}"),
        format!("struct Row {{ id: i32 }} fn load() {{ let (sqlx, _) = x; {query} }}"),
    ] {
        assert_eq!(
            audit(&source, None).mapping,
            "unresolved_persistence_contract",
            "{source}"
        );
    }
}

/// Inline modules are resolved by declaration identity, not a basename search.
#[test]
fn conditional_inline_module_does_not_supply_a_contract() {
    let result = audit(
        r#"#[cfg(feature = "x")] mod nested {
        struct Row { id: i32 }
        fn load() { sqlx::query_as::<_, Row>("SELECT * FROM sessions"); }
    }"#,
        None,
    );
    assert_eq!(result.mapping, "unresolved_persistence_contract");
}
