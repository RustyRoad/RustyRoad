//! Statically declared PostgreSQL names, without evaluating arbitrary codec code.
mod implementation;
mod value;
use super::super::Resolver;
use super::unconditional;
use syn::{Item, Type};

/// Read `impl sqlx::Type<sqlx::Postgres> for Local` with one literal type-info body.
pub(super) fn contracts(
    items: &[Item],
    name: &syn::Ident,
    resolver: &Resolver,
) -> Vec<Option<String>> {
    items
        .iter()
        .filter_map(|item| {
            let Item::Impl(item) = item else {
                return None;
            };
            let Type::Path(target) = item.self_ty.as_ref() else {
                return None;
            };
            if target.qself.is_some() || !target.path.is_ident(name) {
                return None;
            }
            if !implementation::matches(item, resolver) {
                return None;
            }
            if !item.generics.params.is_empty()
                || item.generics.where_clause.is_some()
                || !unconditional(&item.attrs)
            {
                return Some(None);
            }
            let methods: Vec<_> = item
                .items
                .iter()
                .filter_map(|member| match member {
                    syn::ImplItem::Fn(method) if method.sig.ident == "type_info" => Some(method),
                    _ => None,
                })
                .collect();
            let contract = if methods.len() == 1 {
                value::name(methods[0], resolver)
            } else {
                None
            };
            Some(contract)
        })
        .collect()
}
