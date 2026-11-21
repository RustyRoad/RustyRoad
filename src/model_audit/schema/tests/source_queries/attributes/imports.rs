//! Namespace attributes must preserve unconditional constructor identity.
use super::audit;

/// Conditional namespace changes must not become unconditional query evidence.
#[test]
fn rejects_conditional_or_transforming_import_attributes() {
    for attribute in [
        "#[cfg(feature = \"rows\")]",
        "#[cfg_attr(feature = \"rows\", allow(unused_imports))]",
        "#[replace_import]",
    ] {
        for (import, call) in [
            ("use sqlx as db;", "db::query_as"),
            ("use sqlx::query_as as rows;", "rows"),
            ("use other::Thing;", "sqlx::query_as"),
        ] {
            let source = format!(
                "{attribute} {import}
                struct Row {{ id: i32 }} async fn load() {{
                    {call}::<_, Row>(\"SELECT * FROM sessions\");
                }}"
            );
            let result = audit(&source, None);
            assert_eq!(
                result.mapping, "unresolved_persistence_contract",
                "{attribute} {import}"
            );
            assert_eq!(result.table.as_deref(), Some("row"), "{attribute} {import}");
        }
    }
}
