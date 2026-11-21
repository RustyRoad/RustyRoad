//! URL-associated event totals retain unmatched source rows and join multiplicity.
mod query;
mod validation;
use super::allocation::Names;
use crate::database::introspection::Schema;
use std::{collections::BTreeMap, io};

/// An explicitly configured URL-pattern association followed by an event-key join.
/// Prefix/suffix retain PostgreSQL LIKE semantics; no relationship is inferred.
/// # Examples
/// ```toml
/// [models.rust_url_stats.design_scan_totals]
/// table = "door_hanger_designs"
/// key = "id"
/// label = "name"
/// links = "qr_codes"
/// url = "image_url"
/// link_key = "unique_id"
/// events = "scan_events"
/// event_key = "unique_id"
/// timestamp = "scanned_at"
/// prefix = "%door-hangers/%/"
/// suffix = "/%"
/// window_hours = 24
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UrlStats {
    pub table: String,
    pub key: String,
    pub label: String,
    pub links: String,
    pub url: String,
    pub link_key: String,
    pub events: String,
    pub event_key: String,
    pub timestamp: String,
    pub prefix: String,
    pub suffix: String,
    pub window_hours: u32,
}

/// Validates and emits typed source identity/label/count/recent-count tuples.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    reports: &BTreeMap<String, UrlStats>,
) -> io::Result<String> {
    let mut output = String::new();
    for (name, report) in reports {
        validation::validate(schema, names, name, report)?;
        output.push_str(&query::render(schema, names, name, report));
    }
    Ok(output)
}
