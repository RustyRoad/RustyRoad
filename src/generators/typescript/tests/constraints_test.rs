//! Table-level constraints in `schema.ts`.

use super::support::{column, from_tables, schema, users};
use crate::database::introspection::Table;
use crate::generators::typescript::schema::render;
use crate::generators::typescript::Casing;

#[test]
fn constraints_are_rendered_when_present() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("index(\"users_active_idx\")"));
    assert!(ts.contains("unique(\"users_email_key\")"));
    assert!(ts.contains("foreignKey({"));
    assert!(ts.contains(".onDelete(\"cascade\")"));
}

#[test]
fn tables_without_constraints_have_no_second_argument() {
    let mut table = users();
    table.uniques.clear();
    table.indexes.clear();

    let ts = render(&from_tables(vec![table]), Casing::Camel);
    assert!(!ts.contains("(table) => ["));
}

#[test]
fn bare_tables_need_no_constraint_imports() {
    let bare = from_tables(vec![Table {
        name: "logs".to_string(),
        columns: vec![column("body", "text")],
        primary_key: Vec::new(),
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }]);

    let ts = render(&bare, Casing::Camel);
    assert!(!ts.contains("foreignKey"));
    assert!(!ts.contains("primaryKey"));
}

#[test]
fn composite_primary_keys_become_table_level() {
    let table = Table {
        name: "memberships".to_string(),
        columns: vec![column("user_id", "integer"), column("group_name", "text")],
        primary_key: vec!["user_id".to_string(), "group_name".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    };

    let ts = render(&from_tables(vec![table]), Casing::Camel);

    // A composite key cannot be expressed on a single column builder.
    assert!(ts.contains("primaryKey({ columns: [table.userId, table.groupName]"));
    assert!(!ts.contains(".primaryKey()"));
}
