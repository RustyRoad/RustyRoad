//! Import bindings are namespaces; closed type aliases are complete types.
use quote::ToTokens;
use syn::{Path, PathArguments};

/// `use Vec as List` accepts arguments; `type List = Vec<i32>` does not.
#[derive(Clone)]
pub(super) enum Binding {
    Import(Path),
    Alias(Path),
}

impl Binding {
    /// Return a binding's declared target, e.g. a local SQLx codec alias.
    pub(super) fn path(&self) -> &Path {
        match self {
            Self::Import(path) | Self::Alias(path) => path,
        }
    }

    /// Expand imports while preserving closed aliases and recursive uncertainty.
    pub(super) fn expand(&self, original: &Path) -> Option<(Path, bool)> {
        let first = original.segments.first()?;
        if matches!(self, Self::Alias(_)) {
            if original.segments.len() != 1 || !matches!(first.arguments, PathArguments::None) {
                return None;
            }
            return Some((self.path().clone(), false));
        }
        let mut expanded = self.path().clone();
        if !matches!(first.arguments, PathArguments::None) {
            expanded.segments.last_mut()?.arguments = first.arguments.clone();
        }
        expanded
            .segments
            .extend(original.segments.iter().skip(1).cloned());
        let identity =
            expanded.to_token_stream().to_string() == original.to_token_stream().to_string();
        if identity {
            // Only an identity import denotes an external crate, not an alias cycle.
            expanded.leading_colon = Some(Default::default());
        }
        Some((expanded, identity))
    }
}
