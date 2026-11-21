//! Inspect canonical UP files, retaining table-local overlap diagnostics.
use super::{overlap, sql, storage};
use crate::model_audit::schema::types::Proposal;
use std::{fs, path::Path};

pub(super) fn inspect(
    root: &Path,
    directory: &Path,
    schema: &str,
    proposal: &Proposal,
) -> Result<Option<String>, String> {
    if !directory.exists() {
        return Ok(None);
    }
    let owned = storage::known(root, proposal)?;
    let mut conflict = None;
    for entry in fs::read_dir(directory).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        if kind.is_symlink() {
            return Err("Symlink in migration history requires review".into());
        }
        if !kind.is_dir() || owned.as_ref() == Some(&entry.path()) {
            continue;
        }
        let path = entry.path().join("up.sql");
        let kind = fs::symlink_metadata(&path)
            .map_err(|e| format!("Cannot inspect history {}: {e}", path.display()))?;
        if !kind.is_file() || kind.file_type().is_symlink() {
            return Err(format!("Unsafe migration history {}", path.display()));
        }
        let text = fs::read_to_string(&path)
            .map_err(|e| format!("Cannot read history {}: {e}", path.display()))?;
        for operation in sql::operations(&text)
            .map_err(|e| format!("Cannot inspect history {}: {e}", path.display()))?
        {
            if overlap::conflicts(&operation, schema, proposal)
                .map_err(|e| format!("Cannot inspect history {}: {e}", path.display()))?
            {
                conflict.get_or_insert_with(|| format!(
                    "Migration history {} already contains overlapping DDL for {}. Review pending/applied migrations instead of generating a duplicate",
                    path.display(), proposal.table
                ));
            }
        }
    }
    Ok(conflict)
}
