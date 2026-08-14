//! `mod.rs` generation: the struct, its derives, and its constructors.
//!
//! Mirrors the hand-written models: the module root declares the shape of a row and
//! nothing else, so the CRUD submodules are the only place SQL appears. The struct lives
//! here rather than in a `model.rs` submodule because callers name the type, not its file,
//! and re-exporting a struct through two modules earns nothing.

mod constructors;
mod declare;
pub(super) mod fields;

use crate::database::introspection::Schema;
use crate::generators::rust::model::Model;

/// Renders the module root for one model.
pub fn render(model: &Model, schema: &Schema) -> String {
    let mut file = fields::imports(model);

    file.push_str(&declare::submodules(model));

    // Enum types live in the shared `enums` module — seven tables can reference one type,
    // and per-model copies were seven incompatible Rust types — so they are imported here
    // rather than declared.
    let imported = declare::enum_imports(model, schema);
    if !imported.is_empty() {
        file.push_str(&imported);
        file.push('\n');
    }

    file.push_str(&declare::declaration(model));

    // A view is read-only, but callers still need to scope collection reads without loading
    // the entire relation. Every projected column therefore becomes an optional SQL filter.
    if model.view {
        file.push('\n');
        file.push_str(&declare::all_options(model));
    } else {
        file.push('\n');
        file.push_str(&constructors::default_impl(model));
        file.push('\n');
        file.push_str(&constructors::new(model));
    }

    file
}
