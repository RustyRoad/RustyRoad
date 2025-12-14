use crate::database::migrations::{run_migration, CustomMigrationError, MigrationDirection};
use std::{fs, io, path::Path};

/// # Name: run_all_migrations
/// ## Description: Runs all migrations in the migrations directory in order of creation
/// ### Arguments:
/// * `direction` - The direction to run the migrations (up or down)
/// ### Returns:
/// * `Result<(), CustomMigrationError>` - Returns Ok(()) if successful, or a CustomMigrationError if there was an error
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::run_all_migrations;
/// use rustyroad::database::MigrationDirection;
///
/// let result = run_all_migrations(MigrationDirection::Up).await;
/// assert!(result.is_ok());
/// ```
pub async fn run_all_migrations(direction: MigrationDirection) -> Result<(), CustomMigrationError> {
    let migrations_dir_path = "./config/database/migrations";

    if !Path::new(migrations_dir_path).exists() {
        let message = format!(
            "No migrations directory found at '{migrations_dir_path}'.\n\nRustyRoad expects migrations to be generated into:\n  {migrations_dir_path}/<timestamp>-<name>/up.sql\n  {migrations_dir_path}/<timestamp>-<name>/down.sql\n\nTo create your first migration:\n  rustyroad migration generate create_users_table id:serial:primary_key email:string:not_null,unique\n\nThen run them:\n  rustyroad migration all\n"
        );
        return Err(CustomMigrationError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            message,
        )));
    }

    // Get all migration directories
    let mut migration_dirs: Vec<_> = fs::read_dir(migrations_dir_path)
        .map_err(CustomMigrationError::IoError)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .collect();

    if migration_dirs.is_empty() {
        println!(
            "No migrations found in '{migrations_dir_path}'. Create one with: rustyroad migration generate <name> ..."
        );
        return Ok(());
    }

    // Sort directories by name (which includes timestamp)
    migration_dirs.sort_by_key(|dir| dir.file_name());

    // If we're migrating down, reverse the order
    if direction == MigrationDirection::Down {
        migration_dirs.reverse();
    }

    // Run each migration in order
    for dir in migration_dirs {
        let dir_name = dir.file_name();
        let dir_name = dir_name
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid directory name"))
            .map_err(CustomMigrationError::IoError)?;

        let (_, migration_name) = dir_name
            .split_once('-')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid migration directory name format. Expected: <timestamp>-<name>"))
            .map_err(CustomMigrationError::IoError)?;

        let migration_name = migration_name.to_string();

        println!("Running migration: {}", migration_name);
        run_migration(migration_name, direction).await?;
    }

    Ok(())
}
