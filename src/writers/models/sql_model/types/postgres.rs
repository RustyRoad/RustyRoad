pub(super) fn map(name: &str) -> Option<&'static str> {
    Some(match name {
        "smallint" | "int2" | "smallserial" | "serial2" => "i16",
        "integer" | "int" | "int4" | "serial" | "serial4" => "i32",
        "bigint" | "int8" | "bigserial" | "serial8" => "i64",
        "real" | "float4" => "f32",
        "double precision" | "float8" => "f64",
        "numeric" | "decimal" => "sqlx::types::Decimal",
        "boolean" | "bool" => "bool",
        "text" | "varchar" | "character varying" | "char" | "character" | "bpchar" => "String",
        "bytea" => "Vec<u8>",
        "uuid" => "uuid::Uuid",
        "json" | "jsonb" => "serde_json::Value",
        "date" => "chrono::NaiveDate",
        "time" | "time without time zone" => "chrono::NaiveTime",
        "timestamp" | "timestamp without time zone" => "chrono::NaiveDateTime",
        "timestamptz" | "timestamp with time zone" => "chrono::DateTime<chrono::Utc>",
        _ => return None,
    })
}
