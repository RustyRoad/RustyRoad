use super::{filters, keys, Names, Table};

/// Reuses the canonical typed equality predicates for configured aggregate queries.
/// # Examples
/// ```toml
/// [models.rust_group_stats.campaign_scan_summary]
/// table = "scan_events"
/// group_by = "campaign"
/// timestamp = "scanned_at"
/// ```
pub(in crate::generators::rust) fn aggregate_bindings(table: &Table, names: &Names) -> String {
    let columns: Vec<_> = keys(table)
        .iter()
        .map(|key| table.column(&key[0]).unwrap())
        .collect();
    filters::fields::bindings(table, &columns, names)
}

/// Reuses qualified source predicates for configured spatial joins.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn qualified_aggregate_bindings(
    table: &Table,
    names: &Names,
    alias: &str,
) -> String {
    let columns: Vec<_> = keys(table)
        .iter()
        .map(|key| table.column(&key[0]).unwrap())
        .collect();
    filters::fields::bindings_for(table, &columns, names, "filter", Some(alias))
}
