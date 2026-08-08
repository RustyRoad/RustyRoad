//! Routing one migration subcommand to its handler.

use super::{apply, baseline, convert, generate, inspect, repair, rollback, version};
use clap::ArgMatches;

/// Routes one migration subcommand.
pub(super) async fn run(name: &str, args: &ArgMatches, format: &str) {
    match name {
        "generate" => generate::run(args).await,
        "all" => apply::all(args).await,
        "run" => apply::one(args).await,
        "baseline" => baseline::run(args).await,
        "repair-ledger" => repair::run(args).await,
        "start" => version::start(args).await,
        "complete" => version::complete(args).await,
        "rollback-version" => version::rollback(args).await,
        "version-status" => version::status(args).await,
        "rollback" => rollback::one(args).await,
        "redo" => rollback::redo(args).await,
        "reset" => rollback::reset().await,
        "validate" => inspect::validate().await,
        "list" => inspect::list(format).await,
        "convert" => convert::run(args),
        _ => println!("Invalid migration choice"),
    }
}
