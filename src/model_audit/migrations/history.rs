//! Block duplicate DDL already represented in the canonical migration history.

mod overlap;
mod raw;
mod scan;
mod sql;

use super::storage;
use crate::model_audit::schema::types::Proposal;
use std::path::Path;

/// Recheck without ownership, e.g. refuse a concurrent generation lock.
#[cfg(test)]
pub(super) fn check(root: &Path, schema: &str, proposal: &Proposal) -> Result<(), String> {
    reject_conflict(inspect(root, schema, proposal)?)
}

/// Recheck under the writer's exclusive lock without rejecting its own ownership.
pub(super) fn check_locked(
    root: &Path,
    schema: &str,
    proposal: &Proposal,
    _lock: &storage::Lock,
) -> Result<(), String> {
    let directory = storage::directory(root)?;
    reject_conflict(scan::inspect(root, &directory, schema, proposal)?)
}

/// Separate table-local conflicts from unsafe history or another writer's lock.
pub(super) fn inspect(
    root: &Path,
    schema: &str,
    proposal: &Proposal,
) -> Result<Option<String>, String> {
    let directory = storage::directory(root)?;
    if directory.join(".model-audit.lock").exists() {
        return Err("Migration generation lock exists; review a concurrent/stale run".into());
    }
    scan::inspect(root, &directory, schema, proposal)
}

fn reject_conflict(conflict: Option<String>) -> Result<(), String> {
    match conflict {
        Some(conflict) => Err(conflict),
        None => Ok(()),
    }
}
