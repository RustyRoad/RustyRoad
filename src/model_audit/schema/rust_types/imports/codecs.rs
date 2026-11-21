//! Explicit SQLx contracts and scope-aware lookup, not compiler or codec execution.
mod derived;
mod index;
mod lookup;
mod manual;
mod paths;

pub(super) use index::collect;
pub(super) use lookup::lookup;

/// Conditions must not silently select one of several potential storage contracts.
pub(super) fn unconditional(attrs: &[syn::Attribute]) -> bool {
    !attrs
        .iter()
        .any(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr"))
}

/// Accept one unqualified SQLx name and quote it to preserve case and exclude SQL fragments.
pub(super) fn sql_name(value: &str) -> Option<String> {
    let mut chars = value.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '$')
    {
        return None;
    }
    Some(sqlparser::ast::Ident::with_quote('"', value).to_string())
}
