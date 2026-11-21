//! PostgreSQL aliases retain the same accepted row-query identities.
use super::audit;

/// Exact SQLx PostgreSQL imports, for example `Postgres as Pg`, stay supported.
#[test]
fn recognizes_source_resolved_database_aliases() {
    for (imports, query) in [
        (
            "use sqlx::Postgres as Pg;",
            r#"sqlx::query_as::<Pg, Row>("SELECT * FROM sessions");"#,
        ),
        (
            "use sqlx::postgres::Postgres as Pg;",
            r#"sqlx::query_as::<Pg, Row>("SELECT * FROM sessions");"#,
        ),
        (
            "use sqlx::postgres as pg;",
            r#"sqlx::query_as::<pg::Postgres, Row>("SELECT * FROM sessions");"#,
        ),
        (
            "",
            r#"sqlx::query_as::<::sqlx::postgres::Postgres, Row>("SELECT * FROM sessions");"#,
        ),
    ] {
        let source = format!("{imports} struct Row {{ id: i32 }} fn load() {{ {query} }}");
        let result = audit(&source, None);
        assert_eq!(result.mapping, "source_row_query_contract", "{source}");
        assert_eq!(result.table.as_deref(), Some("sessions"));
    }
}
