//! The `products` fixture with an enum-typed column.

use super::support::{enum_type, required, resolve, serial_key, with_enums};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::render;

/// A `products` table with an enum-typed column.
pub(super) fn table() -> Table {
    Table {
        name: "products".to_string(),
        columns: vec![serial_key("id"), required("status", "product_status")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}

/// The schema holding that table and its enum type.
pub(super) fn schema() -> Schema {
    with_enums(
        vec![table()],
        vec![enum_type("product_status", &["active", "archived"])],
    )
}

/// Renders the module root for the enum fixture.
pub(super) fn root() -> String {
    let schema = schema();
    let model = resolve(&table(), &schema);

    render::files(&model, &schema)
        .into_iter()
        .find(|file| file.name == "mod.rs")
        .expect("mod.rs should be emitted")
        .contents
}

/// Renders the shared `enums` module for the enum fixture, via a real write.
pub(super) fn shared_enums() -> String {
    shared_for(&schema())
}

/// Renders the shared `enums` module for any schema, via a real write.
pub(super) fn shared_for(schema: &Schema) -> String {
    let out = super::support::scratch("shared-enums");
    crate::generators::rust::write(&out, schema, false).expect("write should succeed");

    let contents = std::fs::read_to_string(out.join("enums").join("mod.rs"))
        .expect("the shared enums module should be written");
    let _ = std::fs::remove_dir_all(&out);

    contents
}
