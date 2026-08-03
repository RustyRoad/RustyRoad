//! The `users` fixture, exercising nullability, defaults, and awkward types.

use super::super::{column, serial_key};
use crate::database::introspection::{Column, Index, Table, Unique};

/// A `users` table mirroring what live Postgres reports for these columns.
pub(in crate::generators::typescript::tests) fn users() -> Table {
    Table {
        name: "users".to_string(),
        columns: columns(),
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: vec![Unique {
            name: "users_email_key".to_string(),
            columns: vec!["email_address".to_string()],
        }],
        indexes: vec![Index {
            name: "users_active_idx".to_string(),
            columns: vec!["is_active".to_string()],
            unique: false,
        }],
    }
}

/// Builds the column list.
fn columns() -> Vec<Column> {
    vec![
        serial_key("id"),
        Column {
            nullable: false,
            ..column("email_address", "character varying(255)")
        },
        // Nullable, so schemas should express a type union.
        column("display_name", "text"),
        Column {
            default: Some("true".to_string()),
            nullable: false,
            ..column("is_active", "boolean")
        },
        // Live Postgres reports `0` here; numeric must be quoted as a string.
        Column {
            default: Some("0".to_string()),
            ..column("balance", "numeric(12,2)")
        },
        Column {
            nullable: false,
            default: Some("now()".to_string()),
            ..column("created_at", "timestamp(3) with time zone")
        },
        column("external_id", "uuid"),
        column("metadata", "jsonb"),
    ]
}
