//! Extracted fixtures retain their original assertions and inputs.
use super::{audit, empty};
use crate::model_audit::schema::proposals;

/// A whole-row query proves ownership, not a primary-key contract.
#[test]
fn whole_row_source_identity_keeps_unknown_keys_blocked() {
    let source = "struct Row { id: i32 } impl Row { async fn load() {
        sqlx::query_as::<_, Self>(\"SELECT * FROM sessions WHERE id = $1\");
    } }";
    let mut result = audit(source, None);
    assert_eq!(result.table.as_deref(), Some("sessions"));
    assert_eq!(result.mapping, "source_row_query_contract");
    let model = super::super::model("struct Row { id: i32 }");
    proposals::build(&model, &empty(), &mut result, &[]);
    assert!(result.proposal.is_none());
    assert!(result.blockers.iter().any(|reason| reason.contains("key")));
}

/// SQLx macro syntax and RETURNING carry the same whole-row identity evidence.
#[test]
fn returning_macro_establishes_source_identity() {
    let result = audit(
        "struct Row { id: i32 } async fn create() {
        sqlx::query_as!(Row, \"INSERT INTO sessions (id) VALUES ($1) RETURNING *\", 1);
    }",
        None,
    );
    assert_eq!(result.table.as_deref(), Some("sessions"));
    assert_eq!(result.mapping, "source_row_query_contract");
}

/// Multiple targets remain ambiguous unless the caller supplies an override.
#[test]
fn conflicting_queries_require_explicit_ownership() {
    let source = "struct Row { id: i32 } async fn load() {
        sqlx::query_as::<_, Row>(\"SELECT * FROM first\");
        sqlx::query_as::<_, Row>(\"SELECT * FROM second\");
    }";
    let result = audit(source, None);
    assert_eq!(result.status, "ambiguous_table");
    assert_eq!(result.candidates, ["first", "second"]);
    assert!(result.table.is_none());
    let result = audit(source, Some("chosen"));
    assert_eq!(result.table.as_deref(), Some("chosen"));
    assert_eq!(result.mapping, "explicit_persistence_contract");
}
