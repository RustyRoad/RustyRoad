//! Narrow inner joins retain catalog identities without materializing whole rows.
mod fields;
mod query;
mod validation;
use super::{Names, Projection};
use crate::database::introspection::{Schema, Table};
use std::io;

/// Validates a configured foreign-key projection before emitting its typed operation.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    table: &Table,
    projection: &Projection,
) -> io::Result<String> {
    validation::validate(schema, table, projection)?;
    Ok(query::render(schema, names, function, table, projection))
}
