//! PostgreSQL storage inference, preserving explicit overrides and Rust imports.
use super::types::Column;
use crate::model_audit::types::Field;
use std::collections::BTreeMap;
mod attributes;
mod inference;
use inference::infer;
mod builtins;
mod imports;
mod normalization;
mod wrappers;
pub(super) use imports::Resolver;
pub(super) use normalization::normalized;

/// Infer standalone builtin contracts, e.g. `Option<i64>` -> nullable bigint.
#[cfg(test)]
pub(super) fn column(
    name: &str,
    field: &Field,
    mappings: &BTreeMap<String, String>,
) -> Result<Column, String> {
    infer(name, field, mappings, None)
}

/// Resolve source imports before inferring types such as `DateTime<Utc>`.
pub(super) fn model_column(
    name: &str,
    field: &Field,
    mappings: &BTreeMap<String, String>,
    resolver: &Resolver,
) -> Result<Column, String> {
    infer(name, field, mappings, Some(resolver))
}
