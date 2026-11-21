//! Known PostgreSQL codec identities, never guessed from arbitrary custom names.
use super::wrappers::{generic, scalar};
use syn::Type;

/// Infer known codecs, including SQLx's canonical chrono/time re-exports.
/// Example: `sqlx::types::time::PrimitiveDateTime` has no time zone.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn sql(ty: &Type) -> Option<String> {
    let Some(name) = scalar(ty) else {
        return container(ty);
    };
    let scalar = match name.as_str() {
        "bool" | "std::primitive::bool" => "boolean",
        "i8" | "i16" | "std::primitive::i16" => "smallint",
        "i32" | "std::primitive::i32" => "integer",
        "i64" | "std::primitive::i64" => "bigint",
        "f32" | "std::primitive::f32" => "real",
        "f64" | "std::primitive::f64" => "double precision",
        "String" | "std::string::String" | "alloc::string::String" => "text",
        "uuid::Uuid" | "sqlx::types::Uuid" => "uuid",
        "sqlx::postgres::types::PgInterval" => "interval",
        "std::net::IpAddr" | "core::net::IpAddr" => "inet",
        "serde_json::Value" | "sqlx::types::JsonValue" => "jsonb",
        "chrono::NaiveDate"
        | "sqlx::types::chrono::NaiveDate"
        | "time::Date"
        | "sqlx::types::time::Date" => "date",
        "chrono::NaiveDateTime"
        | "sqlx::types::chrono::NaiveDateTime"
        | "time::PrimitiveDateTime"
        | "sqlx::types::time::PrimitiveDateTime" => "timestamp without time zone",
        "chrono::NaiveTime"
        | "sqlx::types::chrono::NaiveTime"
        | "time::Time"
        | "sqlx::types::time::Time" => "time without time zone",
        "time::OffsetDateTime" | "sqlx::types::time::OffsetDateTime" => "timestamp with time zone",
        "rust_decimal::Decimal"
        | "sqlx::types::Decimal"
        | "bigdecimal::BigDecimal"
        | "sqlx::types::BigDecimal" => "numeric",
        _ => return container(ty),
    };
    Some(scalar.into())
}

/// Infer known containers: `Vec<i32>` is `integer[]`, not serialized JSON.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
fn container(ty: &Type) -> Option<String> {
    if let Some(inner) = generic(ty, &["Vec", "std::vec::Vec", "alloc::vec::Vec"]) {
        return if scalar(inner).as_deref() == Some("u8") {
            Some("bytea".into())
        } else {
            sql(inner).map(|sql| format!("{sql}[]"))
        };
    }
    if let Some(zone) = generic(ty, &["chrono::DateTime", "sqlx::types::chrono::DateTime"]) {
        return match scalar(zone)?.as_str() {
            "chrono::Utc"
            | "chrono::Local"
            | "chrono::FixedOffset"
            | "sqlx::types::chrono::Utc"
            | "sqlx::types::chrono::Local"
            | "sqlx::types::chrono::FixedOffset" => Some("timestamp with time zone".into()),
            _ => None,
        };
    }
    generic(ty, &["sqlx::types::Json"]).map(|_| "jsonb".into())
}
