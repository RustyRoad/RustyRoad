//! Follow declared same-crate modules, never basename matches or Cargo execution.
mod globs;
mod lookup;
mod node;
mod origin;
mod root;
use crate::model_audit::types::Model;
use node::Node;
use std::path::PathBuf;
use syn::Path;

/// Verified crate root and the module owning a model's source file.
pub(super) struct Context {
    root: PathBuf,
    modules: Vec<String>,
    inline: Vec<String>,
}

impl Context {
    /// Inline-only evidence is safe only outside any Cargo manifest's ownership.
    /// For example, a standalone `models.rs` fixture needs no crate root.
    pub(super) fn standalone(model: &Model) -> bool {
        !model
            .path
            .ancestors()
            .skip(1)
            .any(|directory| directory.join("Cargo.toml").is_file())
    }

    /// Resolve `crate::codec::State` or a source-declared reexport from this file.
    pub(super) fn storage(&self, path: &Path, scope: usize) -> Option<String> {
        let mut modules = self.modules.clone();
        modules.extend_from_slice(self.inline.get(..scope)?);
        lookup::storage(self, &modules, path, 0)
    }

    /// Load only modules required by a reference, not every file in the crate.
    fn node(&self, modules: &[String]) -> Option<Node> {
        let mut node = Node::read(&self.root, self.root.parent()?.to_path_buf())?;
        for name in modules {
            node = node.child(name)?;
        }
        Some(node)
    }
}
