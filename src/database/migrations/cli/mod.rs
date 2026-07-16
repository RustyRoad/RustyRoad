mod apply;
mod convert;
mod generate;
mod inspect;
mod rollback;
mod validate_command;

use clap::ArgMatches;

pub(crate) fn validate_command() -> clap::Command {
    validate_command::build()
}

pub async fn dispatch(matches: &ArgMatches, format: &str) {
    if matches.subcommand_name() != Some("convert") {
        super::warn_about_rogue_migrations();
    }
    match matches.subcommand() {
        Some(("generate", args)) => generate::run(args).await,
        Some(("all", _)) => apply::all().await,
        Some(("run", args)) => apply::one(args).await,
        Some(("rollback", args)) => rollback::one(args).await,
        Some(("redo", args)) => rollback::redo(args).await,
        Some(("reset", _)) => rollback::reset().await,
        Some(("validate", _)) => inspect::validate().await,
        Some(("list", _)) => inspect::list(format).await,
        Some(("convert", args)) => convert::run(args),
        _ => println!("Invalid migration choice"),
    }
}

pub(super) fn print_config() {
    println!("Config file: {}", crate::database::get_config_file_name());
    println!("Environment: {}", crate::database::get_environment());
}
