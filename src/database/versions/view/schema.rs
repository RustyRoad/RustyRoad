//! Schema-level DDL for a version.

use super::super::naming::versioned_schema;
use super::super::quote::quote_ident;

/// Returns the statement creating the schema that serves `version`.
pub fn create_schema(schema: &str, version: &str) -> String {
    format!(
        "CREATE SCHEMA IF NOT EXISTS {}",
        quote_ident(&versioned_schema(schema, version))
    )
}

/// Returns the statement dropping the schema that served `version`.
///
/// Cascades so the version's views go with it.
pub fn drop_schema(schema: &str, version: &str) -> String {
    format!(
        "DROP SCHEMA IF EXISTS {} CASCADE",
        quote_ident(&versioned_schema(schema, version))
    )
}
