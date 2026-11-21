//! Treat catalog rollback types as data, not executable SQL fragments.
use sqlparser::{dialect::PostgreSqlDialect, parser::Parser, tokenizer::Token};

/// Accept one datatype only; reject statements, comments and column constraints.
pub(super) fn check(sql: &str) -> Result<(), String> {
    if sql.contains([';', '\n', '\r', '\0']) || sql.contains("--") || sql.contains("/*") {
        return Err("expected one PostgreSQL datatype".into());
    }
    let mut parser = Parser::new(&PostgreSqlDialect {})
        .try_with_sql(sql)
        .map_err(|error| error.to_string())?;
    parser
        .parse_data_type()
        .map_err(|error| error.to_string())?;
    if parser.peek_token().token != Token::EOF {
        return Err("datatype contains trailing SQL".into());
    }
    Ok(())
}
