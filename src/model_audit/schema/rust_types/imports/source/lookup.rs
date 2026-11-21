//! Follow explicit imports and module declarations with a bounded reference walk.
mod declared;
use super::Context;
use syn::{Path, PathArguments};

/// Resolve a concrete source contract, rejecting cycles and unknown namespaces.
pub(super) fn storage(
    context: &Context,
    modules: &[String],
    path: &Path,
    depth: usize,
) -> Option<String> {
    if depth > 16
        || path.leading_colon.is_some()
        || path
            .segments
            .iter()
            .any(|part| !matches!(part.arguments, PathArguments::None))
    {
        return None;
    }
    let first = path.segments.first()?.ident.to_string();
    let mut remaining = path.clone();
    remaining.segments = remaining.segments.into_iter().skip(1).collect();
    match first.as_str() {
        "crate" => return storage(context, &[], &remaining, depth + 1),
        "self" => return storage(context, modules, &remaining, depth + 1),
        "super" => {
            let parent = modules.get(..modules.len().checked_sub(1)?)?;
            return storage(context, parent, &remaining, depth + 1);
        }
        _ => {}
    }
    declared::storage(context, modules, path, &first, &remaining, depth)
}
