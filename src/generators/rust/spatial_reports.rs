//! Explicit spatial join projections using catalog-validated identifiers.
mod columns;
mod query;
mod validation;
use super::allocation::Names;
use crate::database::introspection::Schema;
use std::{collections::BTreeMap, io};

/// A source-filtered spatial join returning selected columns and GeoJSON geometry.
/// Joins and grouping are explicit configuration, never inferred relationships.
/// # Examples
/// ```toml
/// [models.rust_spatial_reports.campaign_scan_geometry]
/// source_table = "scan_events"
/// target_table = "parcels"
/// source_key = "parcel"
/// target_key = "parcel_id"
/// source_columns = ["campaign", "parcel", "scanned_at", "unique_id"]
/// target_columns = ["parcel_id", "address", "city", "state", "zip_code", "owner_name"]
/// geometry = "geometry"
/// order_by = "scanned_at"
/// descending = true
/// group_rows = true
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpatialReport {
    pub source_table: String,
    pub target_table: String,
    pub source_key: String,
    pub target_key: String,
    pub source_columns: Vec<String>,
    pub target_columns: Vec<String>,
    pub geometry: String,
    pub order_by: String,
    pub descending: bool,
    pub group_rows: bool,
}

/// Validates all spatial projections before returning generated repository code.
/// Returns typed configuration errors instead of partial or guessed SQL.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    reports: &BTreeMap<String, SpatialReport>,
) -> io::Result<String> {
    let mut output = String::new();
    for (name, report) in reports {
        validation::validate(schema, names, name, report)?;
        output.push_str(&query::render(schema, names, name, report));
    }
    Ok(output)
}
