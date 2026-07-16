use super::error::MigrationValidationError;
use super::identity::Identity;
use super::model::MigrationFile;
use super::{mysql, postgres, sqlite};
use crate::database::{Database, DatabaseType};

pub(super) async fn validate(
    database: &Database,
    identity: &Identity,
    migrations: &[MigrationFile],
) -> Result<(), MigrationValidationError> {
    match &database.database_type {
        DatabaseType::Postgres => postgres::validate(database, identity, migrations).await,
        DatabaseType::Mysql => mysql::validate(database, identity, migrations).await,
        DatabaseType::Sqlite => sqlite::validate(identity, migrations).await,
        DatabaseType::Mongo => Err(MigrationValidationError::new(
            "Migration validation does not support MongoDB",
        )),
    }
}
