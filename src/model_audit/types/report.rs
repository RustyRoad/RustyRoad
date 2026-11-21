//! Report envelope retaining discovery independently of declaration selection.
use serde::Serialize;
use std::path::PathBuf;

/// Machine-readable audit; selected candidates carry stable IDs for another run.
#[derive(Debug, Serialize)]
pub(in crate::model_audit) struct Report {
    pub summary: crate::model_audit::summary::Summary,
    pub version: u32,
    pub read_only: bool,
    pub root: PathBuf,
    pub context: crate::model_audit::context::Context,
    pub generated: Option<PathBuf>,
    pub catalog: Option<crate::model_audit::schema::types::CatalogEvidence>,
    pub migration_files: Vec<crate::model_audit::schema::types::MigrationFile>,
    pub analysis: String,
    pub diagnostics: Vec<String>,
    pub limitations: Vec<String>,
    pub discovered: usize,
    pub selected: usize,
    pub source_files: Vec<crate::model_audit::discover::SourceFile>,
    pub declaration_coverage: Vec<crate::model_audit::coverage::Declaration>,
    pub candidates: Vec<super::Candidate>,
}
