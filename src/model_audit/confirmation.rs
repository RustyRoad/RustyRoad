//! Bulk consent: terminals prompt; automation must explicitly request generation.
use super::types::Report;
use clap::ArgMatches;
use std::io::IsTerminal;

#[derive(Clone, Copy)]
pub(super) enum Scope {
    Both,
    Existing,
    New,
}

impl Scope {
    pub(super) fn includes(self, is_new: bool) -> bool {
        matches!(self, Self::Both)
            || matches!((self, is_new), (Self::New, true) | (Self::Existing, false))
    }
}

/// `--generate-migrations` is consent; JSON/pipes are report-only by default.
pub(super) fn request(
    report: &Report,
    args: &ArgMatches,
    format: &str,
) -> Result<Option<Scope>, String> {
    if args.get_flag("generate-migrations") {
        return Ok(Some(
            match args
                .get_one::<String>("migration-scope")
                .map(String::as_str)
            {
                Some("existing") => Scope::Existing,
                Some("new") => Scope::New,
                _ => Scope::Both,
            },
        ));
    }
    if args.get_flag("report-only")
        || format == "json"
        || report.catalog.is_none()
        || !std::io::stdin().is_terminal()
        || !std::io::stdout().is_terminal()
        || !std::io::stderr().is_terminal()
    {
        return Ok(None);
    }
    eprintln!(
        "{}",
        report.summary.text(report.discovered, report.selected)
    );
    for candidate in &report.candidates {
        if let Some(audit) = &candidate.table_audit {
            eprintln!(
                "  {} -> {} [{}]",
                candidate.model.id,
                audit.table.as_deref().unwrap_or("unresolved"),
                audit.status
            );
            for blocker in &audit.blockers {
                eprintln!("    BLOCKED: {blocker}");
            }
        }
    }
    if !report.diagnostics.is_empty() {
        eprintln!("Audit is incomplete: eligible drafts will retain warnings; review discovery diagnostics before applying SQL.");
    }
    if report.summary.update_migrations + report.summary.create_migrations == 0 {
        return Ok(None);
    }
    let choice = dialoguer::Select::new()
        .with_prompt(
            "Generate eligible migration files? (blocked tables skipped; never applies SQL)",
        )
        .items([
            "No — report only",
            "Both — update existing and create missing tables",
            "Existing tables only",
            "Missing tables only",
        ])
        .default(0)
        .interact_opt()
        .map_err(|e| e.to_string())?;
    Ok(match choice {
        Some(1) => Some(Scope::Both),
        Some(2) => Some(Scope::Existing),
        Some(3) => Some(Scope::New),
        _ => None,
    })
}
