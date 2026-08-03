//! Postgres identifier quoting.
//!
//! View and schema names are derived from user-supplied migration names and table
//! names, so every identifier is quoted rather than interpolated bare.

/// Quotes an identifier for Postgres, escaping embedded double quotes.
pub fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Quotes a string literal for Postgres, escaping embedded single quotes.
pub fn quote_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

/// Quotes a possibly schema-qualified pair as `"schema"."name"`.
pub fn quote_qualified(schema: &str, name: &str) -> String {
    format!("{}.{}", quote_ident(schema), quote_ident(name))
}
