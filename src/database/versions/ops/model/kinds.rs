//! Operation definitions for declarative migrations.

use serde::{Deserialize, Serialize};

/// Adds a column to an existing table.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddColumn {
    pub table: String,
    pub column: String,
    /// SQL type of the new column.
    #[serde(rename = "type")]
    pub column_type: String,
    #[serde(default)]
    pub nullable: bool,
    #[serde(default)]
    pub default: Option<String>,
    /// Expression populating existing rows, evaluated per row.
    #[serde(default)]
    pub up: Option<String>,
}

/// Alters an existing column.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlterColumn {
    pub table: String,
    pub column: String,
    /// New logical name, when renaming.
    #[serde(default)]
    pub name: Option<String>,
    /// New SQL type, when changing type.
    #[serde(default, rename = "type")]
    pub column_type: Option<String>,
    #[serde(default)]
    pub nullable: Option<bool>,
    /// Expression converting old values to new, for readers of the new version.
    #[serde(default)]
    pub up: Option<String>,
    /// Expression converting new values back, for readers of the old version.
    #[serde(default)]
    pub down: Option<String>,
}

/// Drops a column.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DropColumn {
    pub table: String,
    pub column: String,
    /// Expression keeping the column populated for old readers until completion.
    #[serde(default)]
    pub down: Option<String>,
}

/// Raw SQL escape hatch.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawSql {
    pub up: String,
    #[serde(default)]
    pub down: Option<String>,
}
