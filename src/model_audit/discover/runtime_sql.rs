//! Advisory SQLx call detection independent of Rust parsing and row ownership.
mod bindings;
mod calls;
mod lexer;
mod literals;
mod trivia;

use serde::Serialize;

/// A call-site warning, not proof of a table contract or permission to rewrite SQL.
/// Locations are one-based lines and Unicode-character columns in the source file.
#[derive(Debug, Serialize)]
pub(in crate::model_audit) struct Finding {
    pub line: usize,
    pub character: usize,
    pub query_api: String,
    pub argument_kind: &'static str,
    pub remediation: &'static str,
}

/// Retain SQLx calls even in files with unbalanced braces or no declarations.
/// Example: `scan("sqlx::query(sql)")` reports a dynamic argument, not a table.
pub(super) fn scan(source: &str) -> Vec<Finding> {
    let tokens = lexer::tokens(source);
    let bindings = bindings::collect(source, &tokens);
    let mut findings = Vec::new();
    let mut previous_offset = 0;
    let mut line = 1;
    let mut character = 1;
    for index in 0..tokens.len() {
        let Some((api, argument_kind)) = calls::query(&tokens, index, &bindings) else {
            continue;
        };
        let offset = tokens[index].offset;
        // Tokens are source-ordered; visit each prefix character at most once.
        // Character columns intentionally count Unicode scalars, not UTF-8 bytes.
        for ch in source[previous_offset..offset].chars() {
            if ch == '\n' {
                line += 1;
                character = 1;
            } else {
                character += 1;
            }
        }
        previous_offset = offset;
        findings.push(Finding {
            line,
            character,
            query_api: format!("sqlx::{api}"),
            argument_kind,
            remediation: "Inspect the SQL, callers and catalog; replace runtime SQL with \
                canonical RustyRoad/Drizzle generated artifacts. Preserve parameters, \
                nullability, ordering and transaction semantics. If generation cannot \
                express the operation, report the missing capability; do not guess a \
                schema or hand-write a replacement. Repair parse errors without deleting \
                declarations or hiding diagnostics; they remain separate blockers. Do not \
                relabel persisted rows or apply migrations to clear these findings.",
        });
    }
    findings
}
