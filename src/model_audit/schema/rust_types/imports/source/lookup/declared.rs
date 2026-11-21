//! Resolve declarations and explicit imports within an exact source scope.
use super::super::super::{codecs, collect, namespace, Resolver};
use super::super::Context;
use syn::Path;

/// For example, `crate::types::Id` must follow a unique declared type or import.
pub(super) fn storage(
    context: &Context,
    modules: &[String],
    path: &Path,
    first: &str,
    remaining: &Path,
    depth: usize,
) -> Option<String> {
    let node = context.node(modules)?;
    let bindings = collect::bindings(&node.items);
    let mut scopes = Vec::new();
    for length in 0..=modules.len() {
        scopes.push(collect::bindings(&context.node(&modules[..length])?.items));
    }
    let resolver = Resolver {
        scopes,
        ..Resolver::default()
    };
    let contracts = codecs::collect(&node.items, &resolver);
    if !namespace::unique(&node.items, &first) {
        return None;
    }
    if let Some(contract) = contracts.get(first) {
        return remaining
            .segments
            .is_empty()
            .then(|| contract.clone())
            .flatten();
    }
    if let Some(Some(binding)) = bindings.get(first) {
        let (expanded, _) = binding.expand(path)?;
        return super::storage(context, modules, &expanded, depth + 1);
    }
    if remaining.segments.is_empty() {
        return None;
    }
    node.child(first)?;
    let mut child = modules.to_vec();
    child.push(first.to_owned());
    super::storage(context, &child, &remaining, depth + 1)
}
