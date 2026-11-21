//! Observed column facts, catalog provenance, and reversible column differences.
use super::Catalog;
use serde::{Deserialize, Serialize};

/// Observed PostgreSQL type and nullability; constraints are intentionally separate.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub(crate) struct Column {
    pub name: String,
    pub sql_type: String,
    pub nullable: bool,
}

/// Database evidence is separate from Rust declaration comparison.
#[derive(Debug, Serialize)]
pub(crate) struct CatalogEvidence {
    pub source: String,
    pub catalog: Catalog,
}

/// A requested change retains observed facts to render an honest rollback.
#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub(crate) struct ColumnChange {
    pub before: Column,
    pub after: Column,
}
