//! Human-readable explanations and machine-readable review plans.
mod candidate;
mod context;
mod tail;

use super::types::Report;
use std::io::Write as _;

/// Emit a report; `--output` never overwrites existing files or source.
pub(super) fn emit(report: &Report, format: &str, output: Option<&String>) -> Result<(), String> {
    let rendered = if format == "json" {
        serde_json::to_string_pretty(report).map_err(|error| error.to_string())?
    } else {
        text(report)
    };
    if let Some(path) = output {
        let path = report.root.join(path);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| {
                format!(
                    "Cannot create {} (existing files are never overwritten): {error}",
                    path.display()
                )
            })?;
        writeln!(file, "{rendered}").map_err(|error| error.to_string())?;
    }
    let mut stdout = std::io::stdout().lock();
    writeln!(stdout, "{rendered}").map_err(|error| error.to_string())
}

/// Compose context, declaration evidence, and file-generation results separately.
fn text(report: &Report) -> String {
    let mut out = context::render(report);
    out.push_str(&super::coverage::text(report));
    candidate::append(&mut out, report);
    tail::append(&mut out, report);
    out
}
