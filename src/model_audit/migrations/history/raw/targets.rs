//! Recognize executable SQL bodies and table targets without interpreting string data.
use sqlparser::tokenizer::Token;

/// Match only unquoted SQL keywords, not identifiers such as `"TABLE"`.
pub(super) fn keyword(token: &Token, value: &str) -> bool {
    matches!(token, Token::Word(word) if word.quote_style.is_none()
        && word.value.eq_ignore_ascii_case(value))
}

/// Distinguish executable DO/EXECUTE bodies from stored SQL-looking strings.
pub(super) fn executable(tokens: &[Token], index: usize) -> bool {
    (index > 0
        && ["DO", "EXECUTE"]
            .iter()
            .any(|word| keyword(&tokens[index - 1], word)))
        || (index >= 3
            && keyword(&tokens[index - 3], "DO")
            && keyword(&tokens[index - 2], "LANGUAGE"))
        || (index >= 3
            && keyword(&tokens[index - 3], "EXECUTE")
            && keyword(&tokens[index - 2], "FORMAT")
            && matches!(tokens[index - 1], Token::LParen))
}

/// Read an optionally schema-qualified target after TABLE and its modifiers.
pub(super) fn matches(tokens: &[Token], schema: &str, table: &str) -> bool {
    let tokens = tokens
        .iter()
        .skip_while(|token| {
            ["IF", "NOT", "EXISTS", "ONLY"]
                .iter()
                .any(|word| keyword(token, word))
        })
        .collect::<Vec<_>>();
    let Some(Token::Word(first)) = tokens.first().copied() else {
        return false;
    };
    let mut name = first.to_string();
    if matches!(tokens.get(1), Some(Token::Period)) {
        let Some(Token::Word(second)) = tokens.get(2).copied() else {
            return false;
        };
        name = format!("{name}.{second}");
    }
    super::super::sql::same_table(&name, schema, table)
}
