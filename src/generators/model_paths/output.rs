use super::*;

/// Generator destination; an explicit CLI --out remains authoritative in its caller.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(crate) fn rust_output(root: &Path) -> Result<PathBuf, String> {
    let layout = resolve(root)?;
    if let Some(file) = layout.generated {
        return Ok(file.parent().ok_or("Invalid generated model path")?.into());
    }
    if root.join("crates/db/src").is_dir() {
        Ok(root.join("crates/db/src"))
    } else {
        Ok(root.join("src/db"))
    }
}
