//! Explicit conditional aggregate projections over existing catalog relations.
mod configuration;
mod expressions;
mod predicate;
mod query;
mod validation;
use super::allocation::Names;
use crate::database::introspection::Schema;
pub use configuration::{ConditionalMetric, GroupMetrics};
use std::{collections::BTreeMap, io};

/// Validates configured conditional aggregates before producing repository code.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    reports: &BTreeMap<String, GroupMetrics>,
) -> io::Result<String> {
    let mut output = String::new();
    for (name, report) in reports {
        validation::validate(schema, names, name, report)?;
        output.push_str(&query::render(schema, names, name, report));
    }
    Ok(output)
}
