//! Prove a non-ancestor helper cannot introduce names into the type namespace.
use syn::{Item, Visibility};

/// Private imports and impls do not reexport module-level types to sibling rows.
pub(super) fn values_only(items: &[Item]) -> bool {
    items.iter().all(|item| match item {
        Item::Fn(item) => plain(&item.attrs),
        Item::Const(item) => plain(&item.attrs),
        Item::Static(item) => plain(&item.attrs),
        Item::Impl(item) => plain(&item.attrs),
        Item::Use(item) => matches!(item.vis, Visibility::Inherited) && plain(&item.attrs),
        _ => false,
    })
}

/// Documentation and lint levels cannot add names; other attributes stay unknown.
pub(in crate::model_audit::schema::rust_types::imports) fn plain(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().all(|attr| {
        ["doc", "allow", "warn", "deny", "forbid", "expect"]
            .iter()
            .any(|name| attr.path().is_ident(name))
    })
}
