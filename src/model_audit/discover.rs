//! Deterministic source discovery without running Cargo.
mod files;
mod runtime_sql;
mod walk;
use super::types::Model;
pub(super) use files::SourceFile;
use std::path::{Path, PathBuf};

/// Compatibility inventory for callers needing declarations and diagnostics only.
pub(super) fn scan(
    root: &Path,
    sources: &[PathBuf],
    generated: Option<&Path>,
) -> (Vec<Model>, Vec<String>) {
    let (models, diagnostics, _) = inventory(root, sources, generated);
    (models, diagnostics)
}

/// Account for every Rust file, e.g. an empty module or malformed row declaration.
pub(super) fn inventory(
    root: &Path,
    sources: &[PathBuf],
    generated: Option<&Path>,
) -> (Vec<Model>, Vec<String>, Vec<SourceFile>) {
    let mut models = Vec::new();
    let mut diagnostics = Vec::new();
    if sources.is_empty() {
        diagnostics.push("No source roots found; supply --source".into());
    }
    let mut paths = std::collections::BTreeSet::new();
    for path in sources {
        walk::walk(path, &mut paths, &mut diagnostics);
    }
    let files = paths
        .iter()
        .map(|path| files::inspect(path, root, generated, &mut models, &mut diagnostics))
        .collect();
    models.sort_by(|a, b| a.id.cmp(&b.id));
    (models, diagnostics, files)
}

/// Locate canonical generated models; ambiguous installations require explicit selection.
pub(super) fn generated(root: &Path, explicit: Option<&String>) -> Result<PathBuf, String> {
    crate::generators::model_paths::resolve_with(root, explicit.map(Path::new))?
        .generated
        .ok_or_else(|| "No generated models found; use --generated".to_string())?
        .canonicalize()
        .map_err(|e| e.to_string())
}
