//! Writing the canonical flat Rust API folder.

mod scoped;
pub use scoped::{
    write as write_with_scoped_reports, write_with_generation as write_with_scoped_generation,
};
mod configured;
mod output;
use crate::database::introspection::Schema;
use crate::generators::report::Outcome;
use std::io;
use std::path::Path;

/// Writes Rust API output using standard PostgreSQL codecs.
/// # Examples
/// ```no_run
/// # fn example(schema: &rustyroad::database::introspection::Schema) -> std::io::Result<()> {
/// rustyroad::generators::rust::write_api(std::path::Path::new("generated-db"), schema, false)?;
/// # Ok(()) }
/// ```
pub fn write(out: &Path, schema: &Schema, force: bool) -> io::Result<Vec<Outcome>> {
    write_with_codecs(out, schema, force, &std::collections::BTreeMap::new())
}

/// Writes Rust API output after validating project-supplied codec declarations.
/// # Examples
/// ```no_run
/// # fn example(schema: &rustyroad::database::introspection::Schema) -> std::io::Result<()> {
/// use rustyroad::generators::rust::{write_api_with_codecs, types::Override};
/// let codecs = std::collections::BTreeMap::from([("geometry(Polygon,4326)".into(),
///     Override { rust: "geo_codec::Polygon".into(), schema: Some("String".into()) })]);
/// write_api_with_codecs(std::path::Path::new("generated-db"), schema, false, &codecs)?;
/// # Ok(()) }
/// ```
pub fn write_with_codecs(
    out: &Path,
    schema: &Schema,
    force: bool,
    codecs: &std::collections::BTreeMap<String, super::types::Override>,
) -> io::Result<Vec<Outcome>> {
    write_with_reports(
        out,
        schema,
        force,
        codecs,
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &[],
    )
}

/// Generates configured reads and guarded relationship writes with the canonical API.
/// Invalid reports abort before writing any files; reports never create tables.
/// # Examples
/// ```no_run
/// # fn example(schema: &rustyroad::database::introspection::Schema) -> std::io::Result<()> {
/// use rustyroad::generators::rust::write_api_with_reports;
/// write_api_with_reports(std::path::Path::new("generated-db"), schema, false,
///     &Default::default(), &Default::default(), &Default::default(), &Default::default(), &Default::default(), &Default::default(), &Default::default(), &Default::default(), &Default::default(), &Default::default(), &[])?;
/// # Ok(()) }
/// ```
#[allow(
    clippy::too_many_arguments,
    reason = "Preserve the published positional writer API"
)]
pub fn write_with_reports(
    out: &Path,
    schema: &Schema,
    force: bool,
    codecs: &std::collections::BTreeMap<String, super::types::Override>,
    reports: &std::collections::BTreeMap<String, super::aggregates::GroupStats>,
    spatial: &std::collections::BTreeMap<String, super::spatial_reports::SpatialReport>,
    metrics: &std::collections::BTreeMap<String, super::group_metrics::GroupMetrics>,
    inserts: &std::collections::BTreeMap<String, super::relation_inserts::RelationInsert>,
    related_writes: &std::collections::BTreeMap<String, super::related_writes::RelatedWrite>,
    projections: &std::collections::BTreeMap<String, super::projections::Projection>,
    balances: &std::collections::BTreeMap<String, super::balances::Balance>,
    url_stats: &std::collections::BTreeMap<String, super::url_stats::UrlStats>,
    json_inserts: &std::collections::BTreeMap<String, super::json_inserts::JsonInsert>,
    partial_keys: &[crate::database::introspection::PartialUniqueIndex],
) -> io::Result<Vec<Outcome>> {
    scoped::write(
        out,
        schema,
        force,
        codecs,
        reports,
        spatial,
        metrics,
        inserts,
        related_writes,
        projections,
        balances,
        url_stats,
        json_inserts,
        partial_keys,
        "",
        &[],
    )
}
