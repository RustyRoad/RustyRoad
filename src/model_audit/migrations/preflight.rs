//! Validate report destinations and selected persistence mappings before writes.
use crate::model_audit::{schema::types::TableAudit, types::Report};
use clap::ArgMatches;
use std::fs;

/// Require selected mappings and a create-only report destination, e.g. `--output audit.json`.
pub(super) fn mapped<'a>(
    report: &'a Report,
    args: &ArgMatches,
) -> Result<Vec<&'a TableAudit>, String> {
    if let Some(output) = args.get_one::<String>("output") {
        let path = report.root.join(output);
        match fs::symlink_metadata(&path) {
            Ok(_) => return Err(format!("Report output {} already exists", path.display())),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(format!("Cannot inspect {}: {error}", path.display())),
        }
        if !path.parent().is_some_and(|parent| parent.is_dir()) {
            return Err(format!(
                "Report output parent does not exist: {}",
                path.display()
            ));
        }
    }
    if let Some(mappings) = args.get_many::<String>("table") {
        for mapping in mappings {
            let (model, _) = mapping
                .split_once('=')
                .ok_or("--table expects MODEL=TABLE")?;
            if !report
                .candidates
                .iter()
                .any(|candidate| candidate.model.id == model || candidate.model.name == model)
            {
                return Err(format!(
                    "Explicit table mapping {model} did not survive candidate selection"
                ));
            }
        }
    }
    Ok(report
        .candidates
        .iter()
        .filter_map(|candidate| candidate.table_audit.as_ref())
        .collect())
}
