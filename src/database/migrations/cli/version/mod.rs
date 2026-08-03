//! `rustyroad migration start|complete|rollback-version|version-status`.
//!
//! The versioned lifecycle: `start` publishes a new schema version alongside the
//! current one, `complete` retires the old version, and `rollback-version` undoes
//! a started migration that has not yet been completed.

mod command;
mod context;
mod report;
mod status;
mod undo;

use crate::database::versions::lifecycle;
use clap::ArgMatches;

pub(crate) use command::{complete as complete_command, rollback as rollback_command};
pub(crate) use command::{start as start_command, status as status_command};

/// Schema whose versions are managed.
const SCHEMA: &str = "public";

pub(super) async fn start(matches: &ArgMatches) {
    super::print_config();

    let version = matches
        .get_one::<String>("name")
        .map(String::to_string)
        .unwrap_or_default();
    let migration = context::migration(&version);
    let connection = context::connect().await;

    match lifecycle::start(&connection, SCHEMA, &version, &migration).await {
        Ok(started) => report::started(&started, lifecycle::supports_versions(&connection)),
        Err(error) => report::fail(&error.to_string()),
    }
}

pub(super) async fn complete(_matches: &ArgMatches) {
    super::print_config();
    let connection = context::connect().await;

    match lifecycle::complete(&connection, SCHEMA).await {
        Ok(version) => report::completed(&version),
        Err(error) => report::fail(&error.to_string()),
    }
}

pub(super) async fn rollback(_matches: &ArgMatches) {
    super::print_config();
    undo::run().await;
}

pub(super) async fn status(_matches: &ArgMatches) {
    super::print_config();
    let connection = context::connect().await;

    match status::collect(&connection, SCHEMA).await {
        Ok(state) => report::status(&state),
        Err(error) => report::fail(&error.to_string()),
    }
}
