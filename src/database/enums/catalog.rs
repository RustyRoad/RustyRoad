//! Backend dispatch for enum inspection.

use super::model::EnumOutput;
use super::{mysql, postgres};
use crate::database::migrations::CustomMigrationError;
use crate::database::{get_config_file_name, Database, DatabaseConnection};

/// Reads the user-defined enum types of the database configured in
/// `rustyroad.toml`. SQLite has no enum type and yields an empty list.
pub async fn read_enums() -> Result<EnumOutput, CustomMigrationError> {
    let database = Database::get_database_from_rustyroad_toml()
        .expect("Couldn't parse the rustyroad.toml file");

    let config_file = get_config_file_name();

    let connection = Database::create_database_connection(&database)
        .await
        .map_err(CustomMigrationError::SendError)?;

    let database_type = database.database_type.to_string().to_ascii_lowercase();

    let enums = match connection {
        DatabaseConnection::Pg(conn) => postgres::postgres_enums(&conn).await?,
        DatabaseConnection::MySql(conn) => mysql::mysql_enums(&conn).await?,
        DatabaseConnection::Sqlite(_) => Vec::new(),
    };

    Ok(EnumOutput {
        database_type,
        config_file,
        enums,
    })
}
