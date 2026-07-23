use super::patterns::classify;
use super::BreakingChangeFinding;
use std::path::Path;

pub(super) fn detect(path: &Path, sql: &str) -> Vec<BreakingChangeFinding> {
    let mut findings = Vec::new();
    for (index, line) in sql.lines().enumerate() {
        for kind in classify(line) {
            findings.push(BreakingChangeFinding {
                path: path.to_path_buf(),
                line: index + 1,
                kind,
                sql: line.trim().to_string(),
            });
        }
    }
    let active_sql = sql
        .lines()
        .filter(|line| !line.trim_start().starts_with("--"))
        .collect::<Vec<_>>()
        .join(" ");
    for kind in classify(&active_sql) {
        if !findings.iter().any(|finding| finding.kind == kind) {
            findings.push(BreakingChangeFinding {
                path: path.to_path_buf(),
                line: 1,
                kind,
                sql: active_sql.clone(),
            });
        }
    }
    findings
}
