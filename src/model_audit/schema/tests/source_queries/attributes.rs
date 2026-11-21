//! Conditional declarations/imports cannot prove unconditional table ownership.
use super::audit;

/// Even a direct SQLx call does not remove uncertainty in the row declaration.
#[test]
fn rejects_conditional_or_transforming_model_attributes() {
    for attribute in [
        "#[cfg(feature = \"rows\")]",
        "#[cfg_attr(feature = \"rows\", derive(Clone))]",
        "#[replace_model]",
    ] {
        let source = format!(
            "{attribute} struct Row {{ id: i32 }}
            async fn load() {{
                sqlx::query_as::<_, Row>(\"SELECT * FROM sessions\");
            }}"
        );
        let result = audit(&source, None);
        assert_eq!(
            result.mapping, "unresolved_persistence_contract",
            "{attribute}"
        );
        // An unresolved model retains its tentative snake-case name, not the query target.
        assert_eq!(result.table.as_deref(), Some("row"), "{attribute}");
    }
}

/// Documentation and lint controls leave the source identity unchanged.
#[test]
fn accepts_documentation_and_lint_attributes() {
    let result = audit(
        "#[allow(unused_imports)] use sqlx as db;
        #[doc = \"Persisted row\"] #[allow(dead_code)] struct Row { id: i32 }
        #[allow(dead_code)] async fn load() {
            db::query_as::<_, Row>(\"SELECT * FROM sessions\");
        }",
        None,
    );
    assert_eq!(result.mapping, "source_row_query_contract");
    assert_eq!(result.table.as_deref(), Some("sessions"));
}

mod imports;
