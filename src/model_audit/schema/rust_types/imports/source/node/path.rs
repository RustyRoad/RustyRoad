//! Resolve one literal path attribute without escaping the owning package.
use super::Node;
use std::path::PathBuf;
use syn::{Expr, Lit, Meta};

/// Preserve Rust's distinct file and inline-module path rules.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --offline --report-only
/// ```
impl Node {
    /// Return the declared source file and its ordinary child-module directory.
    /// Missing, duplicate, absolute and nonliteral paths remain unresolved.
    /// # Examples
    /// ```sh
    /// rustyroad model audit --source src/models --offline --report-only
    /// ```
    pub(super) fn module_file(&self, module: &syn::ItemMod) -> Option<(PathBuf, PathBuf)> {
        let attributes: Vec<_> = module
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident("path"))
            .collect();
        let file = match attributes.as_slice() {
            [] => {
                let name = module.ident.to_string();
                let candidates = [
                    self.directory.join(format!("{name}.rs")),
                    self.directory.join(&name).join("mod.rs"),
                ];
                let files: Vec<_> = candidates.iter().filter(|file| file.is_file()).collect();
                if files.len() != 1 {
                    return None;
                }
                files[0].to_path_buf()
            }
            [attribute] => {
                let Meta::NameValue(value) = &attribute.meta else {
                    return None;
                };
                let Expr::Lit(value) = &value.value else {
                    return None;
                };
                let Lit::Str(value) = &value.lit else {
                    return None;
                };
                let path = PathBuf::from(value.value());
                if path.is_absolute() {
                    return None;
                }
                self.path_directory.join(path)
            }
            _ => return None,
        }
        .canonicalize()
        .ok()?;
        if !file.starts_with(&self.boundary) || !file.is_file() {
            return None;
        }
        let parent = file.parent()?;
        let directory = if file.file_name()? == "mod.rs" {
            parent.to_path_buf()
        } else {
            parent.join(file.file_stem()?)
        };
        Some((file, directory))
    }
}
