//! Serializable evidence, preserving independent field and contract differences.

mod report;
pub(super) use report::Report;
use serde::Serialize;
use std::{collections::BTreeMap, path::PathBuf};

/// One AST declaration; `id` includes its source path and inline module path.
#[derive(Clone, Debug, Serialize)]
pub(super) struct Model {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub line: u32,
    pub character: u32,
    pub kind: String,
    pub fields: BTreeMap<String, Field>,
    pub attributes: Vec<String>,
    pub generics: String,
    pub visibility: String,
    pub variants: Vec<String>,
}

/// Field syntax, not compiler type equality; e.g. `Option<i32>` is kept intact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct Field {
    pub ty: String,
    pub attributes: Vec<String>,
    pub visibility: String,
}

/// Independent differences; additional columns do not hide type changes.
#[derive(Clone, Debug, Default, Serialize)]
pub(super) struct Differences {
    pub custom_only: Vec<String>,
    pub generated_only: Vec<String>,
    pub type_changes: BTreeMap<String, [String; 2]>,
    pub field_contract_changes: Vec<String>,
    pub attributes_differ: bool,
    pub generics_differ: bool,
    pub visibility_differs: bool,
    pub variants_differ: bool,
}

/// Caller analysis is outside the schema audit; never encode unknown as zero usages.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum UsageAudit {
    NotRun,
}

/// A review candidate, never permission to delete a source declaration.
#[derive(Clone, Debug, Serialize)]
pub(super) struct Candidate {
    pub model: Model,
    pub targets: Vec<String>,
    pub target: Option<Model>,
    pub match_basis: String,
    pub classification: String,
    pub differences: Differences,
    pub blockers: Vec<String>,
    pub actions: Vec<String>,
    pub usage: UsageAudit,
    pub table_audit: Option<super::schema::types::TableAudit>,
}
