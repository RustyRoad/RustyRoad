//! Reviewable SQL proposals and their create-only filesystem outcomes.
use serde::Serialize;
use std::path::PathBuf;

/// Executing this SQL is never part of an audit.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Proposal {
    pub model_id: String,
    pub table: String,
    pub operation: String,
    pub columns: Vec<String>,
    pub up: String,
    pub down: String,
    pub warnings: Vec<String>,
}

/// Generated paths are metadata, not proof that a migration was applied.
#[derive(Debug, Serialize)]
pub(crate) struct MigrationFile {
    pub model_id: String,
    pub directory: PathBuf,
    pub state: String,
}
