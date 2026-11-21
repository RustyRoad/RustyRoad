//! Table-level evidence and reviewable schema-reconciliation migration proposals.
mod artifacts;
mod columns;

pub(crate) use super::catalog::{Catalog, PrimaryKey, Table};
pub(crate) use artifacts::{MigrationFile, Proposal};
pub(crate) use columns::{CatalogEvidence, Column, ColumnChange};
use serde::Serialize;
use std::collections::BTreeMap;

/// Findings remain present even when the declaration happens to match another model.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TableAudit {
    pub shared_models: Vec<String>,
    pub status: String,
    pub table: Option<String>,
    pub mapping: String,
    pub candidates: Vec<String>,
    pub missing_columns: Vec<Column>,
    pub column_changes: Vec<ColumnChange>,
    pub desired_columns: Vec<Column>,
    pub database_only: Vec<String>,
    pub incompatible_columns: BTreeMap<String, String>,
    pub blockers: Vec<String>,
    pub proposal: Option<Proposal>,
}
