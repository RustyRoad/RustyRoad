//! Offline, preview-first repair of missing undo files; never runs migration SQL.

mod inventory;
mod plan;
mod publication;
mod reviewed;
mod sql;
mod storage;
#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Status {
    Existing,
    Ready,
    Written,
    Blocked,
}

#[derive(Debug)]
pub struct Entry {
    pub directory: PathBuf,
    pub status: Status,
    pub detail: String,
    pub sql: Option<String>,
}

struct Pending {
    entry: usize,
    source: Vec<u8>,
}

/// Plans every missing undo before writing. A planning blocker prevents all writes.
/// Publication is no-clobber per file, not an atomic multi-directory transaction.
/// See `MIGRATION_UNDO_REPAIR.md` for CLI examples and reviewed undo requirements.
pub fn repair(path: &Path, reviewed: Option<&Path>, write: bool) -> Result<Vec<Entry>, String> {
    if let Some(root) = reviewed {
        inventory::directory(root)?;
    }
    let mut entries = Vec::new();
    let mut pending = Vec::new();
    for directory in inventory::directories(path)? {
        let (entry, source) = plan::entry(directory, reviewed);
        if let Some(source) = source {
            pending.push(Pending {
                entry: entries.len(),
                source,
            });
        }
        entries.push(entry);
    }
    if !write || entries.iter().any(|e| matches!(e.status, Status::Blocked)) {
        return Ok(entries);
    }
    publication::write(&mut entries, &pending);
    Ok(entries)
}
