//! Rust's ordinary module file layout, requiring one unconditional declaration.
mod child;
mod path;
use super::super::{codecs::unconditional, glob::exports::plain};
use std::path::{Path, PathBuf};
use syn::Item;

/// Parsed module with distinct ordinary-child and path-attribute directories.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --offline --report-only
/// ```
pub(super) struct Node {
    pub(super) items: Vec<Item>,
    pub(super) file: PathBuf,
    pub(super) plain: bool,
    directory: PathBuf,
    path_directory: PathBuf,
    boundary: PathBuf,
}

/// Read declared modules while retaining their source and package boundary.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --offline --report-only
/// ```
impl Node {
    /// Read a module and preserve the physical directory for path attributes.
    /// # Examples
    /// ```sh
    /// rustyroad model audit --source src/models --offline --report-only
    /// ```
    pub(super) fn read(file: &Path, directory: PathBuf) -> Option<Self> {
        let file = file.canonicalize().ok()?;
        let path_directory = file.parent()?.to_path_buf();
        let boundary = file
            .ancestors()
            .skip(1)
            .find(|directory| directory.join("Cargo.toml").is_file())
            .unwrap_or(&path_directory)
            .to_path_buf();
        let source = std::fs::read_to_string(&file).ok()?;
        let parsed = syn::parse_file(&source).ok()?;
        if !unconditional(&parsed.attrs) {
            return None;
        }
        Some(Self {
            plain: plain(&parsed.attrs),
            items: parsed.items,
            file,
            path_directory,
            boundary,
            directory,
        })
    }
}
