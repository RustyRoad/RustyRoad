use super::*;

/// Use configured paths, then workspace generator layout, then the conventional src layout.
/// # Examples
/// ```sh
/// rustyroad model audit --offline --report-only
/// ```
pub(crate) fn resolve(root: &Path) -> Result<Layout, String> {
    resolve_with(root, None)
}

/// Explicit audit overrides beat configured generated paths and layout ambiguity.
/// # Examples
/// ```sh
/// rustyroad model audit --generated crates/db/src/models.rs --offline --report-only
/// ```
pub(crate) fn resolve_with(
    root: &Path,
    generated_override: Option<&Path>,
) -> Result<Layout, String> {
    let settings = settings(root)?;
    let source_explicit = settings.source.is_some();
    let source = source_directory(root, settings.source);
    let generated_explicit = generated_override.is_some() || settings.generated.is_some();
    let generated = if let Some(path) = generated_override {
        Some(root.join(path))
    } else if let Some(path) = settings.generated {
        if path.file_name().is_none_or(|name| name != "models.rs") {
            return Err("[models].generated must name a models.rs file (not a folder)".into());
        }
        Some(root.join(path))
    } else {
        let candidates: Vec<_> = ["crates/db/src/models.rs", "src/db/models.rs"]
            .iter()
            .map(|p| root.join(p))
            .filter(|p| p.is_file())
            .collect();
        match candidates.as_slice() {
            [path] => Some(path.clone()),
            [] => None,
            _ => return Err(
                "Multiple generated model files found; set [models].generated or pass --generated"
                    .into(),
            ),
        }
    };
    Ok(Layout {
        source,
        source_explicit,
        generated,
        generated_explicit,
    })
}
