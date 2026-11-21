use super::{Names, SpatialReport};
use crate::database::introspection::Schema;
use crate::generators::rust::repositories::equality;
use std::{collections::BTreeSet, io};

/// Rejects unknown identifiers, unsupported scalar selections and invalid GeoJSON sources.
/// SQLx tuple projections support at most sixteen selected fields including geometry.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(
    schema: &Schema,
    names: &Names,
    name: &str,
    report: &SpatialReport,
) -> io::Result<()> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid spatial report {name:?}: {reason}"),
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
        return Err(invalid("invalid or reserved Rust function name"));
    }
    let source = schema
        .tables
        .iter()
        .find(|t| t.name == report.source_table)
        .ok_or_else(|| invalid("source table absent from pull scope"))?;
    let target = schema
        .tables
        .iter()
        .find(|t| t.name == report.target_table)
        .ok_or_else(|| invalid("target table absent from pull scope"))?;
    let source_keys = equality::keys(source);
    let target_keys = equality::keys(target);
    for (columns, keys) in [
        (&report.source_columns, &source_keys),
        (&report.target_columns, &target_keys),
    ] {
        if columns.iter().collect::<BTreeSet<_>>().len() != columns.len()
            || columns
                .iter()
                .any(|column| !keys.iter().any(|key| key == &vec![column.clone()]))
        {
            return Err(invalid(
                "duplicate, missing or unsupported projection column",
            ));
        }
    }
    if report.source_columns.len() + report.target_columns.len() + 1 > 16
        || report.source_columns.is_empty()
        || !report.source_columns.contains(&report.order_by)
    {
        return Err(invalid(
            "invalid projection size or unselected ordering column",
        ));
    }
    for (column, keys) in [
        (&report.source_key, &source_keys),
        (&report.target_key, &target_keys),
    ] {
        if !keys.iter().any(|key| key == &vec![column.clone()]) {
            return Err(invalid("join column absent or unsupported"));
        }
    }
    let geometry = target
        .column(&report.geometry)
        .ok_or_else(|| invalid("geometry column absent"))?;
    let kind = geometry.sql_type.trim().to_ascii_lowercase();
    if kind != "geometry" && !(kind.starts_with("geometry(") && kind.ends_with(')')) {
        return Err(invalid(
            "GeoJSON projection requires a scalar PostGIS geometry",
        ));
    }
    Ok(())
}
