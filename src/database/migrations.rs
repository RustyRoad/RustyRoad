use crate::generators::create_file;
use crate::writers::write_to_file;
use crate::Project;
use chrono::prelude::*;
use diesel::Connection;
use diesel::SqliteConnection;
use diesel_migrations::FileBasedMigrations;
use diesel_migrations::MigrationError;
use diesel_migrations::MigrationHarness;
use rustyline::DefaultEditor;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::create_dir_all;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

// Define available column types and constraints
const COLUMN_TYPES: &[&str] = &["VARCHAR(255)", "INTEGER", "TIMESTAMP"];
const CONSTRAINTS: &[&str] = &["PRIMARY KEY", "NOT NULL", "FOREIGN KEY"];

/// ## Name: create_migration
/// ### Description: Creates a migration file
/// #### Parameters:
/// - name: [`&str`] - the name of the migration
/// - Returns: [`Result<(), std::io::Error>`]
/// - if the migration was created successfully: [`Ok(())`]
/// - if there was an error creating the migration: [`Err(std::io::Error)`]
///
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::create_migration;
///
/// create_migration("create_users_table").unwrap();
/// ```
pub fn create_migration(name: &str) -> Result<String, std::io::Error> {
    // Check for database folder
    // If it doesn't exist, create it
    // Ensure that the project is a rustyroad project by looking for the rustyroad.toml file in the root directory

    let path = std::env::current_dir().unwrap();

    // Check the path for the rustyroad.toml file
    // If it doesn't exist, return an error
    // If it does exist, continue
    match std::fs::read_to_string(path.join("rustyroad.toml")) {
        Ok(_) => {}
        Err(_) => {
            println!("This is why you can't have nice things");
            return Ok("This is why you can't have nice things".to_string());
        }
    }

    match std::fs::create_dir("config/database") {
        Ok(_) => {}
        Err(_) => {}
    }

    // Check for migrations folder
    // If it doesn't exist, create it
    match std::fs::create_dir("config/database/migrations") {
        Ok(_) => {}
        Err(_) => {}
    }

    // Create directory with timestamp and name of migration
    // Then create up and down files

    let folder_name = format!(
        "config/database/migrations/{}-{}",
        Local::now().format("%Y%m%d%H%M%S"),
        name
    );

    match std::fs::create_dir(&folder_name) {
        Ok(_) => {}
        Err(_) => {
            println!("Migration already exists");
            return Ok("Migration already exists".to_string());
        }
    }

    create_file(&format!("{}/up.sql", folder_name).to_string())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    let up_file = format!("{}/up.sql", folder_name).to_string();

    let down_file = format!("{}/down.sql", folder_name).to_string();

    // Create the down.sql file
    create_file(&format!("{}/down.sql", folder_name).to_string())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    // Initialize the rustyline Editor with the default helper and in-memory history
    let mut rl = DefaultEditor::new().unwrap_or_else(|why| {
        panic!("Failed to create rustyline editor: {}", why.to_string());
    });
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    // Prompt the user for SQL queries for the up.sql file
    let mut up_sql_contents = String::new();
    let mut down_sql_contents = String::new();

    // get the name of the table
    let table_name = rl
        .readline("Enter the name of the table: ")
        .unwrap_or_else(|why| {
            panic!("Failed to read table name: {}", why.to_string());
        });

    // ask the user how many columns they want to add

    let num_columns = rl
        .readline("Enter the number of columns: ")
        .unwrap_or_else(|why| {
            panic!("Failed to read number of columns: {}", why.to_string());
        });

    // loop through the number of columns and ask the user for the column name, type, and constraints

    for _ in 0..num_columns.parse::<i32>().unwrap() {
        let column_name = rl
            .readline("Enter the name of the column: ")
            .unwrap_or_else(|why| {
                panic!("Failed to read column name: {}", why.to_string());
            });

        let column_type = rl
            .readline("Enter the type of the column: ")
            .unwrap_or_else(|why| {
                panic!("Failed to read column type: {}", why.to_string());
            });

        let column_constraints = rl
            .readline("Enter the constraints of the column: ")
            .unwrap_or_else(|why| {
                panic!("Failed to read column constraints: {}", why.to_string());
            });

        // Ask if the column is nullable
        let nullable_input = rl
            .readline("Is the column nullable? (y/n): ")
            .unwrap_or_else(|why| {
                panic!("Failed to read nullable: {}", why.to_string());
            });

        // Convert input to bool
        let nullable = match nullable_input.trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => {
                println!("Invalid input. Please enter 'y' for yes or 'n' for no.");
                continue; // Ask again if input is invalid
            }
        };

        // validate the column type to sql type
        match column_type.to_lowercase().as_str() {
            "string" => {
                // ask the user for the length of the string
                let string_length = rl
                    .readline("Enter the length of the string: ")
                    .unwrap_or_else(|why| {
                        panic!("Failed to read string length: {}", why.to_string());
                    });

                match nullable {
                    true => {
                        // add the column to the up.sql file
                        up_sql_contents.push_str(&format!(
                            "ALTER TABLE {} ADD COLUMN {} VARCHAR({}) {};",
                            table_name, column_name, string_length, column_constraints
                        ));
                        up_sql_contents.push('\n');

                        // add the column to the up.sql file
                        down_sql_contents.push_str(&format!(
                            "ALTER TABLE {} ADD COLUMN {} VARCHAR({}) {};",
                            table_name, column_name, string_length, column_constraints
                        ));
                        down_sql_contents.push('\n');
                    }
                    false => {
                        // add the column to the up.sql file
                        up_sql_contents.push_str(&format!(
                            "ALTER TABLE {} ADD COLUMN {} VARCHAR({}) NOT NULL {};",
                            table_name, column_name, string_length, column_constraints
                        ));
                        up_sql_contents.push('\n');

                        // add the column to the up.sql file
                        down_sql_contents.push_str(&format!(
                            "ALTER TABLE {} ADD COLUMN {} VARCHAR({}) NOT NULL {};",
                            table_name, column_name, string_length, column_constraints
                        ));
                        down_sql_contents.push('\n');
                    }
                }

                // add the column to the up.sql file
                down_sql_contents.push_str(&format!(
                    "ALTER TABLE {} ADD COLUMN {} VARCHAR({}) {};",
                    table_name, column_name, string_length, column_constraints
                ));
                down_sql_contents.push('\n');

                // write the up.sql file
                write_to_file(&up_file, up_sql_contents.as_bytes())
                    .unwrap_or_else(|why| panic!("Couldn't write to up.sql: {}", why.to_string()));

                // write the down.sql file
                down_sql_contents.push_str(&format!(
                    "ALTER TABLE {} DROP COLUMN {};",
                    table_name, column_name
                ));
                down_sql_contents.push('\n');

                // write the down.sql file
                write_to_file(&down_file, down_sql_contents.as_bytes()).unwrap_or_else(|why| {
                    panic!("Couldn't write to down.sql: {}", why.to_string())
                });

                continue;
            }
            "integer" => {
                // add the column to the down.sql file
                down_sql_contents.push_str(&format!(
                    "ALTER TABLE {} ADD COLUMN {} INT {};",
                    table_name, column_name, column_constraints
                ));
                down_sql_contents.push('\n');

                // write the up.sql file
                write_to_file(&up_file, up_sql_contents.as_bytes())
                    .unwrap_or_else(|why| panic!("Couldn't write to up.sql: {}", why.to_string()));

                // write the down.sql file
                write_to_file(&down_file, down_sql_contents.as_bytes()).unwrap_or_else(|why| {
                    panic!("Couldn't write to down.sql: {}", why.to_string())
                });

                continue;
            }
            "float" => {
                // add the column to the down.sql file
                down_sql_contents.push_str(&format!(
                    "ALTER TABLE {} ADD COLUMN {} FLOAT {};",
                    table_name, column_name, column_constraints
                ));
                down_sql_contents.push('\n');

                // write the up.sql file
                write_to_file(&up_file, up_sql_contents.as_bytes())
                    .unwrap_or_else(|why| panic!("Couldn't write to up.sql: {}", why.to_string()));

                // write the down.sql file
                write_to_file(&down_file, down_sql_contents.as_bytes()).unwrap_or_else(|why| {
                    panic!("Couldn't write to down.sql: {}", why.to_string())
                });

                continue;
            }
            "boolean" => {
                // add the column to the down.sql file
                down_sql_contents.push_str(&format!(
                    "ALTER TABLE {} ADD COLUMN {} BOOLEAN {};",
                    table_name, column_name, column_constraints
                ));
                down_sql_contents.push('\n');

                // write the up.sql file
                write_to_file(&up_file, up_sql_contents.as_bytes())
                    .unwrap_or_else(|why| panic!("Couldn't write to up.sql: {}", why.to_string()));

                // write the down.sql file
                write_to_file(&down_file, down_sql_contents.as_bytes()).unwrap_or_else(|why| {
                    panic!("Couldn't write to down.sql: {}", why.to_string())
                });

                continue;
            }
            "date" => {
                // add the column to the down.sql file
                down_sql_contents.push_str(&format!(
                    "ALTER TABLE {} ADD COLUMN {} DATE {};",
                    table_name, column_name, column_constraints
                ));
                down_sql_contents.push('\n');

                // write the up.sql file
                write_to_file(&up_file, up_sql_contents.as_bytes())
                    .unwrap_or_else(|why| panic!("Couldn't write to up.sql: {}", why.to_string()));

                // write the down.sql file
                write_to_file(&down_file, down_sql_contents.as_bytes()).unwrap_or_else(|why| {
                    panic!("Couldn't write to down.sql: {}", why.to_string())
                });

                continue;
            }
            _ => {
                panic!("Invalid data type: {}", column_type);
            }
        }
    }
    // return the name of the migration
    Ok((&name).to_string())
}

