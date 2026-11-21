//! Restrict transparent inference to one unconditional known builtin field.
use super::super::super::Resolver;
use crate::model_audit::schema::rust_types::builtins;
use syn::{Fields, Item};

/// Read a single-field SQLx transparent wrapper without guessing custom codecs.
/// The caller has already verified one SQLx Type derive and no representation.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn contract(item: &Item, resolver: &Resolver) -> Option<String> {
    let Item::Struct(item) = item else {
        return None;
    };
    let Fields::Unnamed(fields) = &item.fields else {
        return None;
    };
    if fields.unnamed.len() != 1 {
        return None;
    }
    let field = fields.unnamed.first()?;
    if !super::super::unconditional(&field.attrs)
        || field.attrs.iter().any(|attr| attr.path().is_ident("sqlx"))
    {
        return None;
    }
    let mut count = 0;
    for attr in item
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("sqlx"))
    {
        attr.parse_nested_meta(|meta| {
            if !meta.path.is_ident("transparent") || !meta.input.is_empty() {
                return Err(meta.error("transparent builtin codec requires one bare option"));
            }
            count += 1;
            Ok(())
        })
        .ok()?;
    }
    if count != 1 {
        return None;
    }
    builtins::sql(&resolver.resolve(&field.ty))
}
