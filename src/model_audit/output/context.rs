//! Render the evidence source and overall schema counts before individual models.
use crate::model_audit::types::Report;
use std::fmt::Write;

/// Keep catalog evidence distinct from AST comparison and actual file generation.
pub(super) fn render(report: &Report) -> String {
    let mut out = format!(
        "Model audit: {} discovered; {} selected; read_only={}\nAnalysis: {}\nRoot: {}\n",
        report.discovered,
        report.selected,
        report.read_only,
        report.analysis,
        report.root.display()
    );
    let _ = writeln!(out, "Context: {:?}", report.context);
    if let Some(catalog) = &report.catalog {
        let _ = writeln!(out, "Catalog: {:?}", catalog);
    }
    out.push_str(&report.summary.text(report.discovered, report.selected));
    out
}
