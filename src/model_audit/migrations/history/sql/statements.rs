//! Split executable statements without interpreting SQL stored as string data.

mod table;
use crate::database::migrations::SqlOperation;
use sqlparser::{
    dialect::PostgreSqlDialect,
    tokenizer::{Token, Tokenizer},
};

/// Preserve literal escaping while selecting top-level DDL and executable bodies.
/// For example, `INSERT INTO templates VALUES ('CREATE TABLE sample(id int)')`
/// must not reserve `sample` in migration history.
pub(super) fn parse(source: &str) -> Result<Vec<SqlOperation>, String> {
    let tokens = Tokenizer::new(&PostgreSqlDialect {}, source)
        .with_unescape(false)
        .tokenize()
        .map_err(|error| format!("Cannot tokenize migration SQL: {error}"))?;
    let mut operations = Vec::new();
    for statement in tokens.split(|token| matches!(token, Token::SemiColon)) {
        let significant: Vec<_> = statement
            .iter()
            .filter(|token| !matches!(token, Token::Whitespace(_)))
            .collect();
        let Some(first) = significant.first() else {
            continue;
        };
        if !["CREATE", "ALTER", "DO", "EXECUTE"]
            .iter()
            .any(|value| keyword(first, value))
        {
            continue;
        }
        let sql = significant
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let simple_table = significant
            .get(1)
            .is_some_and(|token| keyword(token, "TABLE"))
            && (keyword(first, "CREATE") || keyword(first, "ALTER"));
        if !simple_table {
            operations.push(SqlOperation::RawSql { sql });
            continue;
        }
        operations.extend(table::parse(&sql));
    }
    Ok(operations)
}

/// Recognize keywords only outside quoted SQL identifiers and literals.
fn keyword(token: &Token, value: &str) -> bool {
    matches!(token, Token::Word(word) if word.quote_style.is_none()
        && word.value.eq_ignore_ascii_case(value))
}
