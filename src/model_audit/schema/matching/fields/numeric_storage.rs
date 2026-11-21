//! Decimal codecs do not prescribe PostgreSQL precision or scale.
use super::source;
use crate::model_audit::{
    schema::{rust_types, type_mappings, types::Column},
    types::Field,
};
use std::collections::BTreeMap;

/// Preserve catalog numeric modifiers for inferred standard Decimal codecs.
/// Explicit mappings and custom SQLx storage contracts keep their precedence.
/// Nullability remains an independent comparison in the caller.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn preserve(
    field: &Field,
    mappings: &BTreeMap<String, String>,
    resolver: &rust_types::Resolver,
    existing: &Column,
    desired: &mut Column,
) {
    if desired.sql_type != "numeric" || !numeric_storage(&existing.sql_type) {
        return;
    }
    let Ok(original) = syn::parse_str::<syn::Type>(&field.ty) else {
        return;
    };
    let resolved = resolver.resolve(&original);
    let original = source::inner(&original);
    let resolved = source::inner(&resolved);
    let spelling = type_mappings::spelling(resolved).replace(' ', "");
    if !matches!(
        spelling.as_str(),
        "rust_decimal::Decimal"
            | "sqlx::types::Decimal"
            | "bigdecimal::BigDecimal"
            | "sqlx::types::BigDecimal"
    ) || resolver.storage(original).is_some()
        || mappings.contains_key(&type_mappings::spelling(original))
        || mappings.contains_key(&type_mappings::spelling(resolved))
    {
        return;
    }
    desired.sql_type = existing.sql_type.clone();
}

/// Recognize scalar numeric storage, including negative or greater-than-precision scales.
/// Arrays, domains and trailing SQL are excluded from inferred compatibility.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
fn numeric_storage(sql: &str) -> bool {
    let value = sql.trim().to_ascii_lowercase();
    let Some((base, suffix)) = value.split_once('(') else {
        return matches!(value.as_str(), "numeric" | "decimal");
    };
    if !matches!(base.trim_end(), "numeric" | "decimal") {
        return false;
    }
    let Some(modifiers) = suffix.strip_suffix(')') else {
        return false;
    };
    let mut parts = modifiers.split(',');
    let Some(precision) = parts
        .next()
        .and_then(|part| part.trim().parse::<u16>().ok())
    else {
        return false;
    };
    if !(1..=1000).contains(&precision) {
        return false;
    }
    let scale_valid = parts.next().map_or(true, |part| {
        part.trim()
            .parse::<i16>()
            .is_ok_and(|scale| (-1000..=1000).contains(&scale))
    });
    scale_valid && parts.next().is_none()
}
