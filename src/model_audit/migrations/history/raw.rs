//! Inspect raw/procedural SQL without treating string data as table identifiers.

mod targets;

use sqlparser::{
    dialect::PostgreSqlDialect,
    tokenizer::{Token, Tokenizer},
};

/// Find CREATE/ALTER TABLE targets, including DDL inside DO/EXECUTE bodies.
/// For example, enum data `'menu_upsell'` does not target the table `menu`.
pub(super) fn overlaps(source: &str, schema: &str, table: &str) -> Result<bool, String> {
    let tokens: Vec<_> = Tokenizer::new(&PostgreSqlDialect {}, source)
        .tokenize()
        .map_err(|error| format!("Cannot inspect raw migration SQL: {error}"))?
        .into_iter()
        .filter(|token| !matches!(token, Token::Whitespace(_)))
        .collect();
    for (index, token) in tokens.iter().enumerate() {
        if let Token::DollarQuotedString(body) = token {
            if targets::executable(&tokens, index) && overlaps(&body.value, schema, table)? {
                return Ok(true);
            }
        }
        if let Token::SingleQuotedString(body) = token {
            // Literal SQL in EXECUTE can contain DDL; ordinary strings stay inert.
            if targets::executable(&tokens, index) && overlaps(body, schema, table)? {
                return Ok(true);
            }
        }
        if !targets::keyword(token, "TABLE") {
            continue;
        }
        let ddl = tokens[index.saturating_sub(4)..index]
            .iter()
            .rev()
            .take_while(|token| !matches!(token, Token::SemiColon))
            .any(|token| targets::keyword(token, "CREATE") || targets::keyword(token, "ALTER"));
        if ddl && targets::matches(&tokens[index + 1..], schema, table) {
            return Ok(true);
        }
    }
    Ok(false)
}
