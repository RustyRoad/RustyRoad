//! Filesystem traversal without following symlinks or dependencies.
use std::path::{Path, PathBuf};
/// Skip dependency/build/artifact trees and record IO failures rather than hiding them.
pub(super) fn walk(
    path: &Path,
    files: &mut std::collections::BTreeSet<PathBuf>,
    errors: &mut Vec<String>,
) {
    let result = (|| -> std::io::Result<()> {
        let metadata = std::fs::symlink_metadata(path)?;
        if metadata.file_type().is_symlink() {
            return Ok(());
        }
        if metadata.is_dir() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name.starts_with('.') || ["target", "node_modules", "vendor"].contains(&name) {
                return Ok(());
            }
            for entry in std::fs::read_dir(path)? {
                walk(&entry?.path(), files, errors);
            }
        } else if path.extension().is_some_and(|e| e == "rs") {
            files.insert(path.canonicalize()?);
        }
        Ok(())
    })();
    if let Err(error) = result {
        errors.push(format!("{}: {error}", path.display()));
    }
}
