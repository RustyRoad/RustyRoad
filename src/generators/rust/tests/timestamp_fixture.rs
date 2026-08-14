//! Fixtures for the timestamp-seeding tests.

use super::support::{column, required, resolve};
use crate::database::introspection::{Schema, Table};

/// Builds a table whose audit columns use `sql_type`.
pub(super) fn table(sql_type: &str, nullable: bool) -> Table {
    let audit = |name: &str| {
        if nullable {
            column(name, sql_type)
        } else {
            required(name, sql_type)
        }
    };

    Table {
        name: "events".to_string(),
        columns: vec![
            required("id", "integer"),
            audit("created_at"),
            audit("updated_at"),
        ],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}

/// Renders the module root for that table.
pub(super) fn root(sql_type: &str, nullable: bool) -> String {
    let built = table(sql_type, nullable);
    let schema = Schema {
        tables: vec![built.clone()],
        enums: Vec::new(),
    };
    let model = resolve(&built, &schema);

    crate::generators::rust::render::files(&model, &schema)
        .into_iter()
        .find(|file| file.name == "mod.rs")
        .unwrap()
        .contents
}
