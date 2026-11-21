//! Model source writing uses the auditor's shared project layout resolver.

use std::io;

/// Write a model to the configured or conventional location; e.g. `write("user", source)`.
/// Preserves the existing generator's overwrite behavior; does not run any database operation.
pub(super) fn write(name: &str, contents: &str) -> Result<(), io::Error> {
    let root = std::env::current_dir()?;
    let path = crate::generators::model_paths::model_file(&root, name).map_err(io::Error::other)?;
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("Invalid model source path"))?;
    std::fs::create_dir_all(parent)?;
    std::fs::write(path, contents)
}
