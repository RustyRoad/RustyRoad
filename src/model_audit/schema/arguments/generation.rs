//! Explicit consent and scope options for writing reviewable migration files.
use clap::Arg;

/// `--generate-migrations --dry-run` previews without writing SQL.
pub(super) fn args() -> Vec<Arg> {
    vec![
        Arg::new("generate-migrations")
            .long("generate-migrations")
            .action(clap::ArgAction::SetTrue)
            .conflicts_with("report-only")
            .help("Generate reviewable migration files without applying SQL"),
        Arg::new("migration-scope")
            .long("migration-scope")
            .value_parser(["both", "existing", "new"])
            .requires("generate-migrations")
            .help("Generate for both groups, existing tables, or missing tables (default: both)"),
        Arg::new("dry-run")
            .long("dry-run")
            .action(clap::ArgAction::SetTrue)
            .requires("generate-migrations")
            .help("Preview migration generation without creating files"),
        Arg::new("report-only")
            .long("report-only")
            .action(clap::ArgAction::SetTrue)
            .help("Report only; never prompt for migration generation"),
    ]
}
