//! Distinguish one source binding from an ambiguous type/module namespace.
use syn::Item;

/// Reject imports colliding with declarations before following either contract.
pub(super) fn unique(items: &[Item], name: &str) -> bool {
    items
        .iter()
        .filter(|item| match item {
            Item::Struct(item) => item.ident == name,
            Item::Enum(item) => item.ident == name,
            Item::Type(item) => item.ident == name,
            Item::Mod(item) => item.ident == name,
            Item::Union(item) => item.ident == name,
            Item::Trait(item) => item.ident == name,
            Item::TraitAlias(item) => item.ident == name,
            Item::ExternCrate(item) => {
                item.rename
                    .as_ref()
                    .map(|(_, alias)| alias)
                    .unwrap_or(&item.ident)
                    == name
            }
            Item::Use(item) => {
                super::collect::bindings(&[Item::Use(item.clone())]).contains_key(name)
            }
            _ => false,
        })
        .count()
        == 1
}
