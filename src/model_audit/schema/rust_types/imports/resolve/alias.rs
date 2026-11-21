//! Expand imported paths without losing generics or cycling on crate identity.
use quote::ToTokens;
use syn::{Path, PathArguments};

/// Expand `Key<T>` through its source alias, preserving the type arguments.
/// Identity imports such as `use sqlx::{self, ...}` retain external identity.
pub(super) fn expand(alias: &Path, original: &Path) -> (Path, bool) {
    let mut expanded = alias.clone();
    if let Some(first) = original.segments.first() {
        if !matches!(first.arguments, PathArguments::None) {
            if let Some(last) = expanded.segments.last_mut() {
                last.arguments = first.arguments.clone();
            }
        }
    }
    expanded
        .segments
        .extend(original.segments.iter().skip(1).cloned());
    let identity = expanded.to_token_stream().to_string() == original.to_token_stream().to_string();
    if identity {
        // An identity import is an external crate, not a recursive type alias.
        expanded.leading_colon = Some(Default::default());
    }
    (expanded, identity)
}
