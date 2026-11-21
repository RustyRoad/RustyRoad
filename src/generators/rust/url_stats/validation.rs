use super::{Names, UrlStats};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{repositories::equality, types};
use std::io;

/// Validates explicit relation/column identities and compatible event-key codecs.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(
    schema: &Schema,
    names: &Names,
    name: &str,
    report: &UrlStats,
) -> io::Result<()> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid URL statistics {name:?}: {reason}"),
        )
    };
    if name.is_empty()
        || !name.starts_with(|c: char| c.is_ascii_lowercase())
        || !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        || syn::parse_str::<syn::Ident>(name).is_err()
        || names.function_reserved(name)
    {
        return Err(invalid("invalid or reserved function name"));
    }
    let table = |name: &str| {
        schema
            .table(name)
            .ok_or_else(|| invalid("relation outside pull scope"))
    };
    let source = table(&report.table)?;
    let links = table(&report.links)?;
    let events = table(&report.events)?;
    if source.primary_key != vec![report.key.clone()]
        || report.key == report.label
        || !(1..=876000).contains(&report.window_hours)
        || equality::keys(source).is_empty()
    {
        return Err(invalid(
            "requires one source primary key, distinct label and bounded positive hours",
        ));
    }
    let codec = |table: &Table, column: &str| -> io::Result<String> {
        let mut column = table
            .column(column)
            .ok_or_else(|| invalid("unknown column"))?
            .clone();
        column.nullable = false;
        Ok(types::map_column(table, &column, schema, names).rust)
    };
    if !equality::keys(source).contains(&vec![report.key.clone()]) {
        return Err(invalid("source key must have a scalar codec"));
    }
    codec(source, &report.label)?;
    if codec(links, &report.url)? != "String"
        || codec(links, &report.link_key)? != codec(events, &report.event_key)?
    {
        return Err(invalid(
            "URL must be text and event join keys must have matching codecs",
        ));
    }
    if !equality::keys(links).contains(&vec![report.link_key.clone()])
        || !equality::keys(events).contains(&vec![report.event_key.clone()])
    {
        return Err(invalid("event join keys must be supported scalars"));
    }
    let timestamp = events
        .column(&report.timestamp)
        .ok_or_else(|| invalid("missing event timestamp"))?;
    let kind = timestamp.sql_type.trim().to_ascii_lowercase();
    if !(kind.starts_with("timestamp") || kind == "timestamptz") || kind.ends_with("[]") {
        return Err(invalid("event time must be a scalar timestamp"));
    }
    Ok(())
}
