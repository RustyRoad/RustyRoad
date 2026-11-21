//! Rust pull output with project-configured extension codecs.
use super::report;
use crate::database::introspection::{PartialUniqueIndex, Schema};
use crate::generators::{model_paths, rust};
use clap::ArgMatches;
use std::path::PathBuf;

/// Reject configured targets even when table filtering removes the entire catalog.
/// No writer or database operation is invoked for an empty selection.
/// # Examples
/// ```sh
/// rustyroad pull --schema public --language rust --tables parcels --out crates/db/src
/// ```
pub(super) fn check_empty() -> Result<(), String> {
    let root = std::env::current_dir().map_err(|error| error.to_string())?;
    let settings = model_paths::settings(&root)?;
    if settings.rust_column_types.is_empty() {
        Ok(())
    } else {
        Err("Scoped column codec target table outside pull scope (empty catalog)".into())
    }
}

/// Validate project codec settings and preserve explicit output precedence.
/// Report configuration or filesystem failures through the pull runtime.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn write(
    matches: &ArgMatches,
    schema: &Schema,
    force: bool,
    partial_keys: &[PartialUniqueIndex],
) {
    let root = match std::env::current_dir() {
        Ok(root) => root,
        Err(error) => return report::fail(&error.to_string()),
    };
    let settings = match model_paths::settings(&root) {
        Ok(settings) => settings,
        Err(error) => return report::fail(&error),
    };
    let out = if let Some(path) = matches.get_one::<String>("out") {
        PathBuf::from(path)
    } else {
        match model_paths::rust_output(&root) {
            Ok(path) => path,
            Err(error) => return report::fail(&error),
        }
    };
    let namespace = matches
        .get_one::<String>("schema")
        .map(String::as_str)
        .unwrap_or(super::DEFAULT_SCHEMA);
    match rust::write_api_with_scoped_generation(
        &out,
        schema,
        force,
        &settings.rust_types,
        &settings.rust_group_stats,
        &settings.rust_spatial_reports,
        &settings.rust_group_metrics,
        &settings.rust_relation_inserts,
        &settings.rust_related_writes,
        &settings.rust_projections,
        &settings.rust_balances,
        &settings.rust_url_stats,
        &settings.rust_json_inserts,
        partial_keys,
        namespace,
        &settings.rust_column_types,
        settings.rust_relationship_reads.unwrap_or(true),
    ) {
        Ok(outcomes) => report::artifacts("Rust API", &outcomes),
        Err(error) => report::fail(&error.to_string()),
    }
}
