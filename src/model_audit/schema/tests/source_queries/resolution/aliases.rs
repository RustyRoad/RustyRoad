//! Extracted fixtures retain their original assertions and inputs.
use super::audit;

/// Crate, function, and macro imports must resolve to SQLx before contributing.
#[test]
fn recognizes_source_resolved_constructor_aliases() {
    for (imports, query) in [
        (
            "use sqlx as db;",
            r#"db::query_as::<_, Row>("SELECT * FROM sessions");"#,
        ),
        (
            "use sqlx::query_as;",
            r#"query_as::<_, Row>("SELECT * FROM sessions");"#,
        ),
        (
            "use sqlx::query_as as rows;",
            r#"rows::<_, Row>("SELECT * FROM sessions");"#,
        ),
        (
            "use sqlx::query_as as rows;",
            r#"rows!(Row, "SELECT * FROM sessions");"#,
        ),
        (
            "use sqlx::query_as_unchecked as rows;",
            r#"rows!(Row, "SELECT * FROM sessions");"#,
        ),
        (
            "",
            r#"::sqlx::query_as::<_, self::Row>("SELECT * FROM sessions");"#,
        ),
    ] {
        let source = format!("{imports} struct Row {{ id: i32 }} fn load() {{ {query} }}");
        let result = audit(&source, None);
        assert_eq!(result.mapping, "source_row_query_contract", "{source}");
        assert_eq!(result.table.as_deref(), Some("sessions"));
    }
}

mod database;
