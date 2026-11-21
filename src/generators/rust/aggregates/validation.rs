use super::{GroupStats, Names};
use crate::database::introspection::{Column, Schema, Table};
use crate::generators::rust::repositories::equality;
use std::io;

/// Resolves configured columns without inventing tables, codecs or identifiers.
/// # Examples
/// ```toml
/// [models.rust_group_stats.campaign_scan_summary]
/// table = "scan_events"
/// group_by = "campaign"
/// timestamp = "scanned_at"
/// ```
pub(super) fn columns<'a>(
    schema: &'a Schema,
    names: &Names,
    function: &str,
    report: &GroupStats,
) -> io::Result<(&'a Table, &'a Column, &'a Column)> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid grouped report {function:?}: {reason}"),
        )
    };
    if function.is_empty()
        || !function.starts_with(|c: char| c.is_ascii_lowercase())
        || !function
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        || syn::parse_str::<syn::Ident>(function).is_err()
        || names.function_reserved(function)
    {
        return Err(invalid("invalid or reserved Rust function name"));
    }
    let table = schema
        .tables
        .iter()
        .find(|table| table.name == report.table)
        .ok_or_else(|| invalid("table absent from pull scope"))?;
    let group = table
        .column(&report.group_by)
        .ok_or_else(|| invalid("group column absent from table"))?;
    if !equality::keys(table)
        .iter()
        .any(|key| key == &vec![group.name.clone()])
    {
        return Err(invalid("unsupported group column codec"));
    }
    let timestamp = table
        .column(&report.timestamp)
        .ok_or_else(|| invalid("timestamp column absent from table"))?;
    let kind = timestamp.sql_type.trim().to_ascii_lowercase();
    if kind.ends_with("[]")
        || !(kind.starts_with("timestamp") || kind == "timestamptz")
        || !equality::keys(table)
            .iter()
            .any(|key| key == &vec![timestamp.name.clone()])
    {
        return Err(invalid("range column must use a PostgreSQL timestamp type"));
    }
    Ok((table, group, timestamp))
}
