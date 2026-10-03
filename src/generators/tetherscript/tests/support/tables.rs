//! Fixture tables for TetherScript generation tests.

use super::columns::{column, required, serial_key};
use crate::database::introspection::Table;

/// A `products` table covering the column shapes generation must handle.
pub(in crate::generators::tetherscript::tests) fn products() -> Table {
    Table {
        name: "products".to_string(),
        columns: vec![
            serial_key("id"),
            required("name", "character varying(255)"),
            column("description", "text"),
            required("active", "boolean"),
            required("attributes", "text[]"),
            column("metadata", "jsonb"),
            column("price", "double precision"),
            column("created_at", "timestamp without time zone"),
            column("updated_at", "timestamp without time zone"),
        ],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}

/// A table whose primary key spans two columns.
pub(in crate::generators::tetherscript::tests) fn composite() -> Table {
    Table {
        name: "order_items".to_string(),
        columns: vec![
            required("order_id", "integer"),
            required("product_id", "integer"),
            required("quantity", "integer"),
        ],
        primary_key: vec!["order_id".to_string(), "product_id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}
