//! Reject bindings that can redirect the SQLx path or the declared row type.
mod imports;
use syn::Item;

/// Only known non-transforming attributes are evidence, e.g. documentation.
pub(super) fn unconditional(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().all(|attribute| {
        [
            "doc", "derive", "allow", "warn", "deny", "forbid", "serde", "sqlx",
        ]
        .iter()
        .any(|name| attribute.path().is_ident(name))
    })
}

/// Determine whether an item could shadow either name, e.g. `mod sqlx {}`.
pub(super) fn safe_item(item: &Item, model: &str, declaration: bool) -> bool {
    let name = match item {
        Item::Use(item) => return unconditional(&item.attrs) && imports::safe(&item.tree, model),
        Item::Struct(item) if declaration && item.ident == model => {
            return unconditional(&item.attrs);
        }
        Item::Struct(item) => Some(&item.ident),
        Item::Enum(item) => Some(&item.ident),
        Item::Type(item) => Some(&item.ident),
        Item::Mod(item) => Some(&item.ident),
        Item::Trait(item) => Some(&item.ident),
        Item::Union(item) => Some(&item.ident),
        Item::ExternCrate(item) => Some(
            item.rename
                .as_ref()
                .map(|(_, name)| name)
                .unwrap_or(&item.ident),
        ),
        Item::Fn(item) => Some(&item.sig.ident),
        Item::Const(item) => Some(&item.ident),
        Item::Static(item) => Some(&item.ident),
        Item::Macro(_) => return false,
        _ => None,
    };
    name.is_none_or(|name| name != "sqlx" && name != model)
}

/// Scope-local paths must refer to SQLx and the unique declaration, not imports.
pub(super) fn unshadowed(items: &[Item], model: &str) -> bool {
    items.iter().all(|item| safe_item(item, model, true))
}

/// A function-local binding may hide `sqlx`, even before its textual declaration.
pub(super) fn safe_pattern(pattern: &syn::Pat, model: &str) -> bool {
    if let syn::Pat::Ident(binding) = pattern {
        binding.ident != "sqlx" && binding.ident != model
    } else {
        true
    }
}
