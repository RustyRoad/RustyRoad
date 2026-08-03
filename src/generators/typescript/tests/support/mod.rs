//! Shared fixtures for generation tests.

mod tables;

pub(super) use tables::{posts, users};

use crate::database::introspection::{Column, Schema};

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

/// A schema containing both fixture tables.
pub(super) fn schema() -> Schema {
    Schema {
        tables: vec![users(), posts()],
    }
}
