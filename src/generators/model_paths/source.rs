use super::*;

/// Generator source path. Reject path components in a model name instead of escaping its directory.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(crate) fn model_file(root: &Path, name: &str) -> Result<PathBuf, String> {
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err("Model name must contain only letters, digits and underscores".into());
    }
    // Generated inventory ambiguity must not prevent creating a handwritten model.
    let settings = settings(root)?;
    let directory = source_directory(root, settings.source);
    Ok(directory.join(format!("{name}.rs")))
}

/// One source-directory convention shared by the writer and auditor.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn source_directory(root: &Path, configured: Option<PathBuf>) -> PathBuf {
    configured.map(|p| root.join(p)).unwrap_or_else(|| {
        if root.join("crates/models/src").is_dir() {
            root.join("crates/models/src")
        } else {
            root.join("src/models")
        }
    })
}
