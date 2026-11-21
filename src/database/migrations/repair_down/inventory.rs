//! Deterministic, nonrecursive migration discovery without following symlinks.

use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn directory(path: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "{}: expected a real directory, not a symlink",
            path.display()
        ));
    }
    Ok(())
}

pub(super) fn exists(path: &Path) -> Result<bool, String> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(format!("{}: {e}", path.display())),
    }
}

pub(super) fn directories(path: &Path) -> Result<Vec<PathBuf>, String> {
    directory(path)?;
    let path = fs::canonicalize(path).map_err(|e| e.to_string())?;
    if exists(&path.join("up.sql"))? {
        return Ok(vec![path]);
    }
    let mut found = Vec::new();
    for child in fs::read_dir(&path).map_err(|e| e.to_string())? {
        let child = child.map_err(|e| e.to_string())?;
        let kind = child.file_type().map_err(|e| e.to_string())?;
        if kind.is_symlink() {
            return Err(format!(
                "{}: symlinked entry requires manual review",
                child.path().display()
            ));
        }
        if !kind.is_dir() {
            continue;
        }
        if child.file_name().to_str().is_none() {
            return Err("Non-UTF-8 migration directory requires manual review".into());
        }
        found.push(child.path());
    }
    found.sort();
    if found.is_empty() {
        return Err(format!(
            "{}: no migration directories found",
            path.display()
        ));
    }
    Ok(found)
}
