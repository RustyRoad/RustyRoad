//! Schema-validated writes through an explicit foreign-key identity lookup.
mod columns;
mod configuration;
mod query;
mod validation;
use super::allocation::Names;
use crate::database::introspection::Schema;
pub use configuration::{RelatedOperation, RelatedWrite};
use std::{collections::BTreeMap, io};

/// Generates one-statement writes and returns canonical source/related row pairs.
/// Configuration is validated before any output is emitted; generation writes no data.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    writes: &BTreeMap<String, RelatedWrite>,
) -> io::Result<String> {
    let mut output = String::new();
    for (name, write) in writes {
        validation::validate(schema, names, name, write)?;
        output.push_str(&query::render(schema, names, name, write));
    }
    Ok(output)
}
