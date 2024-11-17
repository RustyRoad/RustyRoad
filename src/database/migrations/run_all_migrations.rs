use std::{fs, io};
use crate::database::migrations::{run_migration, MigrationDirection, CustomMigrationError};

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
    
    // Get all migration directories
    let mut migration_dirs: Vec<_> = fs::read_dir(migrations_dir_path)
        .map_err(|e| CustomMigrationError::IoError(e))?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .collect();
    
    // Sort directories by name (which includes timestamp)
    migration_dirs.sort_by_key(|dir| dir.file_name());
    
    // If we're migrating down, reverse the order
    if direction == MigrationDirection::Down {
        migration_dirs.reverse();
    }
    
    // Run each migration in order
    for dir in migration_dirs {
        let dir_name = dir.file_name();
        let migration_name = dir_name
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid directory name"))
            .map_err(|e| CustomMigrationError::IoError(e))?
            .split('-')
            .nth(1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid migration name format"))
            .map_err(|e| CustomMigrationError::IoError(e))?
            .to_string();
            
        println!("Running migration: {}", migration_name);
        run_migration(migration_name, direction).await?;
    }
    
    Ok(())
}
