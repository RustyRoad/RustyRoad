//! Primary key typing in the generated repository.

use super::support::{column, from_tables, schema};
use crate::database::introspection::Table;
use crate::generators::typescript::client::render;
use crate::generators::typescript::Casing;

#[test]
fn integer_keys_are_inferred_from_the_row() {
    let ts = render(&schema(), Casing::Camel);

    // The inferred field resolves to number while remaining coupled to Drizzle.
    assert!(ts.contains(r#"find(db: Database, id: UsersRow["id"])"#));
}

#[test]
fn uuid_keys_are_inferred_from_the_row() {
    let table = Table {
        name: "sessions".to_string(),
        columns: vec![column("id", "uuid")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    };

    let ts = render(&from_tables(vec![table]), Casing::Camel);
    assert!(ts.contains(r#"find(db: Database, id: SessionsRow["id"])"#));
}

#[test]
fn tables_without_a_primary_key_get_no_repository() {
    let table = Table {
        name: "audit_log".to_string(),
        columns: vec![column("body", "text")],
        primary_key: Vec::new(),
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    };

    let ts = render(&from_tables(vec![table]), Casing::Camel);

    // Row types are still exported; CRUD by id is not expressible.
    assert!(ts.contains("AuditLogRow"));
    assert!(!ts.contains("auditLogRepository"));
}
