//! Fixtures for the enum-variant tests.

use super::support::{enum_type, required, resolve, serial_key, with_enums};
use crate::database::introspection::{Schema, Table};

/// A table whose enum column has labels that fold together.
pub(super) fn table() -> Table {
    Table {
        name: "nodes".to_string(),
        columns: vec![serial_key("id"), required("kind", "node_kind")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}

/// Snake and kebab spellings of the same words, which is legal in Postgres.
pub(super) fn schema() -> Schema {
    with_enums(
        vec![table()],
        vec![enum_type(
            "node_kind",
            &["campaign_type", "campaign-type", "phone-call"],
        )],
    )
}

/// A table keyed by its enum column.
pub(super) fn keyed() -> Table {
    Table {
        name: "catalog".to_string(),
        columns: vec![required("kind", "node_kind"), required("label", "text")],
        primary_key: vec!["kind".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}

/// The schema holding the keyed table.
pub(super) fn keyed_schema() -> Schema {
    with_enums(
        vec![keyed()],
        vec![enum_type("node_kind", &["campaign_type", "phone-call"])],
    )
}

/// Renders one file for a table in its schema.
pub(super) fn file(table: &Table, schema: &Schema, name: &str) -> String {
    let model = resolve(table, schema);

    crate::generators::rust::render::files(&model, schema)
        .into_iter()
        .find(|file| file.name == name)
        .unwrap_or_else(|| panic!("{name} should be emitted"))
        .contents
}

/// Renders the shared `enums` module for the folding-label fixture.
pub(super) fn shared() -> String {
    super::enum_fixture::shared_for(&schema())
}
