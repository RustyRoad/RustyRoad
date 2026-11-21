//! Resolve inline helper targets only through explicitly declared descendants.
use super::super::namespace;
use super::exports::plain;
use syn::{Item, Path};

/// Never interpret `self::*`, `super::*` or a crate-root glob as empty helpers.
pub(super) fn inline(items: &[Item], path: &Path) -> Option<Vec<Item>> {
    let mut items = items;
    let mut parts = path.segments.iter().peekable();
    if parts.peek()?.ident == "self" {
        parts.next();
    }
    let parts: Vec<_> = parts.collect();
    if parts.is_empty() {
        return None;
    }
    for part in parts {
        let name = part.ident.to_string();
        if !namespace::unique(items, &name) {
            return None;
        }
        let module = items.iter().find_map(|item| match item {
            Item::Mod(module) if module.ident == name => Some(module),
            _ => None,
        })?;
        if !plain(&module.attrs) {
            return None;
        }
        items = &module.content.as_ref()?.1;
    }
    Some(items.to_vec())
}
