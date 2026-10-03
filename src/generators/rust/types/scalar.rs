//! Scalar type lookup and type-name normalization.

use super::Mapping;

/// Maps a non-array, modifier-free Postgres type name.
pub(super) fn map(base: &str) -> Mapping {
    match base {
        "smallint" | "int2" | "smallserial" => Mapping::plain("i16"),
        "integer" | "int4" | "serial" => Mapping::plain("i32"),
        "bigint" | "int8" | "bigserial" => Mapping::plain("i64"),
        "boolean" | "bool" => Mapping::plain("bool"),
        "real" | "float4" => Mapping::plain("f32"),
        "double precision" | "float8" => Mapping::plain("f64"),
        "bytea" => Mapping::plain("Vec<u8>"),
        // `Decimal` (rust_decimal) rather than `BigDecimal`: both decode a Postgres numeric,
        // but utoipa's ToSchema derive only recognises `Decimal`, and the struct derives
        // ToSchema. BigDecimal produced a model that failed on its own derive.
        "numeric" | "decimal" | "money" => Mapping::overridden("sqlx::types::Decimal", "string"),
        "uuid" => Mapping::overridden("sqlx::types::Uuid", "string"),
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

/// Returns the element type of an array type, if the type is one.
///
/// `format_type` reports an array as `integer[]`, while the raw catalog reports `_int4`,
/// so both spellings are accepted.
pub(super) fn array_element(lowered: &str) -> Option<String> {
    if let Some(element) = lowered.strip_suffix("[]") {
        return Some(element.trim().to_string());
    }
    lowered
        .strip_prefix('_')
        .map(|element| element.trim().to_string())
}

/// Removes a parenthesized modifier wherever it appears in the type name.
///
/// A precision modifier sits mid-type in `timestamp(3) with time zone`, so trimming at
/// the first parenthesis would leave the suffix behind and break matching.
pub(super) fn strip_modifier(lowered: &str) -> String {
    let Some(start) = lowered.find('(') else {
        return lowered.trim().to_string();
    };
    let Some(end) = lowered.rfind(')') else {
        return lowered.trim().to_string();
    };

    format!("{}{}", &lowered[..start], &lowered[end + 1..])
        .trim()
        .to_string()
}
