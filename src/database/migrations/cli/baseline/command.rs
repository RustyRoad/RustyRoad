//! Clap definition for `migration baseline`.

use clap::{Arg, ArgAction, Command};

/// Returns the `baseline` subcommand.
pub(crate) fn baseline() -> Command {
    Command::new("baseline")
        .alias("adopt")
        .about("Record existing migrations as baselined without running them")
        .long_about(
            "Records every migration on disk as baselined, without executing any SQL.\n\n\
             Use this when a database's schema is already current but its ledger\n\
             does not say so. Replaying that history would fail against columns\n\
             that later migrations already superseded.\n\n\
             This is ledger bookkeeping only. It does not prove the SQL ran or that\n\
             its schema/data effects exist. Baseline rows store provenance and a\n\
             SHA-256 source checksum while effects remain unverified.\n\n\
             WARNING:\n\
              You assert that the database already reflects every migration. If that\n\
              is not true, those migrations will never run. Verify with\n\
              'rustyroad db schema' first.\n\n\
             CONFIG:\n\
              Database connection from ./rustyroad.toml (or ./rustyroad.<ENVIRONMENT>.toml).\n\n\
             EXAMPLE:\n\
              rustyroad migration baseline\n\
              ENVIRONMENT=test rustyroad migration baseline --yes\n",
        )
        .arg(
            Arg::new("yes")
                .long("yes")
                .short('y')
                .action(ArgAction::SetTrue)
                .help("Skip the confirmation prompt"),
        )
        .after_help("Example:\n  rustyroad migration baseline --yes\n")
}
