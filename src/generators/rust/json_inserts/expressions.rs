use super::JsonInsert;
use crate::database::introspection::{Column, Table};
use crate::generators::rust::naming::sql;

/// Uses unbounded text parameters so input casting cannot truncate VARCHAR values.
/// Unsupported input codecs are rejected before rendering.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn cast(column: &Column) -> Option<&'static str> {
    let kind = column.sql_type.trim().to_ascii_lowercase();
    if kind.ends_with("[]") {
        return None;
    }
    if kind.starts_with("character varying")
        || kind.starts_with("varchar")
        || matches!(kind.as_str(), "text" | "char" | "character" | "bpchar")
    {
        return Some("text");
    }
    match kind.as_str() {
        "smallint" | "int2" | "smallserial" => Some("smallint"),
        "integer" | "int4" | "serial" => Some("integer"),
        "bigint" | "int8" | "bigserial" => Some("bigint"),
        "boolean" | "bool" => Some("boolean"),
        "real" | "float4" => Some("real"),
        "double precision" | "float8" => Some("double precision"),
        "uuid" => Some("uuid"),
        "jsonb" => Some("jsonb"),
        _ => None,
    }
}

/// Builds target expressions while retaining explicit-input precedence and JSON guards.
/// JSON keys are quoted literals from configuration; runtime values remain parameters.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn assigned(table: &Table, config: &JsonInsert) -> Vec<(String, String)> {
    let mut assigned: Vec<_> = config
        .columns
        .iter()
        .filter(|name| !config.integers.contains_key(*name))
        .map(|name| (name.clone(), format!("input.{}", sql(name))))
        .collect();
    for (target, integer) in &config.integers {
        let key = integer.key.replace(char::from(39u8), "''");
        let source = format!("input.{}", sql(&config.json_column));
        let kind = cast(table.column(target).unwrap()).unwrap();
        let value = format!("CASE WHEN jsonb_typeof({source}->'{key}')='number' THEN ({source}->>'{key}')::{kind} END");
        let value = if integer.prefer_input {
            format!("COALESCE(input.{}, {value})", sql(target))
        } else {
            value
        };
        assigned.push((target.clone(), value));
    }
    assigned.extend(config.booleans.iter().map(|(name, value)| {
        (
            name.clone(),
            if *value {
                "TRUE".into()
            } else {
                "FALSE".into()
            },
        )
    }));
    assigned
}
