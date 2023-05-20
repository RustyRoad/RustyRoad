use super::database::Database;
use crate::database;
use crate::database::DataTypeCategory;
use crate::generators::create_file;
use crate::writers::write_to_file;
use crate::Project;
use chrono::prelude::*;
// Import Client from postgres crate
use diesel_migrations::MigrationError;
use rustyline::DefaultEditor;
use sqlx::{postgres::PgPoolOptions, sqlite::SqlitePool};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::create_dir_all;
use std::fs::{self};
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;
use std::println;

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

    let table_name = name.to_string();

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

        let database: Database = Database::get_database_from_rustyroad_toml()?.into();

        let database_type = database.clone().database_type;
        // initalize data types
        let column_types = DataTypeCategory::get_all_categories();
        // match the available column types to the database type
        // need to get the values from the enum of the database type

        let column_category_to_list: HashMap<String, DataTypeCategory> = match database_type {
            database::DatabaseType::Postgres => column_types
                .iter()
                .map(|category| (category.to_string(), category.clone()))
                .collect(),
            database::DatabaseType::Mysql => todo!(),
            database::DatabaseType::Sqlite => todo!(),
            database::DatabaseType::Mongo => todo!(),
        };

        println!("Column Types: ");
        // loop through the column types and print them out
        for (index, column_type) in column_category_to_list.iter().enumerate() {
            println!("{}. {}", index + 1, column_type.0);
        }

        // prompt the user for the column type
        let column_type = rl
            // Ask the user for the column type and give them a list of available types
            .readline("Enter the type of the column: ")
            .unwrap_or_else(|why| {
                panic!("Failed to read column type: {}", why.to_string());
            });

        // match the column type to the data type
        let column_type_as_struct = match column_type.trim() {
            "1" => DataTypeCategory::Boolean,
            "2" => DataTypeCategory::Numeric,
            "3" => DataTypeCategory::DateTime,
            "4" => DataTypeCategory::Text,
            "5" => DataTypeCategory::Geometric,
            "6" => DataTypeCategory::NetworkAddress,
            "7" => DataTypeCategory::Json,
            "8" => DataTypeCategory::Search,
            "9" => DataTypeCategory::Array,
            "10" => DataTypeCategory::UUID,
            "11" => DataTypeCategory::Monetary,
            "12" => DataTypeCategory::BitString,
            "13" => DataTypeCategory::Interval,
            "14" => DataTypeCategory::Composite,
            "15" => DataTypeCategory::Range,
            "16" => DataTypeCategory::Other,
            _ => {
                println!("Invalid input. Please enter a number between 1 and 16.");
                continue; // Ask again if input is invalid
            }
        };

        // for the category that the user selected, get the data types for that category
        let data_types_for_category =
            column_type_as_struct.get_data_types_from_data_type_category(database_type.clone());

        // loop through the data types for the category and print them out
        // loop through the data types for the category and print them out
        println!("Data Types: ");
        for (index, data_type) in data_types_for_category.into_iter().enumerate() {
            println!("{}. {}", index + 1, data_type);
        }

        // prompt the user for the column type
        let new_type = rl
            // Ask the user for the column type and give them a list of available types
            .readline("Enter the type of the column: ")
            .unwrap_or_else(|why| panic!("Failed to read column type: {}", why.to_string()));

        // match the column type to the data type
        let column_type = match new_type.trim() {
            "1" => "BOOLEAN",
            "2" => "INTEGER",
            "3" => "FLOAT",
            "4" => "VARCHAR(255)",
            "5" => "DATE",
            "6" => "TIME",
            "7" => "TIMESTAMP",
            "8" => "JSON",
            "9" => "ARRAY",
            "10" => "UUID",
            "11" => "MONEY",
            "12" => "BITSTRING",
            "13" => "INTERVAL",
            "14" => "COMPOSITE",
            "15" => "RANGE",
            "16" => "OTHER",
            _ => {
                println!("Invalid input. Please enter a number between 1 and 16.");
                continue; // Ask again if input is invalid
            }
        };

        // prompt the user for the column constraints
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
        match column_type {
            "varchar(255)" => {
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
                // let the user know that the data type is not supported
                println!(
                    "The data type {} is not supported. Please try again.",
                    column_type
                );
                continue;
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
pub async fn run_migration(
    project: &Project,
    migration_name: String,
    database: Database,
) -> Result<(), CustomMigrationError> {
    // Generate the path to the migrations directory at runtime
    let migrations_dir_path = format!("{}/{}", &project.migrations, &migration_name);

    // Get migration files from the specified directory
    let mut migration_files: Vec<_> = fs::read_dir(&migrations_dir_path)
        .unwrap_or_else(|why| panic!("Couldn't read migrations directory: {}", why.to_string()))
        .filter_map(Result::ok)
        .collect();
    // Sort the migration files based on their name (to apply them in order)
    migration_files.sort_by_key(|entry| entry.file_name());

    // Print the path to the migration directory and the migration name
    println!("Migration directory path: {:?}", migrations_dir_path);
    println!("Migration name: {:?}", &migration_name.clone());

    // Determine the type of database and run the migrations
    match database.database_type {
        database::DatabaseType::Sqlite => {
            // Create a connection to the SQLite database
            // Create a connection pool to the SQLite database
            let pool = SqlitePool::connect(&project.config_dev_db)
                .await
                .unwrap_or_else(|why| panic!("Failed to connect to database: {}", why.to_string()));

            for entry in migration_files {
                let path = entry.path();
                // Ignore non-SQL files
                if path.extension() != Some(std::ffi::OsStr::new("sql")) {
                    continue;
                }
                let mut file = fs::File::open(&path).unwrap_or_else(|why| {
                    panic!("Failed to open migration file: {}", why.to_string())
                });
                let mut sql = String::new();
                file.read_to_string(&mut sql).unwrap_or_else(|why| {
                    panic!("Failed to read migration file: {}", why.to_string())
                });

                // Split the SQL statements and execute each one separately
                let sql_statements: Vec<&str> = sql.split(';').collect();
                for statement in sql_statements {
                    if statement.trim().is_empty() {
                        continue;
                    }
                    // Execute the SQL statement
                    sqlx::query(statement)
                        .execute(&pool)
                        .await
                        .unwrap_or_else(|why| {
                            panic!("Failed to execute migration: {}", why.to_string())
                        });
                    println!("Applied migration: {:?}", path.file_name().unwrap());
                }
            }
        }
        database::DatabaseType::Postgres => {
            // Create a connection pool to the PostgreSQL database
            let pool = PgPoolOptions::new()
                .max_connections(20)
                .connect(&project.config_dev_db)
                .await
                .unwrap_or_else(|why| panic!("Failed to connect to database: {}", why.to_string()));

            for entry in migration_files {
                let path = entry.path();
                // Ignore non-SQL files
                if path.extension() != Some(std::ffi::OsStr::new("sql")) {
                    continue;
                }
                let mut file = fs::File::open(&path).unwrap_or_else(|why| {
                    panic!("Failed to open migration file: {}", why.to_string())
                });
                let mut sql = String::new();
                file.read_to_string(&mut sql).unwrap_or_else(|why| {
                    panic!("Failed to read migration file: {}", why.to_string())
                });

                // Split the SQL statements and execute each one separately
                let sql_statements: Vec<&str> = sql.split(';').collect();
                for statement in sql_statements {
                    if statement.trim().is_empty() {
                        continue;
                    }
                    // Execute the SQL statement
                    sqlx::query(statement)
                        .execute(&pool)
                        .await
                        .unwrap_or_else(|why| {
                            panic!("Failed to execute migration: {}", why.to_string())
                        });
                    println!("Applied migration: {:?}", path.file_name().unwrap());
                }
            }
        }
        database::DatabaseType::Mysql => {
            // Create a connection pool to the MySQL database
            // let pool = MySqlPool::connect(&database.config_dev_db).await?;
            //
            // for entry in migration_files {
            //     let path = entry.path();
            //     // Ignore non-SQL files
            //     if path.extension() != Some(std::ffi::OsStr::new("sql")) {
            //         continue;
            //     }
            //     let mut file = fs::File::open(&path)?;
            //     let mut sql = String::new();
            //     file.read_to_string(&mut sql)?;
            //
            //     // Execute the SQL statements from the migration file
            //     sqlx::query(&sql).execute(&pool).await?;
            //     println!("Applied migration: {:?}", path.file_name().unwrap());
            // }
            todo!("ya");
        }
        database::DatabaseType::Mongo => {
            // Create a connection to the MongoDB database
            // let client = mongodb::Client::with_uri_str(&database.config_dev_db).await?;
            // let db = client.database("my_db"); // You need to specify the database name

            // for entry in migration_files {
            //     let path = entry.path();
            //     // Ignore non-SQL files
            //     if path.extension() != Some(std::ffi::OsStr::new("sql")) {
            //         continue;
            //     }
            //     let mut file = fs::File::open(&path)?;
            //     let mut sql = String::new();
            //     file.read_to_string(&mut sql)?;

            //     // Execute the SQL statements from the migration file
            //     db.run_command(mongodb::bson::doc! { "eval": sql }, None).await?;
            //     println!("Applied migration: {:?}", path.file_name().unwrap());
            // }
            // not implemented yet
            todo!("MongoDB migrations are not implemented yet.");
        }
    }

    println!("Migration executed successfully");
    Ok(())
}
