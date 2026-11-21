//! Reusable custom storage contracts, parsed as types rather than interpolated SQL.

use sqlparser::{dialect::PostgreSqlDialect, parser::Parser, tokenizer::Token};
use std::{collections::BTreeMap, path::Path};

/// Canonical Rust syntax, not compiler identity; map imported spellings explicitly.
pub(super) fn spelling(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string()
}

/// Load project defaults and explicit CLI overrides once, before auditing any fields.
pub(super) fn read(root: &Path, values: &[String]) -> Result<BTreeMap<String, String>, String> {
    let settings = crate::generators::model_paths::settings(root)?;
    let mut result = BTreeMap::new();
    for (rust, sql) in settings.type_mappings {
        let (rust, sql) = mapping(&rust, &sql)?;
        if result.insert(rust.clone(), sql).is_some() {
            return Err(format!("Duplicate configured type mapping for {rust}"));
        }
    }
    let mut explicit = std::collections::BTreeSet::new();
    for value in values {
        let (rust, sql) = value
            .split_once('=')
            .ok_or("--type-map expects RUST_TYPE=POSTGRES_TYPE")?;
        let (rust, sql) = mapping(rust, sql)?;
        if !explicit.insert(rust.clone()) {
            return Err(format!("Duplicate --type-map for {rust}"));
        }
        result.insert(rust, sql);
    }
    Ok(result)
}

/// Reject statements/constraints in a type mapping; render only the parsed datatype AST.
fn mapping(rust: &str, sql: &str) -> Result<(String, String), String> {
    let ty: syn::Type = syn::parse_str(rust).map_err(|_| "Invalid Rust type in type mapping")?;
    let key = spelling(&ty);
    if key.is_empty() {
        return Err("Empty Rust type mapping".into());
    }
    // Comments, statements and control characters are never part of a storage contract.
    if sql.contains([';', '\n', '\r', '\0']) || sql.contains("--") || sql.contains("/*") {
        return Err("Type mapping must contain only one PostgreSQL datatype".into());
    }
    let dialect = PostgreSqlDialect {};
    let mut parser = Parser::new(&dialect)
        .try_with_sql(sql)
        .map_err(|_| "Invalid PostgreSQL type mapping")?;
    let datatype = parser
        .parse_data_type()
        .map_err(|_| "Invalid PostgreSQL type mapping")?;
    if parser.peek_token().token != Token::EOF {
        return Err("Type mapping cannot contain column constraints or SQL statements".into());
    }
    Ok((key, datatype.to_string()))
}

#[cfg(test)]
mod tests {
    use super::mapping;

    #[test]
    fn canonicalizes_syntax_without_changing_literals() {
        assert_eq!(
            mapping("pkg :: Value", "jsonb"),
            mapping("pkg::Value", "JSONB")
        );
        assert_ne!(
            mapping("Wrapper<{\"a b\"}>", "text").unwrap().0,
            mapping("Wrapper<{\"ab\"}>", "text").unwrap().0
        );
        let normalized = super::super::rust_types::normalized;
        assert_eq!(
            normalized("geometry(Polygon,4326)"),
            normalized("geometry(Polygon, 4326)")
        );
        assert_ne!(normalized("numeric(12,2)"), normalized("numeric(12,3)"));
        assert_ne!(
            normalized("public.\"State\""),
            normalized("public.\"state\"")
        );
        assert_eq!(normalized("int4"), normalized("integer"));
    }

    #[test]
    fn accepts_storage_types_not_sql_fragments() {
        for sql in [
            "geometry(Polygon,4326)",
            "public.order_state",
            "numeric(12,2)",
            "jsonb",
            "text[]",
        ] {
            assert!(mapping("DbPolygon", sql).is_ok(), "{sql}");
        }
        for sql in [
            "",
            "text; DROP TABLE users",
            "text NOT NULL",
            "text DEFAULT 'x'",
            "text -- comment",
            "text /*comment*/",
        ] {
            assert!(mapping("DbPolygon", sql).is_err(), "{sql}");
        }
    }
}
