//! Literal whole-row SQL is ownership evidence, not a naming convention.
mod calls;
mod functions;
mod sql;
use crate::model_audit::types::Model;
use super::super::rust_types::Resolver;
use std::collections::BTreeSet;
use syn::{Item, Type};

/// Return distinct proven relations; e.g. two queries for `rows` yield one name.
pub(super) fn tables(model: &Model, schema: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let Ok(source) = std::fs::read_to_string(&model.path) else { return found };
    let Ok(file) = syn::parse_file(&source) else { return found };
    let mut items = file.items.as_slice();
    let mut modules: Vec<_> = model.id.split("::").skip(1).collect();
    modules.pop();
    for name in modules {
        let Some(module) = items.iter().find_map(|item| match item {
            Item::Mod(module) if module.ident == name => Some(module),
            _ => None,
        }) else { return found };
        if !plain(&module.attrs) { return found; }
        let Some((_, children)) = &module.content else { return found };
        items = children;
    }
    let resolver = Resolver::for_model(model);
    let mut collect = |block: &syn::Block, own: bool| {
        functions::collect(block, own, model, schema, &resolver, &mut found);
    };
    for item in items {
        match item {
            Item::Fn(function) if plain(&function.attrs)
                && function.sig.generics.params.is_empty() => collect(&function.block, false),
            Item::Impl(item) if plain(&item.attrs) && item.trait_.is_none()
                && item.generics.params.is_empty() => {
                let own = matches!(item.self_ty.as_ref(), Type::Path(ty)
                    if ty.qself.is_none() && ty.path.is_ident(&model.name));
                for member in &item.items {
                    if let syn::ImplItem::Fn(function) = member {
                        if plain(&function.attrs) && function.sig.generics.params.is_empty() {
                            collect(&function.block, own);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    found
}

/// Only documentation is inert; e.g. `#[cfg(...)]` cannot prove ownership.
fn plain(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().all(|attribute| attribute.path().is_ident("doc"))
}
//! Conservative same-module SQLx row ownership, never a projection/name guess.
mod calls;
mod scope;
mod sql;
mod walk;
use crate::model_audit::types::Model;
use std::collections::BTreeSet;

/// Read typed whole-row queries, e.g. `query_as::<_, Self>("SELECT * FROM rows")`.
pub(super) fn tables(model: &Model) -> BTreeSet<String> {
    let Some(items) = scope::items(model) else {
        return BTreeSet::new();
    };
    walk::tables(model, &items)
}