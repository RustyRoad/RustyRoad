//! Explicit SQLx named derives and transparent wrappers of known builtin codecs.
use super::super::Resolver;
mod options;
mod transparent;
use super::paths;
use syn::{punctuated::Punctuated, Item, Path, Token};

/// Recognize explicit named codecs or transparent single-field builtin wrappers.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn contract(item: &Item, resolver: &Resolver) -> Option<String> {
    let attrs = match item {
        Item::Struct(value) => &value.attrs,
        Item::Enum(value) => &value.attrs,
        _ => return None,
    };
    if attrs.iter().any(|attr| attr.path().is_ident("repr")) {
        return None;
    }
    let mut derives = 0;
    for attr in attrs {
        if attr.path().is_ident("derive") {
            let paths = attr
                .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                .ok()?;
            derives += paths
                .iter()
                .filter(|path| paths::is(path, "sqlx::Type", resolver))
                .count();
        }
    }
    if derives != 1 {
        return None;
    }
    transparent::contract(item, resolver).or_else(|| options::name(attrs))
}
