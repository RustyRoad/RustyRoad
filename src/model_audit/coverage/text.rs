//! Render whole-tree accounting independently from selected migration candidates.
use crate::model_audit::types::Report;
use std::{collections::BTreeMap, fmt::Write};

/// Show file failures and declaration outcomes, e.g. a reused migration is not new SQL.
pub(in crate::model_audit) fn render(report: &Report) -> String {
    let mut files = BTreeMap::<&str, usize>::new();
    for file in &report.source_files {
        *files.entry(&file.status).or_default() += 1;
    }
    let mut outcomes = BTreeMap::<&str, usize>::new();
    for declaration in &report.declaration_coverage {
        *outcomes.entry(&declaration.outcome).or_default() += 1;
    }
    let mut output = format!(
        "Source coverage: {} files, {} declarations\n",
        report.source_files.len(),
        report.declaration_coverage.len()
    );
    for (status, count) in files {
        let _ = writeln!(output, "  files/{status}: {count}");
    }
    for (outcome, count) in outcomes {
        let _ = writeln!(output, "  declarations/{outcome}: {count}");
    }
    for file in &report.source_files {
        for finding in &file.runtime_sql {
            let _ = writeln!(
                output,
                "  runtime_sql {}:{}:{} {} ({})\n    {}",
                file.path.display(),
                finding.line,
                finding.character,
                finding.query_api,
                finding.argument_kind,
                finding.remediation
            );
        }
    }
    output
}
