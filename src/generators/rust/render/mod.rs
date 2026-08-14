//! Rendering the files of one model's module.
//!
//! One renderer per generated file, matching the layout the hand-written models use:
//! the module root declares the struct, and each CRUD verb gets its own submodule
//! contributing an `impl` block. Splitting by verb keeps each file short enough to
//! read whole, and gives a developer an obvious place to look for one statement.

mod create;
mod delete;
pub(super) mod enums;
mod imports;
mod order;
mod read;
mod root;
mod sql;
mod update;

use super::model::Model;
use crate::database::introspection::Schema;
use crate::generators::layout::File;
use crate::generators::ownership::Ownership;

/// Renders every file of a model's module.
///
/// All are derived from the database and so are rewritten on each run; the place for
/// hand-written code is a sibling module the generator never names.
///
/// A view gets only the root and the reads: Postgres rejects writes to one, so emitting
/// `create`/`update`/`delete` would advertise methods that can only fail at runtime.
pub fn files(model: &Model, schema: &Schema) -> Vec<File> {
    let mut files = vec![
        generated("mod.rs", root::render(model, schema)),
        generated("read.rs", read::render(model, schema)),
    ];

    if !model.view {
        files.push(generated("create.rs", create::render(model)));
        files.push(generated("update.rs", update::render(model, schema)));
        files.push(generated("delete.rs", delete::render(model, schema)));
    }

    if model.actix {
        files.push(generated("routes.rs", super::actix::render(model, schema)));
    }

    files
}

/// Builds an entry for a file derived from the database.
fn generated(name: &'static str, contents: String) -> File {
    File {
        name,
        ownership: Ownership::Generated,
        contents,
    }
}
