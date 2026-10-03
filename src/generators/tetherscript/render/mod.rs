//! Rendering the files of one model's module.
//!
//! The same split the Rust generator uses — one file per CRUD verb, plus a module
//! root — so a project reading both sees one layout rather than two.

mod create;
mod delete;
mod order;
mod read;
mod root;
mod sql;
mod update;

use super::model::Model;
use crate::generators::layout::File;
use crate::generators::ownership::Ownership;

/// Renders every file of a model's module.
pub fn files(model: &Model) -> Vec<File> {
    vec![
        generated("mod.tether", root::render(model)),
        generated("create.tether", create::render(model)),
        generated("read.tether", read::render(model)),
        generated("update.tether", update::render(model)),
        generated("delete.tether", delete::render(model)),
    ]
}

/// Builds an entry for a file derived from the database.
fn generated(name: &'static str, contents: String) -> File {
    File {
        name,
        ownership: Ownership::Generated,
        contents,
    }
}
