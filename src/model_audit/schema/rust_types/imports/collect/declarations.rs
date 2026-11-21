//! Local declarations shadow imported names; only closed path aliases expand.
use super::super::{binding::Binding, codecs::unconditional};
use syn::{Item, ItemType, Type};

/// `struct Uuid` blocks guessing; `type Keys = Vec<uuid::Uuid>` may expand.
pub(super) fn binding(item: &Item) -> Option<(String, Option<Binding>)> {
    let name = match item {
        Item::Type(alias) => return Some((alias.ident.to_string(), closed_alias(alias))),
        Item::Struct(item) => &item.ident,
        Item::Enum(item) => &item.ident,
        Item::Union(item) => &item.ident,
        Item::Mod(item) => &item.ident,
        Item::Trait(item) => &item.ident,
        Item::TraitAlias(item) => &item.ident,
        Item::ExternCrate(item) => item
            .rename
            .as_ref()
            .map(|(_, alias)| alias)
            .unwrap_or(&item.ident),
        _ => return None,
    };
    Some((name.to_string(), None))
}

/// Generic, conditional, or non-path aliases cannot establish codec identity.
fn closed_alias(alias: &ItemType) -> Option<Binding> {
    if !alias.generics.params.is_empty() || !unconditional(&alias.attrs) {
        return None;
    }
    let Type::Path(path) = &*alias.ty else {
        return None;
    };
    path.qself
        .is_none()
        .then(|| Binding::Alias(path.path.clone()))
}
