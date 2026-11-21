//! Recognize intact owned migrations without overwriting edited or partial artifacts.
use crate::model_audit::schema::types::Proposal;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Identify an intact owned proposal for history conflict checks.
pub(in crate::model_audit::migrations) fn known(
    root: &Path,
    proposal: &Proposal,
) -> Result<Option<PathBuf>, String> {
    existing(&super::directory(root)?, proposal)
}

/// Same SQL means the same proposed operation, even if the model source was moved.
pub(super) fn fingerprint(proposal: &Proposal) -> String {
    Sha256::digest(format!("{}\0{}", proposal.up, proposal.down).as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Find an intact earlier generated proposal; edited/partial artifacts block retries.
pub(super) fn existing(dir: &Path, proposal: &Proposal) -> Result<Option<PathBuf>, String> {
    if !dir.exists() {
        return Ok(None);
    }
    let fingerprint = fingerprint(proposal);
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_symlink() {
            return Err("Symlink in canonical migration tree; review before generation".into());
        }
        let path = entry.path();
        let metadata = path.join("model-audit.json");
        if !metadata.exists() {
            continue;
        }
        if fs::symlink_metadata(&metadata)
            .map_err(|e| e.to_string())?
            .file_type()
            .is_symlink()
        {
            return Err(format!(
                "Symlinked audit metadata {} requires review",
                metadata.display()
            ));
        }
        let value: serde_json::Value =
            serde_json::from_slice(&fs::read(&metadata).map_err(|e| e.to_string())?)
                .map_err(|e| format!("Invalid {}: {e}", metadata.display()))?;
        if value["fingerprint"].as_str() != Some(&fingerprint) {
            continue;
        }
        for (file, content) in [("up.sql", &proposal.up), ("down.sql", &proposal.down)] {
            let file = path.join(file);
            if fs::symlink_metadata(&file).is_ok_and(|m| m.file_type().is_symlink())
                || !fs::read_to_string(&file).is_ok_and(|text| text == *content)
            {
                return Err(format!("Existing audit migration {} is edited or incomplete; it will not be overwritten", path.display()));
            }
        }
        return Ok(Some(path));
    }
    Ok(None)
}