/// ## Name: initialize_migration
/// ### Description: Creates the initial migration directory and the up.sql and down.sql files for the initial migration
/// ### Arguments:
/// * [`&project`] - The project struct that contains the project name and the project path
///
/// ### Returns:
/// * [`Result<(), CustomMigrationError>`] - Returns a result with a unit type or a CustomMigrationError
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::initial_migration;
///
/// let project = Project {
///    name: "test".to_string(),
///   path: "/home/user/test".to_string(),
///   // .. rest of the struct
/// };
/// let result = initialize_migration(&project);
///
/// assert!(result.is_ok());
/// ```
pub fn initialize_migration(project: &Project) -> Result<(), ErrorKind> {
    // create the migrations directory
    let sql = "
       CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
  );";
    let migrations_dir = Path::new(&project.initial_migration_directory).join("migrations");

    if !migrations_dir.exists() {
        create_dir_all(&migrations_dir).unwrap_or_else(|why| {
            panic!("Couldn't create migrations directory: {}", why.to_string())
        });
    }

    // create the up.sql file
    let up_file = &project.initial_migration_up;

    // write the up.sql file
    write_to_file(&up_file, sql.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to up.sql: {}", why.to_string()));

    let sql_to_down = "
    DROP TABLE IF EXISTS users;
    ";

    // create the down.sql file
    let down_file = &project.initial_migration_down;

    // write the down.sql file
    write_to_file(&down_file, sql_to_down.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to down.sql: {}", why.to_string()));

    Ok(())
}
// Write the user-entered SQL queries to the up.sql file

#[derive(Debug)]
pub enum CustomMigrationError {
    MigrationError(MigrationError),
    IoError(std::io::Error),
    RunError(Box<dyn StdError + Send + Sync>),
}

impl Display for CustomMigrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MigrationError(err) => Display::fmt(err, f),
            Self::RunError(err) => Display::fmt(err, f),
            Self::IoError(err) => Display::fmt(err, f),
        }
    }
}

