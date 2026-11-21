//! Read-only workflow orchestration; output writing is a separate explicit step.

use super::{cli::values, context, discover, parse, plan, schema, select, types::Report};
use clap::ArgMatches;

/// Construct a plan from detected settings; --offline forbids catalog connections.
pub(super) fn audit(args: &ArgMatches) -> Result<Report, String> {
    let root = context::root(args)?;
    let context = context::resolve(&root, args)?;
    let catalog_requested = ["database", "snapshot"].contains(&context.catalog_mode.as_str());
    let generated = context.generated.clone();
    let db = if let Some(path) = &generated {
        let source = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        parse::parse(path, &root, &source)?
    } else {
        Vec::new()
    };
    if db.is_empty() && !catalog_requested {
        return Err("Generated file contains no supported declarations".into());
    }
    let paths = context.sources.clone();
    let (models, diagnostics, source_files) =
        discover::inventory(&root, &paths, generated.as_deref());
    let discovered = models.len();
    let mappings = plan::mappings(&values(args, "map"), &models, &db)?;
    let mut candidates: Vec<_> = models
        .iter()
        .map(|m| plan::candidate(m.clone(), &db, &mappings))
        .collect();
    let catalog = schema::attach(args, &root, &context, &models, &mut candidates)?;
    let declaration_coverage = super::coverage::inventory(&candidates);
    let candidates = select::apply(candidates, args)?;
    let mut report = Report {
        summary: super::summary::Summary::from_candidates(&candidates),
        version: 5, read_only: true, root, context, generated, diagnostics, discovered, catalog, migration_files: vec![],
        selected: candidates.len(), candidates, source_files, declaration_coverage,
        analysis: "ast".into(),
        limitations: vec![
            "Declarations are read from files on disk; generated inventory may be stale or filtered. No Cargo, rustc, or language-server subprocess is invoked.".into(),
            "Name mapping is a hint. Field types compare token spelling, not resolved Rust type identity.".into(),
            "Discovery scans struct/enum/type declarations in files and inline modules; macro-produced and function-local declarations are not expanded.".into(),
            "Runtime SQL findings are advisory lexical matches of explicit SQLx query paths, even in malformed sources. Aliases, query builders, other clients and generated code are not covered; macros and name resolution are not evaluated. Findings do not establish table ownership or clear parse errors.".into(),
            "Caller compatibility and usage are not analyzed. Usage is unknown, not zero; this audit never proves safe deletion.".into(),
            "No source rewrite or deletion is authorized by this plan. Review methods, traits and API contracts separately.".into(),
        ],
    };
    if report.catalog.is_some() {
        report.limitations.push("Table comparison uses the recorded catalog source; snapshots are not live-schema proof. Unmatched structs remain unresolved until an explicit --table mapping establishes persistence ownership. Names alone never authorize new tables. Keys/defaults are not inferred. Generation never applies SQL.".into());
    }
    for candidate in &mut report.candidates {
        candidate.blockers.push(
            "Caller compatibility and usage are unverified; do not delete or replace this type based on schema evidence".into(),
        );
    }
    Ok(report)
}
