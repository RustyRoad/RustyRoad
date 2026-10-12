//! Writing the Rust models and the `db_pool` module they query through.
//!
//! Kept beside, not inside, the other model languages: only Rust output needs a crate-level
//! companion module, and without it every generated CRUD method fails to compile.

use super::report;
use crate::database::introspection::Schema;
use crate::generators::rust::{self, PoolModule};
use std::path::Path;

/// Writes the models into `out`, then the `db_pool` scaffold beside the crate root.
pub(super) fn write(out: &Path, schema: &Schema, options: rust::Options, force: bool) {
    match rust::write_with(out, schema, options, force) {
        Ok(layout) => report::models("Rust models", &layout),
        Err(error) => report::fail(&error.to_string()),
    }

    match rust::write_pool_module(out, force) {
        Ok(module) => pool(&module),
        Err(error) => report::fail(&error.to_string()),
    }
}

/// Reports the scaffold, warning when the crate root does not declare it.
fn pool(module: &PoolModule) {
    report::artifacts("Database pool", std::slice::from_ref(&module.outcome));

    let Some(root) = &module.undeclared_in else {
        return;
    };
    println!(
        "\nWarning: {} does not declare the db_pool module the generated models query \
         through. Add:\n\
        \x20 mod db_pool;\n\
         and depend on rustyroad in Cargo.toml.",
        root.display()
    );
}
