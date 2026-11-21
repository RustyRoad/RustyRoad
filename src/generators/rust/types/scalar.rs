//! Scalar PostgreSQL types mapped to their actual SQLx codecs.
mod normalization;
use super::Mapping;
pub(in crate::generators::rust) use normalization::array_element;
pub(super) use normalization::strip_modifier;

/// Map a scalar, including the generated PgInterval wrapper.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn map(base: &str) -> Mapping {
    match base {
        "smallint" | "int2" | "smallserial" => Mapping::plain("i16"),
        "integer" | "int4" | "serial" => Mapping::plain("i32"),
        "bigint" | "int8" | "bigserial" => Mapping::plain("i64"),
        "boolean" | "bool" => Mapping::plain("bool"),
        "real" | "float4" => Mapping::plain("f32"),
        "double precision" | "float8" => Mapping::plain("f64"),
        "bytea" => Mapping::plain("Vec<u8>"),
        "interval" => Mapping::overridden("Interval", "any"),
        // `Decimal` (rust_decimal) rather than `BigDecimal`: both decode a Postgres numeric,
        // but utoipa's ToSchema derive only recognises `Decimal`, and the struct derives
        // ToSchema. BigDecimal produced a model that failed on its own derive.
        "numeric" | "decimal" | "money" => Mapping::overridden("sqlx::types::Decimal", "string"),
        "uuid" => Mapping::overridden("sqlx::types::Uuid", "string"),
        "inet" | "cidr" => Mapping::overridden("ipnetwork::IpNetwork", "string"),
        "json" | "jsonb" => Mapping::overridden("serde_json::Value", "any"),
        "date" => Mapping::overridden("chrono::NaiveDate", "string"),
        "time" | "time without time zone" => Mapping::overridden("chrono::NaiveTime", "string"),
        "timestamp" | "timestamp without time zone" => {
            Mapping::overridden("chrono::NaiveDateTime", "string")
        }
        // A timestamptz is an instant, not a wall clock, so it keeps its offset.
        // Collapsing it to NaiveDateTime silently drops the zone.
        "timestamptz" | "timestamp with time zone" => {
            Mapping::overridden("chrono::DateTime<chrono::Utc>", "string")
        }
        // Types sqlx has no dedicated decoder for still round-trip as text, which keeps
        // the rest of the struct usable instead of failing the whole run.
        _ => Mapping::plain("String"),
    }
}
