//! Insert statement construction for history rows.

use crate::database::versions::exec;
use crate::database::versions::history::HISTORY_TABLE;
use crate::database::DatabaseConnection;

/// Columns written on every insert.
const COLUMNS: &str = "(name, parent, done, migration_type)";

/// Builds the insert statement, with or without a parent binding.
pub(super) fn build(connection: &DatabaseConnection, done: bool, with_parent: bool) -> String {
    let done = if done { "TRUE" } else { "FALSE" };

    if with_parent {
        return exec::pick(
            connection,
            &format!("INSERT INTO {HISTORY_TABLE} {COLUMNS} VALUES ($1, $2, {done}, $3)"),
            &format!("INSERT INTO {HISTORY_TABLE} {COLUMNS} VALUES (?, ?, {done}, ?)"),
        )
        .to_string();
    }

    exec::pick(
        connection,
        &format!("INSERT INTO {HISTORY_TABLE} {COLUMNS} VALUES ($1, NULL, {done}, $2)"),
        &format!("INSERT INTO {HISTORY_TABLE} {COLUMNS} VALUES (?, NULL, {done}, ?)"),
    )
    .to_string()
}
