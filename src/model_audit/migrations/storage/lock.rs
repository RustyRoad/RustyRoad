//! Exclusive generation ownership shared with the locked history recheck.
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Proof that this invocation acquired the create-only generation lock.
pub(in crate::model_audit::migrations) struct Lock {
    path: PathBuf,
    file: Option<fs::File>,
}

impl Lock {
    /// Never adopt or remove a lock held by another invocation.
    pub(super) fn acquire(directory: &Path) -> Result<Self, String> {
        let path = directory.join(".model-audit.lock");
        let file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| {
                format!(
                    "Migration generation lock {}: {error}; review a stale lock before retrying",
                    path.display()
                )
            })?;
        Ok(Self {
            path,
            file: Some(file),
        })
    }
}

/// Close the handle before unlinking, including on Windows; never remove evidence.
impl Drop for Lock {
    fn drop(&mut self) {
        drop(self.file.take());
        let _ = fs::remove_file(&self.path);
    }
}
