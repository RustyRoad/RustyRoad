//! Extracted fixtures retain their original assertions and inputs.
use super::audit;

/// Generic namespace/type lookalikes cannot stand in for actual SQLx identities.
#[test]
fn rejects_parameterized_constructor_and_database_paths() {
    for query in [
        r#"sqlx::<Other>::query_as::<_, Row>("SELECT * FROM sessions");"#,
        r#"sqlx::query_as::<sqlx::Postgres<Other>, Row>("SELECT * FROM sessions");"#,
        r#"sqlx::query_as::<sqlx::postgres::Postgres<Other>, Row>("SELECT * FROM sessions");"#,
        r#"sqlx::query_as::<sqlx::postgres::<Other>::Postgres, Row>("SELECT * FROM sessions");"#,
        r#"sqlx::query_as::<other::postgres::Postgres, Row>("SELECT * FROM sessions");"#,
        r#"sqlx::query_as::<sqlx::MySql, Row>("SELECT * FROM sessions");"#,
        r#"sqlx::query_as_unchecked::<_, Row>("SELECT * FROM sessions");"#,
    ] {
        let source = format!("struct Row {{ id: i32 }} fn load() {{ {query} }}");
        assert_eq!(
            audit(&source, None).mapping,
            "unresolved_persistence_contract",
            "{source}"
        );
    }
}
