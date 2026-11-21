pub(super) fn map(name: &str) -> Option<&'static str> {
    Some(match name {
        "int" | "integer" | "tinyint" | "smallint" | "mediumint" | "bigint"
        | "unsigned big int" | "int2" | "int8" => "i64",
        "character" | "varchar" | "varying character" | "nchar" | "native character"
        | "nvarchar" | "text" | "clob" => "String",
        "blob" => "Vec<u8>",
        "real" | "double" | "double precision" | "float" => "f64",
        "boolean" | "bool" => "bool",
        "date" => "chrono::NaiveDate",
        "datetime" | "timestamp" => "chrono::NaiveDateTime",
        // SQLite's NUMERIC affinity is dynamic; no single exact decoder is safe.
        _ => return None,
    })
}
