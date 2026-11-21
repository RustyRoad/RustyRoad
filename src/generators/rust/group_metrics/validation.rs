use super::{predicate, ConditionalMetric, GroupMetrics, Names};
use crate::database::introspection::Schema;
use crate::generators::rust::repositories::equality;
use std::io;

/// Rejects unknown identifiers and incompatible predicate/aggregate codecs.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(
    schema: &Schema,
    names: &Names,
    name: &str,
    report: &GroupMetrics,
) -> io::Result<()> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid conditional report {name:?}: {reason}"),
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
    let table = schema
        .tables
        .iter()
        .find(|t| t.name == report.table)
        .ok_or_else(|| invalid("table absent from pull scope"))?;
    let keys = equality::keys(table);
    if keys.is_empty()
        || (!report.group_by.is_empty()
            && !keys.iter().any(|key| key == &vec![report.group_by.clone()]))
    {
        return Err(invalid("unsupported grouping column"));
    }
    let maximum = if report.group_by.is_empty() { 16 } else { 15 };
    if report.metrics.is_empty() || report.metrics.len() > maximum {
        return Err(invalid("metric count exceeds the supported tuple size"));
    }
    if let Some(column) = &report.filter_by {
        let column = table
            .column(column)
            .ok_or_else(|| invalid("runtime filter column absent"))?;
        if predicate::cast(schema, column).is_none() {
            return Err(invalid(
                "runtime filter must be scalar text or a catalog enum",
            ));
        }
    }
    for metric in &report.metrics {
        let (column, equals, value) = match metric {
            ConditionalMetric::CountAll => continue,
            ConditionalMetric::MaxTimestamp { column } => {
                if !table
                    .column(column)
                    .map(predicate::timestamp)
                    .unwrap_or(false)
                {
                    return Err(invalid(
                        "timestamp maximum requires a scalar timestamp column",
                    ));
                }
                continue;
            }
            ConditionalMetric::Count { column, equals } => (column, equals, None),
            ConditionalMetric::Sum {
                column,
                equals,
                value,
            } => (column, equals, Some(value)),
        };
        let predicate = table
            .column(column)
            .ok_or_else(|| invalid("predicate column absent"))?;
        if predicate::cast(schema, predicate).is_none()
            || !predicate::valid_label(schema, predicate, equals)
        {
            return Err(invalid(
                "predicate requires scalar text or a valid catalog enum label",
            ));
        }
        if let Some(value) = value {
            let value = table
                .column(value)
                .ok_or_else(|| invalid("sum column absent"))?;
            let kind = value.sql_type.trim().to_ascii_lowercase();
            if kind.ends_with("[]") {
                return Err(invalid("sum column must be scalar"));
            }
            let kind = kind.split('(').next().unwrap_or("").trim();
            if !matches!(
                kind,
                "smallint"
                    | "int2"
                    | "integer"
                    | "int4"
                    | "bigint"
                    | "int8"
                    | "numeric"
                    | "decimal"
                    | "real"
                    | "float4"
                    | "double precision"
                    | "float8"
            ) {
                return Err(invalid("sum column must have a supported numeric codec"));
            }
        }
    }
    Ok(())
}
