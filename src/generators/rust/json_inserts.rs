//! Typed atomic inserts with schema-validated JSON integer derivations.
mod configuration;
mod expressions;
mod query;
mod validation;
use super::allocation::Names;
use crate::database::introspection::Schema;
pub use configuration::{JsonInsert, JsonInteger};
use std::{collections::BTreeMap, io};

/// Validates configured writes before emitting any generated SQL or source.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    inserts: &BTreeMap<String, JsonInsert>,
) -> io::Result<String> {
    let mut output = String::new();
    for (name, insert) in inserts {
        validation::validate(schema, names, name, insert)?;
        output.push_str(&query::render(schema, names, name, insert));
    }
    Ok(output)
}
