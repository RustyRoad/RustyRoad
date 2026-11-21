//! Construct lexical scopes from declaration source without Cargo or LSP.
use super::{codecs, glob, source, Resolver};
use crate::model_audit::types::Model;
use syn::Item;

impl Resolver {
    /// Read the declaration's source, e.g. `use uuid::Uuid`, without Cargo/LSP.
    pub(in crate::model_audit::schema) fn for_model(model: &Model) -> Self {
        let Ok(source) = std::fs::read_to_string(&model.path) else {
            return Self::default();
        };
        let Ok(file) = syn::parse_file(&source) else {
            return Self::default();
        };
        let mut items = file.items.as_slice();
        let source = source::Context::for_model(model);
        let mut plain = glob::exports::plain(&file.attrs)
            && (source.is_some() || source::Context::standalone(model));
        let mut scopes = vec![glob::bindings(items, source.as_ref(), 0, plain)];
        let root = Self {
            scopes: scopes.clone(),
            ..Self::default()
        };
        let mut codecs = vec![codecs::collect(items, &root)];
        let mut names: Vec<_> = model.id.split("::").skip(1).collect();
        names.pop();
        for name in names {
            let Some(module) = items.iter().find_map(|item| match item {
                Item::Mod(module) if module.ident == name => Some(module),
                _ => None,
            }) else {
                return Self::default();
            };
            let Some((_, children)) = &module.content else {
                return Self::default();
            };
            plain &= glob::exports::plain(&module.attrs);
            items = children;
            scopes.push(glob::bindings(items, source.as_ref(), scopes.len(), plain));
            let scope = Self {
                scopes: scopes.clone(),
                ..Self::default()
            };
            codecs.push(codecs::collect(items, &scope));
        }
        Self {
            scopes,
            codecs,
            source,
        }
    }
}
