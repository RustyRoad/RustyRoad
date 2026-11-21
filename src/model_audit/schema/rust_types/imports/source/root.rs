//! Locate a declared library target without invoking Cargo or following dependencies.
use std::path::{Path, PathBuf};

/// Accept `[lib].path` or Cargo's default `src/lib.rs` in the nearest package.
pub(super) fn for_file(file: &Path) -> Option<PathBuf> {
    let directory = file
        .ancestors()
        .skip(1)
        .find(|directory| directory.join("Cargo.toml").is_file())?;
    let contents = std::fs::read_to_string(directory.join("Cargo.toml")).ok()?;
    let manifest: toml::Value = toml::from_str(&contents).ok()?;
    manifest.get("package")?;
    let relative = match manifest.get("lib").and_then(|lib| lib.get("path")) {
        Some(value) => value.as_str()?,
        None => "src/lib.rs",
    };
    let path = Path::new(relative);
    if path
        .components()
        .any(|part| !matches!(part, std::path::Component::Normal(_)))
    {
        return None;
    }
    let root = directory.join(path);
    root.is_file().then_some(root)
}
