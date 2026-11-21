//! Repository implementations may name rows, but cannot redefine their identity.
use super::audit;

/// Example: an inherent helper's typed whole-row query owns `Row`, not the helper.
#[test]
fn recognizes_explicit_rows_in_repository_implementations() {
    for call in [
        r#"sqlx::query_as::<_, Row>("SELECT * FROM sessions");"#,
        r#"sqlx::query_as!(self::Row, "SELECT * FROM sessions");"#,
    ] {
        let source = format!(
            "struct Row {{ id: i32 }} struct Repository;
            impl Repository {{ fn load() {{ {call} }} }}"
        );
        let result = audit(&source, None);
        assert_eq!(result.mapping, "source_row_query_contract", "{source}");
        assert_eq!(result.table.as_deref(), Some("sessions"));
    }
}

/// Different repository implementations cannot silently pick a conflicting table.
#[test]
fn retains_conflicts_between_free_and_repository_queries() {
    let result = audit(
        r#"struct Row { id: i32 }
        fn load() { sqlx::query_as::<_, Row>("SELECT * FROM sessions"); }
        impl Repository { fn load() {
            sqlx::query_as::<_, Row>("SELECT * FROM archived_sessions");
        } }"#,
        None,
    );
    assert_eq!(result.mapping, "conflicting_source_row_queries");
    assert_eq!(result.status, "ambiguous_table");
    assert!(result.table.is_none());
    assert_eq!(result.candidates, ["archived_sessions", "sessions"]);
}

mod rejected;
