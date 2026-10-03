//! The shared module holding one declaration per Postgres enum type.

use crate::database::introspection::{Enum, Schema};
use crate::generators::layout::{File, Module};
use crate::generators::ownership::Ownership;
use crate::generators::rust::model::Model;
use crate::generators::rust::render;

/// Builds the `enums` module, or nothing when no model references an enum.
///
/// Declared once for the whole schema: seven tables can share one Postgres type, and
/// per-model copies were seven incompatible Rust types.
pub(super) fn enums_module(models: &[Model], schema: &Schema) -> Vec<Module> {
    let referenced = referenced(models, schema);

    if referenced.is_empty() {
        return Vec::new();
    }

    vec![Module {
        name: "enums".to_string(),
        // The module's whole purpose is exporting types, so the parent globs it.
        has_extra_types: true,
        files: vec![File {
            name: "mod.rs",
            ownership: Ownership::Generated,
            contents: render::enums::module(&referenced),
        }],
    }]
}

/// Returns every enum any model references, in schema order and without duplicates.
///
/// Schema order rather than reference order, so the emitted file is stable across runs
/// whatever order the tables were introspected in.
fn referenced<'a>(models: &[Model], schema: &'a Schema) -> Vec<&'a Enum> {
    schema
        .enums
        .iter()
        .filter(|item| {
            models
                .iter()
                .any(|model| model.enums(schema).iter().any(|e| e.name == item.name))
        })
        .collect()
}

/// Builds the `api` module, or nothing when actix output is off.
pub(super) fn api_module(models: &[Module], options: super::Options) -> Vec<Module> {
    if !options.actix || models.is_empty() {
        return Vec::new();
    }

    vec![Module {
        name: "api".to_string(),
        // `configure` and `ApiDoc` are both named by the application wiring them up.
        has_extra_types: true,
        files: vec![File {
            name: "mod.rs",
            ownership: Ownership::Generated,
            contents: crate::generators::rust::actix::parent(models),
        }],
    }]
}