impl StdError for CustomMigrationError {}

impl From<MigrationError> for CustomMigrationError {
    fn from(err: MigrationError) -> Self {
        Self::MigrationError(err)
    }
}

impl From<Box<dyn StdError + Send + Sync>> for CustomMigrationError {
    fn from(err: Box<dyn StdError + Send + Sync>) -> Self {
        Self::RunError(err)
    }
}

/// ## Name: run_migration
/// ### Description: Runs a migration
/// #### Parameters:
/// - name: [`&str`] - the name of the migration
/// - Returns: [`Result<(), std::io::Error>`]
/// - if the migration was created successfully: [`Ok(())`]
/// - if there was an error creating the migration: [`Err(std::io::Error)`]
///
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::run_migration;
///
/// run_migration("create_users_table").unwrap();
/// ```
pub fn run_migration(
    project: &Project,
    migration_name: String,
) -> Result<(), CustomMigrationError> {
    // Establish a database connection
    let mut connection = SqliteConnection::establish(&project.config_dev_db)
        .expect(&format!("Error connecting to {}", &project.config_dev_db));

    // Convert the migrations directory to a PathBuf
    let migrations_dir_path = PathBuf::from(format!("{}/{}", &project.migrations, migration_name));

    // Create a FileBasedMigrations instance based on the path to the migrations directory
    let migrations = FileBasedMigrations::from_path(migrations_dir_path)?;

    // Run pending migrations using the `run_pending_migrations` method on the connection
    // The `MigrationHarness` trait provides this method as an extension trait on the connection
    connection
        .run_pending_migrations(migrations)
        .map_err(|err| CustomMigrationError::RunError(err))?;

    Ok(())
}
