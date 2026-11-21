//! Flag SQLx query builders at construction, before later push/bind composition.
use super::super::lexer::Token;

/// Recognize `QueryBuilder::new(sql)` with an optional turbofish type argument.
/// Example: `sqlx::QueryBuilder::<Postgres>::new("SELECT id FROM rows")`.
pub(super) fn query(tokens: &[Token<'_>], mut at: usize) -> Option<(&'static str, &'static str)> {
    let text = |index: usize| tokens.get(index).map(|token| token.text);
    if text(at) == Some(":") && text(at + 1) == Some(":") && text(at + 2) == Some("<") {
        at += 2;
        let mut depth = 0;
        while let Some(value) = text(at) {
            if value == "<" {
                depth += 1;
            }
            if value == ">" && text(at.saturating_sub(1)) != Some("-") {
                depth -= 1;
            }
            at += 1;
            if depth == 0 {
                break;
            }
        }
        if depth != 0 {
            return None;
        }
    }
    if text(at) != Some(":")
        || text(at + 1) != Some(":")
        || text(at + 2) != Some("new")
        || text(at + 3) != Some("(")
    {
        return None;
    }
    at += 4;
    let kind = if tokens.get(at).is_some_and(|token| token.literal)
        && matches!(text(at + 1), Some("," | ")"))
    {
        "literal"
    } else {
        "dynamic_or_indirect"
    };
    Some(("QueryBuilder::new", kind))
}
