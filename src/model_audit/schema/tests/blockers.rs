//! Ambiguous or unsafe contracts remain explicit blockers.
use super::{model, rows};
use crate::model_audit::schema::{contracts, matching, proposals, types::Catalog};

#[test]
fn views_and_nullable_primary_keys_remain_blocked() {
    for (source, kind) in [
        ("struct Rows { id: i64 }", "view"),
        ("struct Rows { id: Option<i32> }", "table"),
    ] {
        let catalog = rows(kind);
        let desired = model(source);
        let mut audit = matching::audit(&desired, &catalog, None);
        proposals::build(&desired, &catalog, &mut audit, &[]);
        assert!(audit.proposal.is_none());
        assert!(!audit.blockers.is_empty());
    }
}

#[test]
fn incomplete_catalog_is_not_absence_evidence() {
    let raw = r#"{"version":1,"schema":"public","tables":[]}"#;
    assert!(serde_json::from_str::<Catalog>(raw).is_err());
    let mut catalog = super::empty();
    catalog.complete = false;
    let desired = model("struct NewTable { id: i32 }");
    let mut audit = matching::audit(&desired, &catalog, None);
    proposals::build(&desired, &catalog, &mut audit, &[]);
    assert!(audit.proposal.is_none());
    catalog.version = 2;
    assert!(contracts::validate(&catalog).is_err());
}

#[test]
fn unsafe_catalog_sql_is_rejected() {
    let mut catalog = rows("table");
    catalog.tables[0].columns[0].sql_type = "text; DROP TABLE users".into();
    assert!(proposal(&catalog).is_none());
}

#[test]
fn cast_keyword_payload_cannot_escape_type_positions() {
    let mut catalog = rows("table");
    catalog.tables[0].columns[0].sql_type = "integer USING dangerous()".into();
    assert!(proposal(&catalog).is_none());
}

#[test]
fn identifier_length_and_postgres_builtin_type_are_checked() {
    assert!(matching::identifier(&"a".repeat(64)).is_err());
    let mut catalog = rows("table");
    catalog.tables[0].columns[0].sql_type = "float8".into();
    assert!(proposal(&catalog).is_some());
}

#[test]
fn numeric_postgres_type_is_accepted() {
    let mut catalog = rows("table");
    catalog.tables[0].columns[0].sql_type = "numeric(12, 2)".into();
    assert!(proposal(&catalog).is_some());
}

/// Exercise the planner boundary where observed types become rollback SQL.
fn proposal(catalog: &Catalog) -> Option<super::super::types::Proposal> {
    let desired = model("struct Rows { id: i64 }");
    let mut audit = matching::audit(&desired, catalog, None);
    proposals::build(&desired, catalog, &mut audit, &[]);
    audit.proposal
}
