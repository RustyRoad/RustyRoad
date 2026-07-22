mod api;
mod backend;
mod cleanup;
mod discovery;
mod error;
mod execute_mysql;
mod execute_postgres;
mod execute_sqlite;
mod guard;
mod identity;
mod migration_directory;
mod model;
mod mysql;
mod mysql_create;
mod mysql_drop;
mod mysql_pool;
mod postgres;
mod postgres_create;
mod postgres_drop;
mod postgres_pool;
mod sql;
mod sqlite;

#[cfg(test)]
mod discovery_test;
#[cfg(test)]
mod guard_test;
#[cfg(test)]
mod sqlite_test;
#[cfg(test)]
mod test_support;

pub use api::validate_migrations;
pub use error::MigrationValidationError;
pub use model::MigrationValidationReport;
