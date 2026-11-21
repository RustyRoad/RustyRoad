//! Migration subcommand definitions and dispatch.

mod apply;
mod baseline;
mod breaking_change;
mod convert;
mod generate;
mod inspect;
mod repair;
mod repair_down;
mod rollback;
mod route;
mod validate_command;
mod version;

use clap::ArgMatches;

pub(crate) fn validate_command() -> clap::Command {
    validate_command::build()
}

pub(crate) fn migration_command() -> clap::Command {
    clap::Command::new("migration").arg(breaking_change::allow_breaking_arg())
}

/// Returns lifecycle, baseline, and repair subcommands.
pub(crate) fn lifecycle_commands() -> Vec<clap::Command> {
    vec![
        version::start_command(),
        version::complete_command(),
        version::rollback_command(),
        version::status_command(),
        baseline::baseline_command(),
        repair::command(),
        repair_down::command(),
    ]
}

pub async fn dispatch(matches: &ArgMatches, format: &str) {
    if !matches!(matches.subcommand_name(), Some("convert" | "repair-down")) {
        super::warn_about_rogue_migrations();
    }
    match matches.subcommand() {
        Some((name, args)) => route::run(name, args, format).await,
        None => println!("Invalid migration choice"),
    }
}

/// Sends configuration diagnostics to stderr, preserving machine-readable stdout.
///
/// # Examples
///
/// ```sh
/// rustyroad --format json migration list > migration-ledger.json
/// jq '.migrations | length' migration-ledger.json
/// ```
pub(super) fn print_config() {
    eprintln!("Config file: {}", crate::database::get_config_file_name());
    eprintln!("Environment: {}", crate::database::get_environment());
}
