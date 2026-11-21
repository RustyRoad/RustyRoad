//! Persist one proposal under exclusive ownership after rechecking migration history.
use super::{
    owned::{existing, fingerprint},
    Lock,
};
use crate::model_audit::schema::types::{MigrationFile, Proposal};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Return an intact identical migration or exclusively create UP/DOWN/manifest files.
pub(in crate::model_audit::migrations) fn write(
    root: &Path,
    schema: &str,
    proposal: &Proposal,
) -> Result<MigrationFile, String> {
    let dir = super::directory(root)?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let guard = Lock::acquire(&dir)?;
    super::super::history::check_locked(root, schema, proposal, &guard)?;
    if let Some(directory) = existing(&dir, proposal)? {
        return Ok(MigrationFile {
            model_id: proposal.model_id.clone(),
            directory,
            state: "already_generated".into(),
        });
    }
    let id = fingerprint(proposal);
    let name = format!("audit_{}_{}", proposal.table, &id[..12]);
    let directory = PathBuf::from(crate::database::migrations::folder::folder_for(
        dir.to_str().ok_or("Migration path must be UTF-8")?,
        &name,
    ));
    fs::create_dir(&directory)
        .map_err(|e| format!("Cannot exclusively create {}: {e}", directory.display()))?;
    let metadata = serde_json::to_string_pretty(&serde_json::json!({
        "generator":"rustyroad model audit", "version":1, "model_id":proposal.model_id,
        "fingerprint":id, "warnings":proposal.warnings
    }))
    .map_err(|e| e.to_string())?;
    // Manifest first: a failed partial write is detected, retained and blocked on retry.
    for (file, content) in [
        ("model-audit.json", metadata.as_str()),
        ("up.sql", &proposal.up),
        ("down.sql", &proposal.down),
    ] {
        let path = directory.join(file);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        file.write_all(content.as_bytes())
            .and_then(|_| file.sync_all())
            .map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(MigrationFile {
        model_id: proposal.model_id.clone(),
        directory,
        state: "generated".into(),
    })
}
