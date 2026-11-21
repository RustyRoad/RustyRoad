//! Guarded relationship inserts are generated as a single PostgreSQL statement.
mod configuration;
mod query;
mod validation;
use super::allocation::Names;
use crate::database::introspection::Schema;
pub use configuration::RelationInsert;
use std::{collections::BTreeMap, io};

/// Validates configured relationship writes before returning generated code.
/// No database mutation occurs during generation.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    reports: &BTreeMap<String, RelationInsert>,
) -> io::Result<String> {
    let mut output = String::new();
    for (name, report) in reports {
        validation::validate(schema, names, name, report)?;
        output.push_str(&query::render(schema, names, name, report));
    }
    Ok(output)
}
