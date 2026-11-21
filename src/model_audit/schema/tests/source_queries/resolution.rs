//! Constructor aliases retain their source identity without basename guessing.
use super::audit;

/// Imported aliases do not bypass lexical shadowing or conditional bindings.
#[test]
fn rejects_unsafe_constructor_aliases() {
    for source in [
        r#"use sqlx::query_as as rows; struct Row { id: i32 }
            fn load() { let rows = other; rows::<_, Row>("SELECT * FROM sessions"); }"#,
        r#"use sqlx as db; struct Row { id: i32 }
            fn load(db: Other) { db::query_as::<_, Row>("SELECT * FROM sessions"); }"#,
        r#"#[cfg(feature = "db")] use sqlx::query_as as rows; struct Row { id: i32 }
            fn load() { rows::<_, Row>("SELECT * FROM sessions"); }"#,
        r#"use other::query_as as rows; struct Row { id: i32 }
            fn load() { rows::<_, Row>("SELECT * FROM sessions"); }"#,
        r#"use sqlx::query_as as rows; struct Row { id: i32 }
            fn rows() {} fn load() { rows::<_, Row>("SELECT * FROM sessions"); }"#,
        r#"use sqlx as db; struct Row { id: i32 }
            fn load() { let (_, db) = x; db::query_as::<_, Row>("SELECT * FROM sessions"); }"#,
    ] {
        assert_eq!(
            audit(source, None).mapping,
            "unresolved_persistence_contract",
            "{source}"
        );
    }
}

/// Parent-module imports are followed only through an explicit `super` path.
#[test]
fn resolves_aliases_through_the_exact_inline_scope() {
    let result = audit(
        r#"use sqlx as db; mod nested {
        struct Row { id: i32 }
        fn load() { super::db::query_as::<_, Row>("SELECT * FROM sessions"); }
    }"#,
        None,
    );
    assert_eq!(result.mapping, "source_row_query_contract");
    assert_eq!(result.table.as_deref(), Some("sessions"));
}

mod aliases;

mod paths;
