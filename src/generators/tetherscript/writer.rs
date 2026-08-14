//! Writing generated TetherScript models into the output folder.
//!
//! Each table becomes a folder holding its own module, and the parent `mod.tether`
//! imports and re-exports them. That parent is a scaffold rather than derived output: a
//! project keeps hand-written models beside the generated ones, and rewriting the file
//! would delete their imports.

use super::model::Model;
use super::render;
use crate::database::introspection::Schema;
use crate::generators::layout::{self, Module};
use crate::generators::naming;
use std::io;
use std::path::Path;

pub use crate::generators::layout::Layout;

/// The parent module file, which the developer owns after the first run.
const ROOT: &str = "mod.tether";

/// Writes a model module per table into `out`, creating it when absent.
pub fn write(out: &Path, schema: &Schema, force: bool) -> io::Result<Layout> {
    // Names are resolved for the whole schema first: two tables can singularize to the
    // same word, and naming them independently would silently overwrite a model.
    let names = naming::resolve(schema);

    let modules: Vec<Module> = schema
        .tables
        .iter()
        .zip(&names)
        .map(|(table, names)| {
            let model = Model::resolve(table, schema, names);

            Module {
                name: model.module.clone(),
                // TetherScript has no types to export: the parent re-exports the module
                // itself, and every function reaches through it.
                has_extra_types: false,
                files: render::files(&model),
            }
        })
        .collect();

    layout::write(out, &modules, ROOT, declarations, declares, force)
}

/// Renders the parent `mod.tether` importing and re-exporting every model.
fn declarations(modules: &[Module]) -> String {
    let mut file = String::from(
        "// Database models.\n\
         //\n\
         // Written once by `rustyroad pull`, then yours: add hand-written models here\n\
         // alongside the generated ones. A model added to the database later is\n\
         // reported by `pull` rather than inserted, so your edits survive.\n\n",
    );

    for module in modules {
        file.push_str(&format!(
            "import \"./{}/mod.tether\" as {}\n",
            module.name, module.name
        ));
    }
    file.push('\n');
    for module in modules {
        file.push_str(&format!("export {}\n", module.name));
    }

    file
}

/// Returns `true` when `contents` imports the module `name`.
///
/// Matched on the import path rather than the name anywhere in the file, so a model
/// mentioned only in a comment still counts as missing.
fn declares(contents: &str, name: &str) -> bool {
    let path = format!("\"./{name}/mod.tether\"");

    contents
        .lines()
        .any(|line| line.trim_start().starts_with("import") && line.contains(&path))
}
