//! Shared fixtures for generation tests.

mod tables;

pub(super) use tables::{posts, users};

use crate::database::introspection::{Column, Enum, Schema, Table};

/// Builds a nullable column with no default.
pub(super) fn column(name: &str, sql_type: &str) -> Column {
    Column {
        name: name.to_string(),
        sql_type: sql_type.to_string(),
        nullable: true,
        default: None,
        auto_increment: false,
    }
}

/// Builds a serial primary key column.
pub(super) fn serial_key(name: &str) -> Column {
    Column {
        nullable: false,
        auto_increment: true,
        default: Some("nextval('users_id_seq'::regclass)".to_string()),
        ..column(name, "integer")
    }
}

/// Wraps tables in a schema with no enum types.
pub(super) fn from_tables(tables: Vec<Table>) -> Schema {
    Schema {
        tables,
        enums: Vec::new(),
    }
}

/// Wraps tables and enum types in a schema.
pub(super) fn with_enums(tables: Vec<Table>, enums: Vec<Enum>) -> Schema {
    Schema { tables, enums }
}

/// Builds an enum type.
pub(super) fn enum_type(name: &str, values: &[&str]) -> Enum {
    Enum {
        name: name.to_string(),
        values: values.iter().map(|value| value.to_string()).collect(),
    }
}

/// A schema containing both fixture tables.
pub(super) fn schema() -> Schema {
    from_tables(vec![users(), posts()])
}
