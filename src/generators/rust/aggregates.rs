//! Explicit grouped reports generated from verified catalog columns.
mod query;
mod validation;
use super::allocation::Names;
use crate::database::introspection::Schema;
use std::{collections::BTreeMap, io};

/// A configured count and timestamp-range projection over one existing table.
/// Fields name catalog identifiers; they never contain arbitrary SQL.
/// # Examples
/// ```
/// use rustyroad::generators::rust::aggregates::GroupStats;
/// let report = GroupStats {
///     table: "scan_events".into(), group_by: "campaign".into(),
///     timestamp: "scanned_at".into(),
/// };
/// assert_eq!(report.group_by, "campaign");
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupStats {
    pub table: String,
    pub group_by: String,
    pub timestamp: String,
}

/// Validates every configured report before returning generated repository text.
/// Unknown names, unsupported types and function collisions abort generation.
/// # Examples
/// ```toml
/// [models.rust_group_stats.campaign_scan_summary]
/// table = "scan_events"
/// group_by = "campaign"
/// timestamp = "scanned_at"
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    reports: &BTreeMap<String, GroupStats>,
) -> io::Result<String> {
    let mut output = String::new();
    for (name, report) in reports {
        let (table, group, timestamp) = validation::columns(schema, names, name, report)?;
        output.push_str(&query::render(schema, names, name, table, group, timestamp));
    }
    Ok(output)
}
