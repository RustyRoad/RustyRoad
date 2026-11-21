//! Create-only canonical files with deterministic operation fingerprints for retries.

mod lock;
mod owned;
mod writer;

pub(super) use self::{lock::Lock, owned::known, writer::write};
use crate::model_audit::schema::types::Proposal;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Refuse symlinked canonical paths, e.g. a config/database symlink outside the project.
pub(super) fn directory(root: &Path) -> Result<PathBuf, String> {
    let mut path = root.to_owned();
    for part in ["config", "database", "migrations"] {
        path.push(part);
        match fs::symlink_metadata(&path) {
            Ok(meta) if !meta.is_dir() || meta.file_type().is_symlink() => {
                return Err(format!("Unsafe migration directory {}", path.display()))
            }
            Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
                return Err(error.to_string())
            }
            _ => (),
        }
    }
    Ok(path)
}

/// Check directory and prior-proposal integrity without creating anything.
pub(super) fn preflight(root: &Path, proposal: &Proposal) -> Result<(), String> {
    owned::existing(&directory(root)?, proposal).map(|_| ())
}
