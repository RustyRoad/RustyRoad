//! `pgEnum` declarations for user-defined enum types.

use super::support::{column, enum_type, serial_key, with_enums};
use crate::database::introspection::{Column, Schema, Table};
use crate::generators::typescript::schema::render;
use crate::generators::typescript::Casing;

/// A table with two enum columns, plus the enum type they reference.
pub(super) fn orders() -> Schema {
    let table = Table {
        name: "orders".to_string(),
        columns: vec![
            serial_key("id"),
            Column {
                nullable: false,
                default: Some("'pending'::order_status".to_string()),
                ..column("status", "order_status")
            },
            column("fulfilment", "order_status"),
        ],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
    };

    with_enums(
        vec![table],
        vec![enum_type(
            "order_status",
            &["pending", "shipped", "delivered"],
        )],
    )
}

#[test]
fn enum_types_are_declared() {
    let ts = render(&orders(), Casing::Camel);

    // Without this the column degrades to text and the allowed values are lost.
    assert!(ts.contains(
        r#"export const orderStatus = pgEnum("order_status", ["pending", "shipped", "delivered"]);"#
    ));
}

#[test]
fn enum_columns_reference_their_declaration() {
    let ts = render(&orders(), Casing::Camel);

    assert!(ts.contains(r#"status: orderStatus("status")"#));
    assert!(ts.contains(r#"fulfilment: orderStatus("fulfilment")"#));
    // The column must not fall back to a plain text builder.
    assert!(!ts.contains(r#"status: text("status")"#));
}

#[test]
fn enums_are_declared_before_the_tables_using_them() {
    let ts = render(&orders(), Casing::Camel);

    // A `const` is not hoisted, so a table referencing an enum declared later
    // would fail at module evaluation.
    let enum_at = ts.find("export const orderStatus").expect("enum declared");
    let table_at = ts.find("export const orders").expect("table declared");
    assert!(enum_at < table_at);
}
