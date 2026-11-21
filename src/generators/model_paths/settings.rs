use super::*;

/// Optional project-wide `[models]` settings in the base rustyroad.toml.
/// # Examples
/// ```toml
/// [models]
/// source = "crates/models/src"
/// generated = "crates/db/src/models.rs"
/// ```
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Settings {
    /// Explicit report name -> grouped count and timestamp-range projection.
    #[serde(default)]
    pub rust_group_stats:
        std::collections::BTreeMap<String, crate::generators::rust::aggregates::GroupStats>,
    /// Explicit report name -> selected spatial relation projection.
    #[serde(default)]
    pub rust_spatial_reports:
        std::collections::BTreeMap<String, crate::generators::rust::spatial_reports::SpatialReport>,
    /// Explicit ordered conditional aggregate projections.
    #[serde(default)]
    pub rust_group_metrics:
        std::collections::BTreeMap<String, crate::generators::rust::group_metrics::GroupMetrics>,
    /// Explicit guarded relationship INSERT SELECT operations.
    #[serde(default)]
    pub rust_relation_inserts: std::collections::BTreeMap<
        String,
        crate::generators::rust::relation_inserts::RelationInsert,
    >,
    /// Explicit one-statement foreign-key identity writes.
    #[serde(default)]
    pub rust_related_writes:
        std::collections::BTreeMap<String, crate::generators::rust::related_writes::RelatedWrite>,
    /// Explicit narrow projections over generated typed filters.
    #[serde(default)]
    pub rust_projections:
        std::collections::BTreeMap<String, crate::generators::rust::projections::Projection>,
    /// Explicit signed integer balances with optional database-clock expiration.
    #[serde(default)]
    pub rust_balances:
        std::collections::BTreeMap<String, crate::generators::rust::balances::Balance>,
    /// Explicit URL-pattern-linked event count projections.
    #[serde(default)]
    pub rust_url_stats:
        std::collections::BTreeMap<String, crate::generators::rust::url_stats::UrlStats>,
    /// Explicit atomic inserts with JSON-derived nullable integer identities.
    #[serde(default)]
    pub rust_json_inserts:
        std::collections::BTreeMap<String, crate::generators::rust::json_inserts::JsonInsert>,
    pub source: Option<PathBuf>,
    pub generated: Option<PathBuf>,
    /// Project-wide Rust spelling -> PostgreSQL storage contract for model auditing.
    #[serde(default)]
    pub type_mappings: std::collections::BTreeMap<String, String>,
    /// PostgreSQL declaration -> explicitly supplied Rust codec and schema type.
    #[serde(default)]
    pub rust_types: std::collections::BTreeMap<String, crate::generators::rust::types::Override>,
    /// Exact namespace/relation/column codecs, validated against the selected catalog.
    #[serde(default)]
    pub rust_column_types: Vec<crate::generators::rust::types::ColumnOverride>,
    /// Automatic FK reads; omission keeps the historical enabled default.
    /// False retains models, ordinary operations and configured projections.
    #[serde(default)]
    pub rust_relationship_reads: Option<bool>,
}

/// Read only model settings, never exposing database configuration in diagnostics.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(crate) fn settings(root: &Path) -> Result<Settings, String> {
    let path = root.join("rustyroad.toml");
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Settings::default()),
        Err(_) => return Err("Cannot read project rustyroad.toml".into()),
    };
    let config: toml::Value =
        toml::from_str(&content).map_err(|_| "Invalid project TOML (contents redacted)")?;
    match config.get("models") {
        Some(value) => value.clone().try_into().map_err(|_| {
            "Invalid [models] paths, storage mappings, codecs, or configured Rust reports".into()
        }),
        None => Ok(Settings::default()),
    }
}
