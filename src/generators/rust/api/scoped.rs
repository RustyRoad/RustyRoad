//! Full-artifact writer with explicit namespace provenance for column codecs.
use crate::database::introspection::{PartialUniqueIndex, Schema};
use crate::generators::report::Outcome;
use crate::generators::rust::{
    aggregates, balances, group_metrics, json_inserts, projections, related_writes,
    relation_inserts, spatial_reports, url_stats,
};
use crate::generators::rust::{
    allocation::Names,
    types::{ColumnOverride, Override},
};
use std::{collections::BTreeMap, io, path::Path};

/// Validate all scoped mappings and configured operations before writing output.
/// Offline callers must supply the namespace represented by their Schema snapshot.
/// Existing writers delegate with empty scoped entries and keep their signatures.
/// # Examples
/// ```no_run
/// # fn example(schema: &rustyroad::database::introspection::Schema) -> std::io::Result<()> {
/// use rustyroad::generators::rust::{write_api_with_scoped_reports, types::ColumnOverride};
/// let columns = [ColumnOverride { schema: "public".into(), table: "parcels".into(),
///     column: "geometry".into(), sql_type: "geometry(Polygon,4326)".into(),
///     rust: "geography::DbPolygon".into(), nullable: Some(false), openapi_type: None }];
/// write_api_with_scoped_reports(std::path::Path::new("generated-db"), schema, false,
///     &Default::default(), &Default::default(), &Default::default(), &Default::default(),
///     &Default::default(), &Default::default(), &Default::default(), &Default::default(),
///     &Default::default(), &Default::default(), &[], "public", &columns)?;
/// # Ok(()) }
/// ```
pub fn write(
    out: &Path,
    schema: &Schema,
    force: bool,
    codecs: &BTreeMap<String, Override>,
    reports: &BTreeMap<String, aggregates::GroupStats>,
    spatial: &BTreeMap<String, spatial_reports::SpatialReport>,
    metrics: &BTreeMap<String, group_metrics::GroupMetrics>,
    inserts: &BTreeMap<String, relation_inserts::RelationInsert>,
    related_writes: &BTreeMap<String, related_writes::RelatedWrite>,
    projections: &BTreeMap<String, projections::Projection>,
    balances: &BTreeMap<String, balances::Balance>,
    url_stats: &BTreeMap<String, url_stats::UrlStats>,
    json_inserts: &BTreeMap<String, json_inserts::JsonInsert>,
    partial_keys: &[PartialUniqueIndex],
    namespace: &str,
    columns: &[ColumnOverride],
) -> io::Result<Vec<Outcome>> {
    write_with_generation(
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
        namespace,
        columns,
        true,
    )
}

/// Write the full selected catalog, optionally omitting automatic relationship reads.
/// Models, procedures, configured operations and validation remain unchanged.
/// False omits automatic FK joins and two-edge reads, not ordinary filters.
/// Full name reservations are retained, including suppressed read names.
/// Invalid configuration returns an error before any artifacts are written.
/// # Examples
/// ```no_run
/// # fn example(schema: &rustyroad::database::introspection::Schema) -> std::io::Result<()> {
/// use rustyroad::generators::rust::write_api_with_scoped_generation;
/// write_api_with_scoped_generation(std::path::Path::new("generated-db"), schema, false,
///     &Default::default(), &Default::default(), &Default::default(), &Default::default(),
///     &Default::default(), &Default::default(), &Default::default(), &Default::default(),
///     &Default::default(), &Default::default(), &[], "public", &[], false)?;
/// # Ok(()) }
/// ```
pub fn write_with_generation(
    out: &Path,
    schema: &Schema,
    force: bool,
    codecs: &BTreeMap<String, Override>,
    reports: &BTreeMap<String, aggregates::GroupStats>,
    spatial: &BTreeMap<String, spatial_reports::SpatialReport>,
    metrics: &BTreeMap<String, group_metrics::GroupMetrics>,
    inserts: &BTreeMap<String, relation_inserts::RelationInsert>,
    related_writes: &BTreeMap<String, related_writes::RelatedWrite>,
    projections: &BTreeMap<String, projections::Projection>,
    balances: &BTreeMap<String, balances::Balance>,
    url_stats: &BTreeMap<String, url_stats::UrlStats>,
    json_inserts: &BTreeMap<String, json_inserts::JsonInsert>,
    partial_keys: &[PartialUniqueIndex],
    namespace: &str,
    columns: &[ColumnOverride],
    relationship_reads: bool,
) -> io::Result<Vec<Outcome>> {
    let names = Names::with_partial_keys(schema, codecs, partial_keys)?
        .with_columns(schema, namespace, columns)?;
    let configured = super::configured::render(
        schema,
        &names,
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
    )?;
    super::output::write(out, schema, &names, force, &configured, relationship_reads)
}
