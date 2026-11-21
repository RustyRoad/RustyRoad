//! Small tolerant lexer: strings/comments cannot masquerade as executable calls.
use super::{literals, trivia};

/// Source-backed token; offset is a UTF-8 byte boundary for location reporting.
pub(super) struct Token<'a> {
    pub text: &'a str,
    pub offset: usize,
    pub literal: bool,
}

/// Tokenize without requiring balanced delimiters; e.g. `tokens("fn f() {")`.
pub(super) fn tokens(source: &str) -> Vec<Token<'_>> {
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < source.len() {
        if let Some(end) = trivia::end(source, offset) {
            offset = end;
            continue;
        }
        let start = offset;
        let literal = if let Some(end) = literals::end(source, offset) {
            offset = end;
            true
        } else {
            let first = source[offset..].chars().next().unwrap();
            offset += first.len_utf8();
            if first.is_alphabetic() || first == '_' {
                while let Some(next) = source[offset..].chars().next() {
                    if !next.is_alphanumeric() && next != '_' {
                        break;
                    }
                    offset += next.len_utf8();
                }
            }
            false
        };
        result.push(Token {
            text: &source[start..offset],
            offset: start,
            literal,
        });
    }
    result
}
