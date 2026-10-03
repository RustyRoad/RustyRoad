//! Data model for enum inspection results.

use serde::Serialize;

/// One enum type (Postgres) or enum column (MySQL) as reported by `db enums`.
#[derive(Debug, Serialize)]
pub struct DatabaseEnum {
    pub name: String,
    pub values: Vec<String>,
    /// Set for backends where an enum belongs to a column rather than a type
    /// catalog (MySQL). None for Postgres, whose enums are standalone types.
    pub table: Option<String>,
    pub column: Option<String>,
}

/// All enum types of the configured database.
#[derive(Debug, Serialize)]
pub struct EnumOutput {
    pub database_type: String,
    pub config_file: String,
    pub enums: Vec<DatabaseEnum>,
}
