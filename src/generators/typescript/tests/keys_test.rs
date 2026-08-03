//! Primary key typing in the generated repository.

use super::support::{column, schema};
use crate::database::introspection::{Schema, Table};
use crate::generators::typescript::client::render;
use crate::generators::typescript::Casing;

#[test]
fn integer_keys_are_typed_as_numbers() {
    let ts = render(&schema(), Casing::Camel);

    // users.id is serial, so lookups take a number.
    assert!(ts.contains("find(db: Database, id: number)"));
}

#[test]
fn uuid_keys_are_typed_as_strings() {
    let table = Table {
        name: "sessions".to_string(),
        columns: vec![column("id", "uuid")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
    };

    let ts = render(&Schema { tables: vec![table] }, Casing::Camel);
    assert!(ts.contains("find(db: Database, id: string)"));
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
    };

    let ts = render(&Schema { tables: vec![table] }, Casing::Camel);

    // Row types are still exported; CRUD by id is not expressible.
    assert!(ts.contains("AuditLogRow"));
    assert!(!ts.contains("auditLogRepository"));
}
