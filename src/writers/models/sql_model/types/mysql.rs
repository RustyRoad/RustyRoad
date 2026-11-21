pub(super) fn map(name: &str) -> Option<&'static str> {
    Some(match name {
        "tinyint" | "tinyint signed" => "i8",
        "smallint" | "smallint signed" => "i16",
        "mediumint" | "mediumint signed" | "int" | "integer" | "int signed" => "i32",
        "bigint" | "bigint signed" => "i64",
        "tinyint unsigned" => "u8",
        "smallint unsigned" => "u16",
        "mediumint unsigned" | "int unsigned" | "integer unsigned" => "u32",
        "bigint unsigned" => "u64",
        "float" | "real" => "f32",
        "double" | "double precision" => "f64",
        "numeric" | "decimal" | "dec" => "sqlx::types::Decimal",
        "bool" | "boolean" => "bool",
        "char" | "varchar" | "tinytext" | "text" | "mediumtext" | "longtext" => "String",
        "binary" | "varbinary" | "tinyblob" | "blob" | "mediumblob" | "longblob" => "Vec<u8>",
        "json" => "serde_json::Value",
        "date" => "chrono::NaiveDate",
        "time" => "chrono::NaiveTime",
        "timestamp" | "datetime" => "chrono::NaiveDateTime",
        _ => return None,
    })
}
