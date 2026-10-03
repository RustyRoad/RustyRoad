//! Writing generated Rust models into the output folder.
//!
//! Each table becomes a folder holding its own module, and the parent `mod.rs` declares
//! them. That parent is a scaffold rather than derived output: a project keeps hand-written
//! models beside the generated ones, and rewriting the file would delete their declarations.
//! So the generator writes it once, then only reports what a developer needs to add.

mod parent;
mod shared;

use super::model::Model;
use super::render;
use crate::database::introspection::Schema;
use crate::generators::layout::{self, Module};
use crate::generators::naming;
use std::io;
use std::path::Path;

pub use crate::generators::layout::Layout;

/// The parent module file, which the developer owns after the first run.
const ROOT: &str = "mod.rs";

/// What to emit alongside the models.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Options {
    /// Emit actix-web handlers with utoipa annotations.
    pub actix: bool,
}

/// Writes a model module per table into `out`, creating it when absent.
pub fn write(out: &Path, schema: &Schema, force: bool) -> io::Result<Layout> {
    write_with(out, schema, Options::default(), force)
}

/// Writes the models, plus whatever `options` asks for.
pub fn write_with(
    out: &Path,
    schema: &Schema,
    options: Options,
    force: bool,
) -> io::Result<Layout> {
    // Names are resolved for the whole schema first: two tables can singularize to the
    // same word, and naming them independently would silently overwrite a model.
    let names = naming::resolve(schema);

    let models: Vec<Model> = schema
        .tables
        .iter()
        .zip(&names)
        .map(|(table, names)| Model::resolve(table, schema, names).with_actix(options.actix))
        .collect();

    // Shared modules come first so the parent declares them before the models that use them.
    let mut modules = shared::enums_module(&models, schema);

    let model_modules: Vec<Module> = models
        .iter()
        .map(|model| Module {
            name: model.module.clone(),
            // The model itself is the only name a model module exports; its enum types live
            // in the shared module, which is what carries the glob.
            has_extra_types: false,
            files: render::files(model, schema),
        })
        .collect();

    modules.extend(shared::api_module(&model_modules, options));
    modules.extend(model_modules);

    layout::write(out, &modules, ROOT, parent::render, declares, force)
}

/// Returns `true` when `contents` declares the module `name`.
///
/// Matched on the declaration rather than the name anywhere in the file, so a model mentioned
/// only in a comment still counts as missing.
fn declares(contents: &str, name: &str) -> bool {
    contents.lines().any(|line| {
        let line = line.trim();
        line == format!("pub mod {name};") || line == format!("mod {name};")
    })
}
