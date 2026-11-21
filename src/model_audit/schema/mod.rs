//! Live or snapshot-backed table comparison, distinct from Rust declaration comparison.
mod arguments;
mod catalog;
mod composition;
mod connection;
mod contracts;
mod evidence;
mod groups;
mod live;
mod matching;
mod projections;
mod proposals;
mod reconciliation;
pub(crate) mod render;
mod rust_types;
#[cfg(test)]
mod tests;
mod type_mappings;
pub(crate) mod types;
use crate::model_audit::{
    context::Context,
    types::{Candidate, Model},
};
use clap::{ArgMatches, Command};
use std::path::Path;

/// Add catalog and storage-contract options to the model audit command.
pub(crate) fn arguments(command: Command) -> Command {
    command.args(arguments::args())
}

/// Attach database evidence before selection so shared-table conflicts remain visible.
pub(crate) fn attach(
    args: &ArgMatches,
    root: &Path,
    context: &Context,
    models: &[Model],
    candidates: &mut [Candidate],
) -> Result<Option<types::CatalogEvidence>, String> {
    let evidence = match evidence::load(args, root, context)? {
        Some(evidence) => evidence,
        None if !super::cli::values(args, "storage-projection").is_empty() => {
            return Err(
                "--storage-projection requires database or schema-snapshot evidence".into(),
            );
        }
        None => return Ok(None),
    };
    reconciliation::attach(args, root, models, candidates, &evidence.catalog)?;
    Ok(Some(evidence))
}
