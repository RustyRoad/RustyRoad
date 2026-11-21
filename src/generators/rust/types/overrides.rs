//! Explicit codecs for extension or project-owned PostgreSQL types.
use std::collections::BTreeMap;
use std::io;

/// A SQLx codec path and optional OpenAPI representation supplied by the project.
/// The codec must implement SQLx encode/decode and Serde for the declared SQL type.
/// # Examples
/// ```
/// use rustyroad::generators::rust::types::Override;
/// let codec = Override { rust: "geo_codec::Polygon".into(), schema: Some("String".into()) };
/// assert!(codec.schema.is_some());
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Override {
    pub rust: String,
    pub schema: Option<String>,
}

/// Validate all codec declarations before generation writes any output.
/// Reject duplicate normalized SQL names and non-path Rust codecs.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn validate(
    entries: &BTreeMap<String, Override>,
) -> io::Result<BTreeMap<String, Override>> {
    let mut validated = BTreeMap::new();
    for (sql, codec) in entries {
        let key = normalize(sql);
        let path = syn::parse_str::<syn::Path>(&codec.rust);
        let valid_path = path.as_ref().is_ok_and(|path| {
            path.segments
                .iter()
                .all(|segment| matches!(segment.arguments, syn::PathArguments::None))
        });
        let valid_schema = codec
            .schema
            .as_ref()
            .is_none_or(|schema| syn::parse_str::<syn::Type>(schema).is_ok());
        if key.is_empty() || !valid_path || !valid_schema || validated.contains_key(&key) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid or duplicate [models.rust_types] codec declaration",
            ));
        }
        validated.insert(key, codec.clone());
    }
    Ok(validated)
}

/// Normalize unquoted SQL declaration case and spacing without erasing modifiers.
/// Quoted identifiers preserve their exact case and spacing.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn normalize(sql: &str) -> String {
    if sql.contains('"') {
        return sql.trim().to_owned();
    }
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}
