use crate::database::introspection::{PartialUniqueIndex, Schema};
use crate::generators::rust::{
    aggregates, allocation::Names, balances, group_metrics, json_inserts, projections,
    related_writes, relation_inserts, spatial_reports, url_stats,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

/// Named inputs for validating and rendering configured operations as one unit.
/// Public writers keep their existing signatures; this bundle is internal only.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) struct Operations<'a> {
    pub reports: &'a BTreeMap<String, aggregates::GroupStats>,
    pub spatial: &'a BTreeMap<String, spatial_reports::SpatialReport>,
    pub metrics: &'a BTreeMap<String, group_metrics::GroupMetrics>,
    pub inserts: &'a BTreeMap<String, relation_inserts::RelationInsert>,
    pub related_writes: &'a BTreeMap<String, related_writes::RelatedWrite>,
    pub projections: &'a BTreeMap<String, projections::Projection>,
    pub balances: &'a BTreeMap<String, balances::Balance>,
    pub url_stats: &'a BTreeMap<String, url_stats::UrlStats>,
    pub json_inserts: &'a BTreeMap<String, json_inserts::JsonInsert>,
    pub partial_keys: &'a [PartialUniqueIndex],
}

/// Validates all configured operation names before rendering any output artifact.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    operations: Operations<'_>,
) -> io::Result<String> {
    let Operations {
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
    } = operations;
    let mut configured_names = BTreeSet::new();
    for name in reports
        .keys()
        .chain(spatial.keys())
        .chain(metrics.keys())
        .chain(inserts.keys())
        .chain(related_writes.keys())
        .chain(projections.keys())
        .chain(balances.keys())
        .chain(url_stats.keys())
        .chain(json_inserts.keys())
    {
        if !configured_names.insert(name) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Duplicate configured report name",
            ));
        }
    }
    let mut aggregates = crate::generators::rust::aggregates::render(schema, names, reports)?;
    aggregates.push_str(&crate::generators::rust::spatial_reports::render(
        schema, names, spatial,
    )?);
    aggregates.push_str(&crate::generators::rust::group_metrics::render(
        schema, names, metrics,
    )?);
    aggregates.push_str(&crate::generators::rust::relation_inserts::render(
        schema, names, inserts,
    )?);
    aggregates.push_str(
        &crate::generators::rust::repositories::partial_insert::render(
            schema,
            names,
            partial_keys,
        )?,
    );
    aggregates.push_str(&crate::generators::rust::related_writes::render(
        schema,
        names,
        related_writes,
    )?);
    aggregates.push_str(&crate::generators::rust::projections::render(
        schema,
        names,
        projections,
    )?);
    aggregates.push_str(&crate::generators::rust::balances::render(
        schema, names, balances,
    )?);
    aggregates.push_str(&crate::generators::rust::url_stats::render(
        schema, names, url_stats,
    )?);
    aggregates.push_str(&crate::generators::rust::json_inserts::render(
        schema,
        names,
        json_inserts,
    )?);
    Ok(aggregates)
}
