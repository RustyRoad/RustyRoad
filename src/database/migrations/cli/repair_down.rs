//! Offline repair of missing undo files, separate from conversion and ledger repair.

mod report;
#[cfg(test)]
mod tests;

use crate::database::migrations::repair_down;
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::Path;

pub(super) fn command() -> Command {
    Command::new("repair-down")
        .about("Preview missing undo files; never rewrite existing migrations or run SQL")
        .arg(Arg::new("path").long("path")
            .default_value("./config/database/migrations")
            .help("Migration root, or a single migration directory"))
        .arg(Arg::new("reviewed-down-dir").long("reviewed-down-dir")
            .help("Reviewed <migration>/up.sql and down.sql copies; up must match exactly"))
        .arg(Arg::new("write").long("write").action(ArgAction::SetTrue)
            .help("Create only missing down.sql files after a blocker-free plan"))
        .arg(Arg::new("dry-run").long("dry-run").action(ArgAction::SetTrue)
            .conflicts_with("write")
            .help("Preview only (the default); cannot be combined with --write"))
        .after_help(concat!(
            "Examples:\n  rustyroad migration repair-down\n  ",
            "rustyroad migration repair-down --path config/database/migrations/20260101000000-example\n  ",
            "rustyroad migration repair-down --reviewed-down-dir reviewed-undo --write\n\n",
            "Preview is the default. Review undo SQL: dropping objects can destroy data.\n",
            "Conditional/unsupported SQL requires reviewed undo, not guessed reversals.\n",
            "See MIGRATION_UNDO_REPAIR.md for review layout and safety limitations.",
        ))
}

pub(super) fn run(args: &ArgMatches) {
    let path = Path::new(args.get_one::<String>("path").expect("default path"));
    let reviewed = args.get_one::<String>("reviewed-down-dir").map(Path::new);
    let write = args.get_flag("write");
    let entries = match repair_down::repair(path, reviewed, write) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("Undo repair stopped: {error}");
            std::process::exit(2);
        }
    };
    report::print(entries, write);
}
