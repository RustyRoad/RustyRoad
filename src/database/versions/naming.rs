//! Naming rules for versioned schemas and physical columns.
//!
//! Mirrors pgroll: version schemas are `<schema>_<version>`, and in-progress
//! physical columns carry a reserved prefix so the old and new column can coexist
//! in the same table while both schema versions are being served.

/// Prefix marking a physical column added by an in-flight migration.
pub const TEMPORARY_PREFIX: &str = "_rustyroad_new_";

/// Prefix marking a physical column pending removal.
pub const DELETION_PREFIX: &str = "_rustyroad_del_";

/// Returns the schema name serving `version`.
pub fn versioned_schema(schema: &str, version: &str) -> String {
    format!("{schema}_{version}")
}

/// Returns the physical column name used while a migration is in flight.
pub fn temporary_name(column: &str) -> String {
    format!("{TEMPORARY_PREFIX}{column}")
}

/// Returns the physical column name for a column pending removal.
pub fn deletion_name(column: &str) -> String {
    format!("{DELETION_PREFIX}{column}")
}

/// Returns `true` when `name` is an internal physical column rather than a
/// client-visible one.
pub fn is_internal(name: &str) -> bool {
    name.starts_with(TEMPORARY_PREFIX) || name.starts_with(DELETION_PREFIX)
}

/// Strips any internal prefix, returning the logical column name.
pub fn logical_name(name: &str) -> &str {
    name.strip_prefix(TEMPORARY_PREFIX)
        .or_else(|| name.strip_prefix(DELETION_PREFIX))
        .unwrap_or(name)
}
