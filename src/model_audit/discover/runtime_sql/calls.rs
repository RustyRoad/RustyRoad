//! Recognize qualified SQLx query calls without interpreting their SQL contents.
mod builder;
use super::{bindings::Bindings, lexer::Token};

/// Match calls such as `sqlx::query_as::<_, Row>(sql)` or `query_as!(Row, sql)`.
pub(super) fn query<'a>(
    tokens: &[Token<'a>],
    index: usize,
    bindings: &'a Bindings,
) -> Option<(&'a str, &'static str)> {
    let text = |at: usize| tokens.get(at).map(|token| token.text);
    let (api, mut at) = bindings.resolve(tokens, index)?;
    if api == "QueryBuilder" {
        return builder::query(tokens, at);
    }
    if ![
        "query",
        "query_as",
        "query_scalar",
        "raw_sql",
        "query_with",
        "query_as_with",
        "query_scalar_with",
        "query_unchecked",
        "query_as_unchecked",
        "query_scalar_unchecked",
        "query_file",
        "query_file_as",
        "query_file_scalar",
        "query_file_unchecked",
        "query_file_as_unchecked",
        "query_file_scalar_unchecked",
    ]
    .contains(&api)
    {
        return None;
    }
    if text(at) == Some(":") && text(at + 1) == Some(":") {
        at += 2;
        if text(at) != Some("<") {
            return None;
        }
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
    let is_macro = text(at) == Some("!");
    if is_macro {
        at += 1;
    }
    if !matches!(text(at), Some("(" | "[" | "{")) {
        return None;
    }
    if !is_macro && text(at) != Some("(") {
        return None;
    }
    at += 1;
    if is_macro
        && matches!(
            api,
            "query_as" | "query_as_unchecked" | "query_file_as" | "query_file_as_unchecked"
        )
    {
        let mut depth = 0;
        while let Some(value) = text(at) {
            if value == "," && depth == 0 {
                at += 1;
                break;
            }
            match value {
                "(" | "[" | "{" | "<" => depth += 1,
                ">" if text(at.saturating_sub(1)) == Some("-") => (),
                ")" | "]" | "}" | ">" => {
                    if depth == 0 {
                        return None;
                    }
                    depth -= 1;
                }
                _ => (),
            }
            at += 1;
        }
    }
    let argument_kind = if api.starts_with("query_file") {
        "sql_file"
    } else if tokens.get(at).is_some_and(|token| token.literal)
        && matches!(text(at + 1), Some("," | ")" | "]" | "}"))
    {
        "literal"
    } else {
        "dynamic_or_indirect"
    };
    Some((api, argument_kind))
}
