//! Postgres-to-Rust type mapping used by models and procedure paths.

use super::naming::pascal;
use crate::database::introspection::{Column, Schema};

/// Returns a Rust type suitable for SQLx's Postgres implementation.
pub(super) fn column_type(column: &Column, schema: &Schema) -> String {
    let base = base_type(&column.sql_type, schema);
    if column.nullable {
        format!("Option<{base}>")
    } else {
        base
    }
}

/// Returns the non-null Rust type for a column.
pub(super) fn base_type(sql_type: &str, schema: &Schema) -> String {
    let lowered = sql_type.trim().to_ascii_lowercase();
    if let Some(element) = lowered.strip_suffix("[]") {
        return format!("Vec<{}>", base_type(element, schema));
    }

    let unqualified = lowered
        .rsplit('.')
        .next()
        .unwrap_or(&lowered)
        .trim_matches('"');
    if let Some(item) = schema.enums.iter().find(|item| item.name == unqualified) {
        return pascal(&item.name);
    }

    let normalized = strip_modifier(unqualified);
    match normalized.as_str() {
        "smallint" | "int2" | "smallserial" => "i16".to_string(),
        "integer" | "int4" | "serial" => "i32".to_string(),
        "bigint" | "int8" | "bigserial" => "i64".to_string(),
        "real" | "float4" => "f32".to_string(),
        "double precision" | "float8" => "f64".to_string(),
        "numeric" | "decimal" => "bigdecimal::BigDecimal".to_string(),
        "boolean" | "bool" => "bool".to_string(),
        "uuid" => "uuid::Uuid".to_string(),
        "json" | "jsonb" => "serde_json::Value".to_string(),
        "date" => "chrono::NaiveDate".to_string(),
        "timestamp" | "timestamp without time zone" => "chrono::NaiveDateTime".to_string(),
        "timestamp with time zone" | "timestamptz" => "chrono::DateTime<chrono::Utc>".to_string(),
        "time" | "time without time zone" => "chrono::NaiveTime".to_string(),
        "bytea" => "Vec<u8>".to_string(),
        "inet" | "cidr" => "ipnetwork::IpNetwork".to_string(),
        "macaddr" | "macaddr8" => "mac_address::MacAddress".to_string(),
        "interval" => "Interval".to_string(),
        _ => "String".to_string(),
    }
}

fn strip_modifier(value: &str) -> String {
    let Some(start) = value.find('(') else {
        return value.trim().to_string();
    };
    let Some(end) = value.rfind(')') else {
        return value.trim().to_string();
    };
    format!("{}{}", &value[..start], &value[end + 1..])
        .trim()
        .to_string()
}
