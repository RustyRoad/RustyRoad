//! Resolve explicitly imported SQLx call names without requiring a parsed file.
mod imports;
use super::lexer::Token;
use std::collections::{HashMap, HashSet};

/// Advisory import bindings; nested imports conservatively cover the whole file.
/// Example: `use sqlx::query_as as rows;` binds `rows` to `query_as`.
#[derive(Default)]
pub(super) struct Bindings {
    modules: HashSet<String>,
    functions: HashMap<String, String>,
    glob: bool,
}

/// Resolve qualified, renamed and glob-imported calls to their SQLx API names.
/// Example: `use sqlx as db; db::query(sql)` resolves to `query`.
impl Bindings {
    /// Returns the API name and first token after it; ignores function declarations.
    /// Example: `rows::<_, Row>(sql)` leaves the cursor before `::`.
    pub(super) fn resolve<'a>(
        &'a self,
        tokens: &[Token<'a>],
        index: usize,
    ) -> Option<(&'a str, usize)> {
        let token = tokens.get(index)?;
        if token.literal || index > 0 && tokens[index - 1].text == "fn" {
            return None;
        }
        let text = |at: usize| tokens.get(at).map(|token| token.text);
        if self.modules.contains(token.text)
            && text(index + 1) == Some(":")
            && text(index + 2) == Some(":")
        {
            return Some((text(index + 3)?, index + 4));
        }
        if index > 0 && matches!(tokens[index - 1].text, ":" | ".") {
            return None;
        }
        self.functions
            .get(token.text)
            .map(|api| (api.as_str(), index + 1))
            .or_else(|| self.glob.then_some((token.text, index + 1)))
    }
}

/// Parse individual use statements so malformed surrounding Rust cannot hide SQL.
/// Example: a missing function brace still permits `use sqlx::query_as;` detection.
pub(super) fn collect(source: &str, tokens: &[Token<'_>]) -> Bindings {
    let mut bindings = Bindings::default();
    bindings.modules.insert("sqlx".into());
    let mut uses = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if token.literal || token.text != "use" {
            continue;
        }
        let Some(end) = tokens[index..].iter().find(|token| token.text == ";") else {
            continue;
        };
        if let Ok(item) = syn::parse_str::<syn::ItemUse>(&source[token.offset..end.offset + 1]) {
            uses.push(item);
        }
    }
    // Revisit imports to resolve aliases imported through another local binding.
    for _ in 0..=uses.len() {
        for item in &uses {
            imports::visit(&item.tree, &[], &mut bindings);
        }
    }
    bindings
}
