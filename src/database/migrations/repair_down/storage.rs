//! Read regular files and publish staged undo bytes without replacing any entry.

use super::inventory;
use std::fs;
use std::io::Write;
use std::path::Path;

pub(super) fn read(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!(
            "{}: expected a regular file, not a symlink",
            path.display()
        ));
    }
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

pub(super) fn preflight(directory: &Path, source: &[u8]) -> Result<(), String> {
    inventory::directory(directory)?;
    let destination = directory.join("down.sql");
    if inventory::exists(&destination)? {
        return Err(format!(
            "{} appeared since preview; left untouched",
            destination.display()
        ));
    }
    if read(&directory.join("up.sql"))? != source {
        return Err(format!(
            "{}: up.sql changed since planning; undo not published",
            directory.display()
        ));
    }
    Ok(())
}

pub(super) fn publish(directory: &Path, source: &[u8], sql: &str) -> Result<(), String> {
    if sql.trim().is_empty() || sql.contains('\0') {
        return Err("Undo must be nonempty SQL without NUL bytes; nothing published".into());
    }
    preflight(directory, source)?;
    let mut staged = tempfile::NamedTempFile::new_in(directory).map_err(|e| e.to_string())?;
    staged
        .write_all(sql.as_bytes())
        .map_err(|e| e.to_string())?;
    staged.as_file().sync_all().map_err(|e| e.to_string())?;
    preflight(directory, source)?;
    staged
        .persist_noclobber(directory.join("down.sql"))
        .map_err(|e| {
            format!(
                "{}: no-clobber publication failed: {}",
                directory.display(),
                e.error
            )
        })?;
    Ok(())
}
