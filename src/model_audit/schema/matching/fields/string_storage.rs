//! Keep catalog character storage when Rust's String supplies no storage override.
use super::source;
use crate::model_audit::{
    schema::{rust_types, type_mappings, types::Column},
    types::Field,
};
use std::collections::BTreeMap;

/// Preserve only inferred text storage; nullability and explicit codecs stay independent.
/// String codecs support both varying and fixed-width character storage.
/// # Examples
/// ```sh
/// rustyroad model audit --report-only
/// ```
pub(super) fn preserve(
    field: &Field,
    mappings: &BTreeMap<String, String>,
    resolver: &rust_types::Resolver,
    existing: &Column,
    desired: &mut Column,
) {
    if desired.sql_type != "text" || !character_storage(&existing.sql_type) {
        return;
    }
    let Ok(original) = syn::parse_str::<syn::Type>(&field.ty) else {
        return;
    };
    let resolved = resolver.resolve(&original);
    let original = source::inner(&original);
    let resolved = source::inner(&resolved);
    if !source::is_string(resolved)
        || resolver.storage(original).is_some()
        || mappings.contains_key(&type_mappings::spelling(original))
        || mappings.contains_key(&type_mappings::spelling(resolved))
    {
        return;
    }
    desired.sql_type = existing.sql_type.clone();
}

/// Recognize character storage with an optional positive length and no other SQL.
/// Domain names and arrays are excluded; fixed-width padding semantics are preserved.
/// # Examples
/// ```sh
/// rustyroad model audit --report-only
/// ```
fn character_storage(sql: &str) -> bool {
    let value = sql.trim().to_ascii_lowercase();
    let (base, length) = match value.split_once('(') {
        Some((base, suffix)) => {
            let Some(length) = suffix.strip_suffix(')') else {
                return false;
            };
            let Ok(length) = length.trim().parse::<u32>() else {
                return false;
            };
            (base.trim_end(), Some(length))
        }
        None => (value.as_str(), None),
    };
    matches!(
        base,
        "character varying" | "varchar" | "character" | "char" | "bpchar"
    ) && length.map_or(true, |length| length > 0)
}
