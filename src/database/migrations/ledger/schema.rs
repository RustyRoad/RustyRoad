//! Ledger table definition and creation.

use super::exec;
use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;

/// Name of the table used to track migration bookkeeping.
pub const LEDGER_TABLE: &str = "_rustyroad_migrations";

/// Column definitions shared by every backend.
const COLUMNS: &str = "name {name_type} NOT NULL,
     applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
     direction {direction_type} NOT NULL,
     provenance VARCHAR(16) NOT NULL DEFAULT 'legacy',
     checksum VARCHAR(64) NULL,
     verified_at TIMESTAMP NULL,
     UNIQUE (name, direction)";

/// Returns the `(primary key, name type, direction type)` triple for `connection`.
fn column_types(connection: &DatabaseConnection) -> (&'static str, &'static str, &'static str) {
    match connection {
        DatabaseConnection::Pg(_) => ("id SERIAL PRIMARY KEY", "VARCHAR(255)", "VARCHAR(10)"),
        DatabaseConnection::MySql(_) => (
            "id INT AUTO_INCREMENT PRIMARY KEY",
            "VARCHAR(255)",
            "VARCHAR(10)",
        ),
        DatabaseConnection::Sqlite(_) => ("id INTEGER PRIMARY KEY AUTOINCREMENT", "TEXT", "TEXT"),
    }
}

/// Creates the ledger table when it does not already exist, then ensures the
/// uniqueness constraint that makes the table authoritative.
pub async fn ensure_table(connection: &DatabaseConnection) -> Result<(), CustomMigrationError> {
    let (primary_key, name_type, direction_type) = column_types(connection);
    let columns = COLUMNS
        .replace("{name_type}", name_type)
        .replace("{direction_type}", direction_type);
    let sql = format!("CREATE TABLE IF NOT EXISTS {LEDGER_TABLE} ({primary_key}, {columns})");

    exec::execute(connection, &sql, &[]).await?;
    add_metadata_columns(connection).await?;
    add_unique_index(connection).await;

    Ok(())
}

/// Adds metadata to ledgers created by versions before provenance existed.
///
/// Column discovery keeps this idempotent without hiding real `ALTER TABLE` failures.
async fn add_metadata_columns(connection: &DatabaseConnection) -> Result<(), CustomMigrationError> {
    let definitions = [
        "provenance VARCHAR(16) NOT NULL DEFAULT 'legacy'",
        "checksum VARCHAR(64) NULL",
        "verified_at TIMESTAMP NULL",
    ];
    for definition in definitions {
        let column = definition
            .split_whitespace()
            .next()
            .expect("metadata definition should start with a column name");
        if !column_exists(connection, column).await? {
            let sql = format!("ALTER TABLE {LEDGER_TABLE} ADD COLUMN {definition}");
            exec::execute(connection, &sql, &[]).await?;
        }
    }
    Ok(())
}

async fn column_exists(
    connection: &DatabaseConnection,
    column: &str,
) -> Result<bool, CustomMigrationError> {
    let sql = match connection {
        DatabaseConnection::Pg(_) => {
            "SELECT 1 FROM information_schema.columns \
             WHERE table_schema = current_schema() AND table_name = '_rustyroad_migrations' \
             AND column_name = $1 LIMIT 1"
        }
        DatabaseConnection::MySql(_) => {
            "SELECT 1 FROM information_schema.columns \
             WHERE table_schema = DATABASE() AND table_name = '_rustyroad_migrations' \
             AND column_name = ? LIMIT 1"
        }
        DatabaseConnection::Sqlite(_) => {
            "SELECT 1 FROM pragma_table_info('_rustyroad_migrations') WHERE name = ? LIMIT 1"
        }
    };
    exec::exists(connection, sql, &[column]).await
}

/// Adds the uniqueness constraint to a ledger created before it existed.
///
/// Best effort: this fails when the table still holds duplicate `(name, direction)`
/// rows, which is exactly the state a legacy insert-only ledger is in. The skip gate
/// does not depend on the constraint, so a failure is reported and tolerated rather
/// than aborting the migration run.
async fn add_unique_index(connection: &DatabaseConnection) {
    // MySQL has no `CREATE UNIQUE INDEX IF NOT EXISTS`; a duplicate-name error there
    // simply means the index is already present.
    let guard = match connection {
        DatabaseConnection::MySql(_) => "",
        _ => "IF NOT EXISTS ",
    };
    let sql = format!(
        "CREATE UNIQUE INDEX {guard}_rustyroad_migrations_name_direction_key \
         ON {LEDGER_TABLE} (name, direction)"
    );

    if let Err(error) = exec::execute(connection, &sql, &[]).await {
        eprintln!(
            "Note: could not enforce UNIQUE (name, direction) on {LEDGER_TABLE}. \
             The idempotency gate still works. Deduplicate the table to make it \
             authoritative. Details: {error}"
        );
    }
}
