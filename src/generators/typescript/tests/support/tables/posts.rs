//! The `posts` fixture, for relation and declaration-ordering coverage.

use super::super::{column, serial_key};
use crate::database::introspection::{Column, ForeignKey, Table};

/// A `posts` table referencing `users` with a cascading delete.
pub(in crate::generators::typescript::tests) fn posts() -> Table {
    Table {
        name: "posts".to_string(),
        columns: vec![
            serial_key("id"),
            Column {
                nullable: false,
                ..column("author_id", "integer")
            },
            column("body", "text"),
        ],
        primary_key: vec!["id".to_string()],
        foreign_keys: vec![ForeignKey {
            name: "posts_author_id_fkey".to_string(),
            columns: vec!["author_id".to_string()],
            foreign_table: "users".to_string(),
            foreign_columns: vec!["id".to_string()],
            on_delete: Some("cascade".to_string()),
            on_update: None,
        }],
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}
