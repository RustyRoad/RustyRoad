//! Canonical comparison without losing quoted identifiers or type modifiers.
use sqlparser::{dialect::PostgreSqlDialect, parser::Parser, tokenizer::Token};

/// Compare aliases such as `int8` and `bigint`, retaining `numeric(10,2)`.
pub(in crate::model_audit::schema) fn normalized(sql: &str) -> String {
    let mut parser = match Parser::new(&PostgreSqlDialect {}).try_with_sql(sql) {
        Ok(parser) => parser,
        Err(_) => return sql.trim().to_owned(),
    };
    let Ok(ty) = parser.parse_data_type() else {
        return sql.trim().to_owned();
    };
    if parser.peek_token().token != Token::EOF {
        return sql.trim().to_owned();
    }
    let canonical = ty.to_string();
    if canonical.contains('"') {
        return canonical;
    }
    match canonical.to_ascii_lowercase().as_str() {
        "int2" | "smallint" => "smallint".into(),
        "int" | "int4" | "integer" => "integer".into(),
        "int8" | "bigint" => "bigint".into(),
        "float4" | "real" => "real".into(),
        "float8" | "double precision" => "double precision".into(),
        "bool" | "boolean" => "boolean".into(),
        "timestamptz" | "timestamp with time zone" => "timestamp with time zone".into(),
        "timestamp" | "timestamp without time zone" => "timestamp without time zone".into(),
        "time" | "time without time zone" => "time without time zone".into(),
        _ => canonical.to_ascii_lowercase(),
    }
}
