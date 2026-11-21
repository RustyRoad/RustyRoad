//! Locate the exact declaration scope; conditional modules provide no contract.
use crate::model_audit::types::Model;
use syn::Item;

/// Follow inline module IDs, e.g. `models.rs::nested::Row`, without name searches.
pub(super) fn items<'a>(mut items: &'a [Item], model: &Model) -> Option<&'a [Item]> {
    let mut names: Vec<_> = model.id.split("::").skip(1).collect();
    if names.pop()? != model.name {
        return None;
    }
    for name in names {
        if !super::namespace::unshadowed(items, &model.name) {
            return None;
        }
        let modules: Vec<_> = items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(module) if module.ident == name => Some(module),
                _ => None,
            })
            .collect();
        let [module] = modules.as_slice() else {
            return None;
        };
        if !super::namespace::unconditional(&module.attrs) {
            return None;
        }
        items = &module.content.as_ref()?.1;
    }
    let declarations: Vec<_> = items
        .iter()
        .filter_map(|item| match item {
            Item::Struct(item) if item.ident == model.name => Some(item),
            _ => None,
        })
        .collect();
    let [declaration] = declarations.as_slice() else {
        return None;
    };
    if !super::namespace::unconditional(&declaration.attrs) {
        return None;
    }
    Some(items)
}
