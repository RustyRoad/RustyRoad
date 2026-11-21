//! CLI surface for discovery, selection and read-only adoption plans.

use clap::{Arg, ArgAction, ArgMatches, Command};

/// Build `rustyroad model audit`; for example `model audit --name Customer --interactive`.
pub(crate) fn command() -> Command {
    let mut audit = Command::new("audit")
        .about("Discover handwritten models and audit their adoption into generated database types")
        .long_about("Model adoption audit with optional table-backed migration generation.\n\
            Uses syn AST declarations and PostgreSQL catalog facts; no language server or Cargo subprocess.\n\
            Never deletes models, rewrites callers or applies migrations.\n\
            Database config is auto-detected; --offline disables catalog connections.\n\
            Shows existing/missing table counts and prompts in terminals; --generate-migrations approves both groups without prompting.\n\
            Missing tables require a reviewed --table MODEL=TABLE mapping; unmatched structs are not automatically persisted rows.\n\
            Blocked table groups are skipped; unrelated eligible tables generate files. Exit 2 can include generated files.\n\
            Caller compatibility and usage are not analyzed.\n\
            Name matches are suggestions, field equality is syntactic, and no candidate is safe-to-delete.\n\
            Exit: 0 report produced, 1 invalid input/output, 2 incomplete discovery or blocked generation.")
        .arg(Arg::new("root").long("root").default_value(".").help("Workspace root; all input paths are relative to it"))
        .arg(Arg::new("generated").long("generated").help("Generated models.rs; auto-detect crates/db/src or src/db"))
        .arg(Arg::new("symbols").long("symbols").value_parser(["syntax"]).hide(true).help("Legacy compatibility option; all audits now use AST analysis"))
        .arg(Arg::new("output").long("output").help("Save report to a NEW path, relative to --root; never overwrite"))
        .arg(Arg::new("interactive").long("interactive").action(ArgAction::SetTrue).help("Multi-select candidates in a terminal"));
    for (name, help) in [
        (
            "source",
            "Source directory/file override; default generator model locations",
        ),
        (
            "name",
            "Case-insensitive name/path substring filter; repeatable (OR)",
        ),
        ("select", "Exact candidate ID from a report; repeatable"),
        (
            "map",
            "Explicit SOURCE_ID_OR_UNIQUE_NAME=GENERATED_ID_OR_UNIQUE_NAME; repeatable",
        ),
    ] {
        audit = audit.arg(
            Arg::new(name)
                .long(name)
                .action(ArgAction::Append)
                .help(help),
        );
    }
    audit = audit.arg(Arg::new("status").long("status").action(ArgAction::Append)
        .value_parser(["field_syntax_equal", "types_differ", "db_superset", "fields_differ", "unmatched", "ambiguous_mapping", "non_row_review"])
        .help("Classification filter; repeatable (OR), independent field differences remain visible"));
    Command::new("model")
        .about("Audit and plan adoption of handwritten database models")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(super::context::arguments(super::schema::arguments(audit)))
}

/// Return repeated string values; absent options become an empty list.
pub(super) fn values(matches: &ArgMatches, name: &str) -> Vec<String> {
    matches
        .get_many::<String>(name)
        .map(|v| v.cloned().collect())
        .unwrap_or_default()
}
