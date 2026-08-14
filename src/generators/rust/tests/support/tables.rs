//! Fixture tables for Rust generation tests.

use super::columns::{column, required, serial_key};
use crate::database::introspection::{ForeignKey, Table};

/// A `products` table mirroring the hand-written reference model.
pub(in crate::generators::rust::tests) fn products() -> Table {
    Table {
        name: "products".to_string(),
        columns: vec![
            serial_key("id"),
            required("stripe_product_id", "character varying(255)"),
            column("name", "character varying(255)"),
            column("description", "text"),
            required("active", "boolean"),
            required("attributes", "text[]"),
            column("created_at", "timestamp without time zone"),
            column("updated_at", "timestamp without time zone"),
            column("metadata", "jsonb"),
            column("price", "double precision"),
        ],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}

/// A table whose primary key spans two columns.
pub(in crate::generators::rust::tests) fn composite() -> Table {
    Table {
        name: "order_items".to_string(),
        columns: vec![
            required("order_id", "integer"),
            required("product_id", "integer"),
            required("quantity", "integer"),
        ],
        primary_key: vec!["order_id".to_string(), "product_id".to_string()],
        foreign_keys: vec![ForeignKey {
            name: "order_items_product_id_fkey".to_string(),
            columns: vec!["product_id".to_string()],
            foreign_table: "products".to_string(),
            foreign_columns: vec!["id".to_string()],
            on_delete: None,
            on_update: None,
        }],
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}
