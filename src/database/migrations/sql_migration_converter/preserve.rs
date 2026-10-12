//! Keep executable cleanup from an existing `down.sql` that regeneration lacks.

use super::alter::split_alter;
use super::split::split_outside;
use regex::Regex;

fn strip_comments(sql: &str) -> String {
    let block = Regex::new(r"(?s)/\*.*?\*/").unwrap();
    let line = Regex::new(r"--[^\n]*").unwrap();
    line.replace_all(&block.replace_all(sql, ""), "")
        .to_string()
}

fn norm(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Per-action normalized keys, so multi-action ALTERs compare to one-per-column.
fn keys(stmt: &str) -> Vec<String> {
    match split_alter(stmt) {
        Some((t, actions)) => actions
            .iter()
            .map(|a| norm(&format!("alter table {t} {a}")))
            .collect(),
        None => vec![norm(stmt)],
    }
}

/// Known generator defects that must not be carried forward.
fn defective(key: &str) -> bool {
    key.ends_with("drop column if exists if") || key.starts_with("alter table public ")
}

/// Append existing executable statements missing from `generated`.
pub(super) fn merge_existing_down(generated: String, existing: &str) -> String {
    let have: Vec<String> = split_outside(&strip_comments(&generated), ';')
        .iter()
        .flat_map(|s| keys(s))
        .collect();
    let mut extra = Vec::new();
    for stmt in split_outside(&strip_comments(existing), ';') {
        let ks = keys(&stmt);
        if ks.iter().all(|k| have.contains(k) || defective(k)) {
            continue;
        }
        extra.push(format!("{};", stmt.trim()));
    }
    if extra.is_empty() {
        return generated;
    }
    format!(
        "{generated}\n\n-- Preserved from existing down.sql:\n{}\n",
        extra.join("\n")
    )
}
