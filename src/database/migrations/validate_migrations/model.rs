use crate::database::DatabaseType;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct MigrationValidationReport {
    pub database_type: DatabaseType,
    pub migrations_validated: usize,
}

#[derive(Debug)]
pub(super) struct MigrationFile {
    pub(super) name: String,
    pub(super) up_sql_path: PathBuf,
}
