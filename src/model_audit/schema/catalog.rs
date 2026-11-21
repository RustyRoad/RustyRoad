//! Catalog identity, separate from proposed column changes.

use super::types::Column;
use serde::{Deserialize, Serialize};

/// Complete inventory; e.g. an empty schema has `complete: true, tables: []`.
/// A filtered inventory cannot prove absence.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Catalog {
    pub version: u32,
    pub schema: String,
    pub complete: bool,
    pub tables: Vec<Table>,
    pub reserved_names: Vec<String>,
}

/// Relation facts, including keys that must not silently become nullable.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Table {
    pub name: String,
    pub kind: String,
    pub columns: Vec<Column>,
    pub primary_key: Vec<String>,
    #[serde(default)]
    pub primary_key_details: Option<PrimaryKey>,
}

/// Observed key identity and dependencies; missing metadata cannot authorize redesign.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PrimaryKey {
    pub name: String,
    pub deferrable: bool,
    pub initially_deferred: bool,
    pub referenced_by: Vec<String>,
    pub standard_index: bool,
}
