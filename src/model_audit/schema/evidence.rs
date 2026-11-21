//! Load catalog evidence without applying SQL or creating a database.
mod database;
use super::types::CatalogEvidence;
use crate::model_audit::{cli::values, context::Context};
use clap::ArgMatches;
use std::path::Path;

/// Load a named snapshot or a bounded read-only database observation.
pub(super) fn load(
    args: &ArgMatches,
    root: &Path,
    context: &Context,
) -> Result<Option<CatalogEvidence>, String> {
    if let Some(file) = args.get_one::<String>("schema-snapshot") {
        let path = root.join(file);
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Schema snapshot {}: {e}", path.display()))?;
        let catalog =
            serde_json::from_str(&content).map_err(|e| format!("Invalid schema snapshot: {e}"))?;
        return Ok(Some(CatalogEvidence {
            source: format!("snapshot:{}", path.display()),
            catalog,
        }));
    }
    if context.catalog_mode != "database" {
        if !values(args, "table").is_empty() {
            return Err("--table requires a configured database or --schema-snapshot; --offline alone cannot establish schema gaps".into());
        }
        return Ok(None);
    }
    let catalog = database::read(
        root,
        args.get_one::<String>("schema").unwrap().clone(),
        context.config_file.clone(),
    )?;
    Ok(Some(CatalogEvidence {
        source: format!("live_postgresql:environment={}", context.environment),
        catalog,
    }))
}
