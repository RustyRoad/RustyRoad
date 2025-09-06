use chrono::Local;
use regex::Regex;
use sqlx::Executor;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, DirEntry};
use std::io::Read;
use std::path::Path;
use std::{
    fmt, fs,
    io::{self, ErrorKind},
};

use crate::database::{Database, DatabaseConnection, DatabaseType};
use rustyline::DefaultEditor;
use serde::de::StdError;
use serde_derive::Deserialize;
use strum_macros::Display;

use crate::generators::create_file;
use crate::writers::write_to_file;
use crate::Project;

use super::column_loop::column_loop;

const CONSTRAINTS: &[&str] = &["PRIMARY KEY", "NOT NULL", "FOREIGN KEY"];

/// Represents the type of migration operation
#[derive(Debug, PartialEq)]
enum MigrationType {
    CreateTable(String),
    AddColumn { table_name: String, columns: Vec<String> },
}

/// Parse migration name to determine the type of operation
fn parse_migration_name(name: &str, columns: &[String]) -> MigrationType {
    // Check for "add_*_to_*" pattern
    let add_pattern = Regex::new(r"^add_(.+)_to_(.+)$").unwrap();
    
    if let Some(captures) = add_pattern.captures(name) {
        let column_part = captures.get(1).unwrap().as_str();
        let table_name = captures.get(2).unwrap().as_str().to_string();
        
        // If columns are provided via CLI, use those; otherwise derive from name
        let columns_to_add = if columns.is_empty() {
            // Extract column names from the migration name
            // For "add_page_id_to_funnel_steps", extract "page_id"
            vec![column_part.to_string()]
        } else {
            columns.to_vec()
        };
        
        return MigrationType::AddColumn {
            table_name,
            columns: columns_to_add,
        };
    }
    
    // Default to CREATE TABLE operation
    MigrationType::CreateTable(name.to_string())
}

/// Generate foreign key constraint name and reference table
fn generate_foreign_key_info(column_name: &str) -> Option<(String, String)> {
    if column_name.ends_with("_id") {
        let table_name = column_name.strip_suffix("_id").unwrap();
        let table_name_plural = if table_name.ends_with('s') {
            table_name.to_string()
        } else {
            format!("{}s", table_name) // Simple pluralization
        };
        let constraint_name = format!("fk_{}", table_name);
        Some((constraint_name, table_name_plural))
    } else {
        None
    }
}



/// ## Name: MigrationInput
/// ### Description: A struct that represents the input for a migration
/// #### Fields:
/// - name: [`String`] - the name of the migration
/// - columns: [`Vec<ColumnInput>`] - a vector of columns
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::MigrationInput;
/// use rustyroad::database::migrations::ColumnInput;
/// 
/// let migration_input = MigrationInput {
///     name: "create_users_table".to_string(),
///     columns: vec![
///     ColumnInput {
///         name: "id".to_string(),
///         data_type: "SERIAL".to_string(),
///     },
///     ColumnInput {
///             name: "name".to_string(),
///             data_type: "VARCHAR(255)".to_string(),
///         },
///     ],
/// };
/// ```
#[derive(Deserialize)]
pub struct MigrationInput {
    pub name: String,
    pub columns: Vec<ColumnInput>,
}

/// ## Name: ColumnInput
/// ### Description: A struct that represents the input for a column
/// #### Fields:
/// - name: [`String`] - the name of the column
/// - data_type: [`String`] - the data type of the column
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::ColumnInput;
/// 
/// let column_input = ColumnInput {
///     name: "id".to_string(),
///     data_type: "SERIAL".to_string(),
/// };
/// ```
#[derive(Deserialize)]
pub struct ColumnInput {
    pub name: String,
    pub data_type: String,
}

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
pub async fn create_migration(name: &str, columns: Vec<String>) -> Result<(), io::Error> {
    let path = std::env::current_dir().unwrap();

    if fs::read_to_string(path.join("rustyroad.toml")).is_err() {
        return Err(io::Error::new(
            ErrorKind::Other,
            "Error reading the rustyroad.toml, please see the documentation for more information.",
        ));
    }

    match fs::create_dir("config/database") {
        Ok(_) => {}
        Err(_) => {}
    }

    match fs::create_dir("config/database/migrations") {
        Ok(_) => {}
        Err(_) => {}
    }

    // Parse the migration name to determine the operation type
    let migration_type = parse_migration_name(name, &columns);
    
    let (up_sql_contents, down_sql_contents, folder_name) = match migration_type {
        MigrationType::CreateTable(table_name) => {
            // Existing CREATE TABLE logic
            let mut column_definitions_sql = Vec::new();
            
            if columns.is_empty() {
                println!("No columns specified. Adding default 'id SERIAL PRIMARY KEY' column.");
                column_definitions_sql.push("id SERIAL PRIMARY KEY".to_string());
            } else {
                for col_def in columns {
                    let parts: Vec<&str> = col_def.split(':').collect();
                    if parts.len() < 2 {
                        eprintln!("Skipping invalid column definition: '{}'. Format is name:type[:constraints]", col_def);
                        continue;
                    }
                    let col_name = parts[0];
                    let col_type = parts[1];
                    let sql_type = map_common_type_to_sql(col_type);

                    let mut constraints_sql = Vec::new();
                    if parts.len() > 2 {
                        let constraints_str = parts[2];
                        for constraint in constraints_str.split(',') {
                            match constraint.to_lowercase().as_str() {
                                "primary_key" => constraints_sql.push("PRIMARY KEY".to_string()),
                                "not_null" => constraints_sql.push("NOT NULL".to_string()),
                                "unique" => constraints_sql.push("UNIQUE".to_string()),
                                _ if constraint.starts_with("default=") => {
                                    let default_value = constraint.splitn(2, '=').nth(1).unwrap_or("");
                                    let quoted_value = if default_value.chars().all(char::is_numeric) {
                                        default_value.to_string()
                                    } else {
                                        format!("'{}'", default_value)
                                    };
                                    constraints_sql.push(format!("DEFAULT {}", quoted_value));
                                }
                                _ => eprintln!("Warning: Unsupported constraint '{}' for column '{}'", constraint, col_name),
                            }
                        }
                    }

                    column_definitions_sql.push(format!("{} {} {}", col_name, sql_type, constraints_sql.join(" ")).trim().to_string());
                }
            }

            let up_sql = format!(
                "CREATE TABLE IF NOT EXISTS {} (\n    {}\n);",
                table_name,
                column_definitions_sql.join(",\n    ")
            );
            let down_sql = format!("DROP TABLE IF EXISTS {};", table_name);
            let folder_name = format!(
                "config/database/migrations/{}-{}",
                Local::now().format("%Y%m%d%H%M%S"),
                table_name
            );
            
            (up_sql, down_sql, folder_name)
        },
        
        MigrationType::AddColumn { table_name, columns: cols_to_add } => {
            // New ALTER TABLE logic
            let mut alter_statements = Vec::new();
            let mut foreign_key_statements = Vec::new();
            let mut column_names_for_down = Vec::new();
            
            if !columns.is_empty() {
                // Use columns from CLI
                for col_def in columns {
                    let parts: Vec<&str> = col_def.split(':').collect();
                    if parts.len() < 2 {
                        eprintln!("Skipping invalid column definition: '{}'. Format is name:type[:constraints]", col_def);
                        continue;
                    }
                    let col_name = parts[0];
                    let col_type = parts[1];
                    let sql_type = map_common_type_to_sql(col_type);

                    alter_statements.push(format!("ADD COLUMN {} {}", col_name, sql_type));
                    column_names_for_down.push(col_name.to_string());
                    
                    // Check if this is a foreign key column
                    if let Some((constraint_name, ref_table)) = generate_foreign_key_info(col_name) {
                        foreign_key_statements.push(format!(
                            "ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {}(id)",
                            constraint_name, col_name, ref_table
                        ));
                    }
                }
            } else {
                // Use column names derived from migration name with default type
                for col_name in &cols_to_add {
                    // Default to INTEGER type, but this should ideally be specified in the command
                    alter_statements.push(format!("ADD COLUMN {} INTEGER", col_name));
                    column_names_for_down.push(col_name.clone());
                    
                    // Check if this is a foreign key column
                    if let Some((constraint_name, ref_table)) = generate_foreign_key_info(col_name) {
                        foreign_key_statements.push(format!(
                            "ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {}(id)",
                            constraint_name, col_name, ref_table
                        ));
                    }
                }
            }

            // Combine ALTER statements
            let mut all_statements = alter_statements;
            all_statements.extend(foreign_key_statements);
            
            let up_sql = format!(
                "ALTER TABLE {}\n{};",
                table_name,
                all_statements.join(",\n")
            );
            
            let down_sql = format!(
                "ALTER TABLE {}\n{};",
                table_name,
                column_names_for_down.iter()
                    .map(|col| format!("DROP COLUMN {}", col))
                    .collect::<Vec<_>>()
                    .join(",\n")
            );
            
            let folder_name = format!(
                "config/database/migrations/{}-{}",
                Local::now().format("%Y%m%d%H%M%S"),
                name // Use original migration name for folder
            );
            
            (up_sql, down_sql, folder_name)
        }
    };

    create_migration_files(&folder_name, &up_sql_contents, &down_sql_contents)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_migration_name_add_column() {
        let name = "add_page_id_to_funnel_steps";
        let columns = vec!["page_id:integer".to_string()];
        
        let result = parse_migration_name(name, &columns);
        
        match result {
            MigrationType::AddColumn { table_name, columns: _cols } => {
                assert_eq!(table_name, "funnel_steps");
            },
            _ => panic!("Expected AddColumn migration type"),
        }
    }

    #[test] 
    fn test_parse_migration_name_create_table() {
        let name = "create_users_table";
        let columns = vec!["name:string".to_string()];
        
        let result = parse_migration_name(name, &columns);
        
        match result {
            MigrationType::CreateTable(table_name) => {
                assert_eq!(table_name, "create_users_table");
            },
            _ => panic!("Expected CreateTable migration type"),
        }
    }

    #[test]
    fn test_generate_foreign_key_info() {
        let result = generate_foreign_key_info("page_id");
        assert_eq!(result, Some(("fk_page".to_string(), "pages".to_string())));
        
        let result = generate_foreign_key_info("user_id");
        assert_eq!(result, Some(("fk_user".to_string(), "users".to_string())));
        
        let result = generate_foreign_key_info("name");
        assert_eq!(result, None);
    }

    #[test]
    fn test_map_common_type_to_sql() {
        assert_eq!(map_common_type_to_sql("integer"), "INTEGER");
        assert_eq!(map_common_type_to_sql("string"), "VARCHAR(255)");
        assert_eq!(map_common_type_to_sql("text"), "VARCHAR(255)");
        assert_eq!(map_common_type_to_sql("boolean"), "BOOLEAN");
    }
}

// Helper function placeholder for mapping common types to SQL types
// TODO: Implement proper mapping based on selected database dialect
fn map_common_type_to_sql(common_type: &str) -> String {
    match common_type.to_lowercase().as_str() {
        "string" | "text" => "VARCHAR(255)".to_string(), // Default, adjust as needed
        "integer" | "int" => "INTEGER".to_string(),
        "biginteger" | "bigint" => "BIGINT".to_string(),
        "boolean" | "bool" => "BOOLEAN".to_string(),
        "float" => "FLOAT".to_string(),
        "double" | "decimal" => "DECIMAL".to_string(), // Precision/scale might be needed
        "datetime" | "timestamp" => "TIMESTAMP".to_string(),
        "date" => "DATE".to_string(),
        "time" => "TIME".to_string(),
        "binary" | "blob" => "BYTEA".to_string(), // Example for PostgreSQL
        "json" => "JSONB".to_string(), // Example for PostgreSQL
        "uuid" => "UUID".to_string(), // Example for PostgreSQL
        "serial" => "SERIAL".to_string(), // Example for PostgreSQL (auto-incrementing integer)
        "bigserial" => "BIGSERIAL".to_string(), // Example for PostgreSQL (auto-incrementing bigint)
        // Add more mappings as needed
        _ => common_type.to_uppercase(), // Assume it's already a valid SQL type if not recognized
    }
}

pub fn create_migration_files(folder_name: &str, up_sql_contents: &str, down_sql_contents: &str) -> Result<(), io::Error> {
    let up_file = format!("{}/up.sql", folder_name);
    let down_file = format!("{}/down.sql", folder_name);

    match std::fs::create_dir(&folder_name) {
        Ok(_) => {}
        Err(_) => {
            println!("Migration already exists");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Migration already exists",
            ));
        }
    }

    create_file(&up_file)?;
    create_file(&down_file)?;

    write_to_file(&up_file, up_sql_contents.as_bytes())?;
    write_to_file(&down_file, down_sql_contents.as_bytes())?;

    Ok(())
}

 
/// # Name: get_column_details
/// ### Description: gets the details for each column
/// ### Arguments: 
/// * [`num_columns`] - The number of columns in the table
/// * [`table_name`] - The name of the table
/// 
/// ### Returns:
/// * [`Result<(String, String), io::Error>`] - Returns a tuple with the up.sql contents and the rust struct contents or an io::Error
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::get_column_details;
/// 
/// let num_columns = 2;
/// let table_name = "users";
/// let result = get_column_details(num_columns, table_name);
/// 
/// assert!(result.is_ok());
/// ```
pub fn get_column_details(num_columns: i32, table_name: &str) -> Result<(String, String), io::Error> {
   let result  =  column_loop(num_columns, table_name.to_string()).expect("wrong");
    
    Ok((result.up_sql_contents, result.rust_struct_contents))
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
/// use rustyroad::database::initialize_migration;
/// use rustyroad::database::migrations::initial_migration;
/// use rustyroad::Project;
///
/// let project = Project {
///    name: "test".to_string(),
///   rustyroad_toml: "".to_string(),src_dir: "".to_string(),main_rs: "".to_string(),cargo_toml: "".to_string(),package_json: "".to_string(),readme: "".to_string(),gitignore: "".to_string(),templates: "".to_string(),static_dir: "".to_string(),template_components: "".to_string(),template_sections: "".to_string(),template_layouts: "".to_string(),template_pages: "".to_string(),static_css: "".to_string(),static_js: "".to_string(),index_js: "".to_string(),static_images: "".to_string(),config: "".to_string(),config_env: "".to_string(),config_dev_env: "".to_string(),config_prod_env: "".to_string(),config_test_env: "".to_string(),config_default_env: "".to_string(),db: "".to_string(),config_dev_db: "".to_string(),config_prod_db: "".to_string(),config_test_db: "".to_string(),controllers: "".to_string(),controllers_module: "".to_string(),controllers: "".to_string(),models: "".to_string(),models_module: "".to_string(),migrations: "".to_string(),seeders: "".to_string(),tests: "".to_string(),config_initializers: "".to_string(),config_initializers_assets: "".to_string(),config_initializers_db: "".to_string(),config_initializers_default: "".to_string(),config_initializers_middleware: "".to_string(),config_initializers_controllers: "".to_string(),index_html: "".to_string(),base_html: "".to_string(),tailwind_css: "".to_string(),tailwind_config: "".to_string(),postcss_config: "".to_string(),not_found_html: "".to_string(),server_error_html: "".to_string(),favicon_ico: "".to_string(),robots_txt: "".to_string(),login_page_html: "".to_string(),signup_page_html: "".to_string(),reset_password_page_html: "".to_string(),forgot_password_page_html: "".to_string(),dashboard_page_html: "".to_string(),user_controller_directory: "".to_string(),user_controller: "".to_string(),user_controller_module: "".to_string(),user_model: "".to_string(),initial_migration_directory: "".to_string(),initial_migration_up: "".to_string(),initial_migration_down: "".to_string(),user_test: "".to_string(),user_controller: "".to_string(),index_controller: "".to_string(),login_controller: "".to_string(),signup_controller: "".to_string(),reset_password_controller: "".to_string(),forgot_password_controller: "".to_string(),dashboard_controller: "".to_string(),navbar_component: "".to_string(),
///   // .. rest of the struct
/// header_section: "".to_string(),};
/// let result = initialize_migration(&project);
///
/// assert!(result.is_ok());
/// ```
pub fn initialize_migration(project: &Project) -> Result<(), ErrorKind> {
    // create the migrations directory
    let sql = "
       CREATE TABLE IF NOT EXISTSIF NOT EXISTS users (
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
    SqlxError(sqlx::Error),
    RunError(Box<dyn StdError + Send + Sync>),
    SendError(Box<dyn StdError + Send>),
}

impl Display for CustomMigrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MigrationError(err) => Display::fmt(err, f),
            Self::RunError(err) => Display::fmt(err, f),
            Self::IoError(err) => Display::fmt(err, f),
            Self::SqlxError(err) => Display::fmt(err, f),
            Self::SendError(err) => Display::fmt(err, f),
        }
    }
}

impl StdError for CustomMigrationError {}

impl From<MigrationError> for CustomMigrationError {
    fn from(err: MigrationError) -> Self {
        Self::MigrationError(err)
    }
}

impl From<std::io::Error> for CustomMigrationError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<sqlx::Error> for CustomMigrationError {
    fn from(err: sqlx::Error) -> Self {
        Self::SqlxError(err)
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
/// use rustyroad::database::MigrationDirection;
/// use rustyroad::database::migrations::run_migration;
///
/// run_migration("create_users_table".to_string(), MigrationDirection::Up);
/// ```
pub async fn run_migration(
    migration_name: String,
    direction: MigrationDirection,
) -> Result<(), CustomMigrationError> {
    // get the database
    let database: Database = Database::get_database_from_rustyroad_toml().expect("Couldn't parse the rustyroad.toml file. Please check the documentation for a proper implementation.");
    match database.database_type {
        DatabaseType::Postgres => {
            println!("Database Type: PostGres");
        }
        DatabaseType::Mysql => {
            println!("Database Type: MySql");
        }
        DatabaseType::Sqlite => {
            println!("Database Type: Sqlite");
        }
        _ => {
            println!("coming soon");
        }
    }
    let migrations_dir_path = format!("./config/database/migrations");
    // find the folder that has the name of the migration in the migrations directory with the latest timestamp
    let migration_dir_selected =
        find_migration_dir(migrations_dir_path.clone(), migration_name.clone())
            .unwrap_or_else(|why| panic!("Couldn't find migration directory: {}", why.to_string()));
    // Generate the path to the migrations directory at runtime
    let migration_dir = &migration_dir_selected;
    println!("Migration directory: {:?}", migration_dir);
    // Get migration files from the specified directory
    let mut migration_files: Vec<_> = fs::read_dir(migration_dir.clone())
        .unwrap_or_else(|why| panic!("Couldn't read migrations directory: {}", why.to_string()))
        .filter_map(Result::ok)
        .collect();
    // Sort the migration files based on their name (to apply them in order)
    migration_files.sort_by_key(|entry| entry.file_name());

    // Print the path to the migration directory and the migration name
    println!("Migration directory path: {:?}", migration_dir.clone());
    println!("Migration name: {:?}", &migration_name.clone());

    // create the connection pool
    let connection = Database::create_database_connection(&database)
        .await
        .unwrap_or_else(|why| panic!("Couldn't create database connection: {}", why.to_string()));

    // Create migrations table if it doesn't exist
    match connection.clone() {
        DatabaseConnection::Pg(conn) => {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS _rustyroad_migrations (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    direction VARCHAR(10) NOT NULL
                )"
            ).await.map_err(CustomMigrationError::from)?;
        }
        DatabaseConnection::MySql(conn) => {
            match conn.execute(
                "CREATE TABLE IF NOT EXISTS _rustyroad_migrations (
                    id INT AUTO_INCREMENT PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    direction VARCHAR(10) NOT NULL
                )"
            ).await {
                Ok(_) => (),
                Err(e) => return Err(CustomMigrationError::SqlxError(e)),
            }
        }
        DatabaseConnection::Sqlite(conn) => {
            match conn.execute(
                "CREATE TABLE IF NOT EXISTS _rustyroad_migrations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    direction TEXT NOT NULL
                )"
            ).await {
                Ok(_) => (),
                Err(e) => return Err(CustomMigrationError::SqlxError(e)),
            }
        }
    }

    // Execute the migration and handle potential errors
    match execute_migration_with_connection(connection.clone(), migration_files, direction.clone()).await {
        Ok(_) => {
            // Record the migration in the tracking table
            let direction_str = match direction {
                MigrationDirection::Up => "up",
                MigrationDirection::Down => "down",
            };
            match connection {
                DatabaseConnection::Pg(conn) => {
                    conn.execute(
                        format!(
                            "INSERT INTO _rustyroad_migrations (name, direction) VALUES ('{}', '{}')",
                            migration_name, direction_str
                        ).as_str()
                    ).await.map_err(CustomMigrationError::from)?;
                }
                DatabaseConnection::MySql(conn) => {
                    conn.execute(
                        format!(
                            "INSERT INTO _rustyroad_migrations (name, direction) VALUES ('{}', '{}')",
                            migration_name, direction_str
                        ).as_str()
                    ).await.map_err(CustomMigrationError::from)?;
                }
                DatabaseConnection::Sqlite(conn) => {
                    conn.execute(
                        format!(
                            "INSERT INTO _rustyroad_migrations (name, direction) VALUES ('{}', '{}')",
                            migration_name, direction_str
                        ).as_str()
                    ).await.map_err(CustomMigrationError::from)?;
                }
            }

            // Only print success message if execution was successful
            match direction {
                MigrationDirection::Up => println!("Migration applied successfully"),
                MigrationDirection::Down => println!("Migration rolled back successfully"),
            }
        }
        Err(why) => {
            // Print the detailed error and return it
            eprintln!("Failed to execute migration: {}", why);
            return Err(CustomMigrationError::RunError(Box::new(why)));
        }
    }
    Ok(())
}

/// Create a new migration file
/// # Arguments
/// * `up_sql_contents` - The contents of the up.sql file
///
/// # Returns
/// * `Result<String, Box<dyn Error>>` - The up_sql_content with Array column added or an error
///
/// # Example
/// ```
/// use rustyroad::RustyRoad;
/// let nullable = false;
/// let up_sql_contents = String::from("CREATE TABLE IF NOT EXISTS users (
///    id SERIAL PRIMARY KEY,
///   username VARCHAR(255) NOT NULL,
///  password VARCHAR(255) NOT NULL,
/// email VARCHAR(255) NOT NULL,
/// created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
/// );
/// ");
/// let array = rustyroad::database::create_array(up_sql_contents, nullable);
/// assert!(array.is_ok());
/// ```
///
pub fn create_array(up_sql_contents: String, nullable: bool) -> Result<String, Box<dyn Error>> {
    // ask the user how many dimensions the array should have
    let dimensions = String::new();
    // Initialize the rustyline Editor with the default helper and in-memory history
    let mut rl = DefaultEditor::new().unwrap_or_else(|why| {
        panic!("Failed to create rustyline editor: {}", why.to_string());
    });
    rl.readline_with_initial(
        "How many dimensions should the array have? ",
        (dimensions.as_str(), ""),
    )
    .unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

    // ask the user for the type of the array
    let array_type = String::new();
    rl.readline_with_initial(
        "What type should the array have? ",
        (array_type.as_str(), ""),
    )
    .unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

    // ask the user for the name of the array
    let array_name = String::new();

    rl.readline_with_initial(
        "What should the name of the array be? ",
        (array_name.as_str(), ""),
    )
    .unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

    // ask the user for the size of the array
    let array_size = String::new();
    rl.readline_with_initial(
        "What should the size of the array be? ",
        (array_size.as_str(), ""),
    )
    .unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

    // add the array to the up_sql_contents
    let mut up_sql_contents = up_sql_contents;

    match nullable {
        true => {
            up_sql_contents.push_str(&format!(
                "ALTER TABLE users ADD COLUMN {} {} ARRAY[{}] NULL;\n",
                array_name, array_type, array_size
            ));
        }
        false => {
            up_sql_contents.push_str(&format!(
                "ALTER TABLE users ADD COLUMN {} {} ARRAY[{}] NOT NULL;\n",
                array_name, array_type, array_size
            ));
        }
    }

    Ok(up_sql_contents)
}

/// # Name: find_migration_dir
/// ### Description: Find the migration directory of a given migration name
/// ### This is used in case there are multiple migrations with the same name and different timestamps
/// ### If there are multiple migrations with the same name and different timestamps, the user will be prompted to choose one
/// ### If there is only one migration with the given name, the user will not be prompted to choose one
/// Given: A migration name and a rustyroad project
/// When: The user wants to execute a migration
/// Then: The user will be prompted to choose a migration if there are multiple migrations with the same name and different timestamps
/// Then: The user will not be prompted to choose a migration if there is only one migration with the given name
/// Then: The path to the migration directory will be returned
/// # Arguments
/// * `migrations_dir_path` - The path to the migrations directory
/// * `migration_name` - The name of the migration
/// # Returns
/// * `Result<String, Error>` - The path to the migration directory or an error
pub fn find_migration_dir(
    migrations_dir_path: String,
    migration_name: String,
) -> Result<String, Box<dyn Error>> {
    println!("Searching for migration directory: {}", migration_name);
    // Initialize the rustyline Editor with the default helper and in-memory history
    let mut rl = DefaultEditor::new().unwrap_or_else(|why| {
        panic!("Failed to create rustyline editor: {}", why.to_string());
    });
    println!("Migrations directory path: {:?}", migrations_dir_path.clone());
    // get all the migration directories
    let mut migration_dirs = Vec::new();
    for entry in fs::read_dir(migrations_dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            migration_dirs.push(path);
        }
    }
    println!("Migration directories: {:?}", migration_dirs.clone());

    // filter the migration directories by the migration name
    let mut filtered_migration_dirs = Vec::new();
    for migration_dir in migration_dirs {
        let migration_dir_name = migration_dir
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .ok_or("Failed to convert OsStr to str")?;
        println!("Migration directory name: {}", migration_dir_name);
        // Extract the name part after the timestamp and hyphen
        if let Some(name_part) = migration_dir_name.split_once('-').map(|(_, name)| name) {
            if name_part == migration_name { // Exact match comparison
                filtered_migration_dirs.push(migration_dir);
                println!("Filtered migration directories (exact match): {:?}", filtered_migration_dirs.clone());
            }
        }
    }

    // if there is only one migration directory with the given name, return it
    if filtered_migration_dirs.len() == 1 {
        println!("Filtered migration directories: {:?}", filtered_migration_dirs.clone());
        return Ok(filtered_migration_dirs[0].to_str().unwrap().to_string());
    }

    // if there are multiple migration directories with the given name, prompt the user to choose one
    if filtered_migration_dirs.len() > 1 {
        let mut migration_dir_names = Vec::new();
        println!("Filtered migration directories: {:?}", filtered_migration_dirs.clone());
        for migration_dir in &filtered_migration_dirs {
            println!("Migration directory: {:?}", migration_dir.clone());
            let migration_dir_name = migration_dir.file_name().unwrap().to_str().unwrap();
            println!("Migration directory name: {}", migration_dir_name);
            migration_dir_names.push(migration_dir_name);
        }
        let migration_dir_name = rl
            .readline_with_initial(
                "Which migration do you want to execute? ",
                (migration_dir_names[0], ""),
            )
            .unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

        print!("You chose: {}", migration_dir_name);

        for migration_dir in filtered_migration_dirs {
            println!("Migration directory: {:?}", migration_dir.clone());
            let migration_dir_name_from_list = migration_dir
                .file_name()
                .expect("Failed to get file name")
                .to_str()
                .ok_or("Failed to convert OsStr to str")?;
            if migration_dir_name == migration_dir_name_from_list {
                println!("Migration directory name: {}", migration_dir_name);
                return Ok(migration_dir
                    .to_str()
                    .ok_or("Failed to convert PathBuf to str")?
                    .to_string());
            }
        }
    }

    Err(Box::new(std::io::Error::new(
        ErrorKind::Other,
        "Failed to find migration directory",
    )))
}

#[derive(Debug)]
pub enum MigrationError {
    Io(io::Error),
    Sql(sqlx::Error),
}

impl From<io::Error> for MigrationError {
    fn from(err: io::Error) -> MigrationError {
        MigrationError::Io(err)
    }
}

impl From<sqlx::Error> for MigrationError {
    fn from(err: sqlx::Error) -> MigrationError {
        MigrationError::Sql(err)
    }
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::Io(err) => write!(f, "IO error: {}", err),
            MigrationError::Sql(err) => write!(f, "SQL error: {}", err),
        }
    }
}


impl std::error::Error for MigrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MigrationError::Io(err) => Some(err),
            MigrationError::Sql(err) => Some(err),
        }
    }
}


#[derive(PartialEq, Clone, Copy, Debug, Display)]
pub enum MigrationDirection {
    Up,
    Down,
}

async fn execute_migration_with_connection(
    connection: DatabaseConnection,
    migration_files: Vec<DirEntry>,
    direction: MigrationDirection,
) -> Result<(), MigrationError> {
    for entry in migration_files {
        let path = entry.path();
        // Ignore non-SQL files
        if path.extension() != Some(std::ffi::OsStr::new("sql")) {
            continue;
        }
        let mut file = fs::File::open(&path).expect("Failed to open migration file");
        let mut sql = String::new();
        file.read_to_string(&mut sql)
            .expect("Failed to read migration file");

        // Skip the migration if it is a down.sql file and we're migrating up, or vice versa
        let is_down_file = path.file_stem() == Some(std::ffi::OsStr::new("down"));
        if (direction == MigrationDirection::Up && is_down_file)
            || (direction == MigrationDirection::Down && !is_down_file)
        {
            continue;
        }
        match connection.clone() {
            DatabaseConnection::Pg(connection) => {
                println!("Executing query: {:?}", sql.clone().as_str());
                //unwrap the arc
                let rows_affected = connection.execute(sql.as_str()).await?;
                println!("{:?} rows affected", rows_affected);
            }
            DatabaseConnection::MySql(connection) => {
                println!("Executing query: {:?}", sql);
                let rows_affected = connection.execute(sql.as_str()).await?;
                println!("{:?} rows affected", rows_affected);
            }
            DatabaseConnection::Sqlite(connection) => {
                println!("Executing query: {:?}", sql);
                let rows_affected = connection.execute(sql.as_str()).await?;
                println!("{:?} rows affected", rows_affected);
            }
        };
    }
    Ok(())
}

/// ## Name: list_migrations
/// ### Description: Lists all migrations and their status (applied or not)
/// ### Returns:
/// * `Result<(), CustomMigrationError>` - Returns Ok if successful, or an error
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::list_migrations;
///
/// list_migrations().await?;
/// ```
pub async fn list_migrations() -> Result<(), CustomMigrationError> {
    // get the database
    let database: Database = Database::get_database_from_rustyroad_toml().expect("Couldn't parse the rustyroad.toml file. Please check the documentation for a proper implementation.");
    
    // create the connection pool
    let connection = Database::create_database_connection(&database)
        .await
        .unwrap_or_else(|why| panic!("Couldn't create database connection: {}", why.to_string()));

    // Get all migration files from the migrations directory
    let migrations_dir = Path::new("./config/database/migrations");
    let mut migration_files = Vec::new();
    
    if migrations_dir.exists() {
        let entries = match fs::read_dir(migrations_dir) {
            Ok(entries) => entries,
            Err(e) => return Err(CustomMigrationError::IoError(e)),
        };
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => return Err(CustomMigrationError::IoError(e)),
            };
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some((_, migration_name)) = name.split_once('-') {
                        migration_files.push(migration_name.to_string());
                    }
                }
            }
        }
    }

    // Get applied migrations from the database
    let applied_migrations = match connection {
        DatabaseConnection::Pg(conn) => {
            match sqlx::query_as::<_, (String, String, String)>(
                "SELECT name, applied_at, direction FROM _rustyroad_migrations ORDER BY applied_at"
            )
            .fetch_all(&*conn)
            .await {
                Ok(rows) => rows,
                Err(e) => return Err(CustomMigrationError::SqlxError(e)),
            }
        }
        DatabaseConnection::MySql(conn) => {
            match sqlx::query_as::<_, (String, String, String)>(
                "SELECT name, applied_at, direction FROM _rustyroad_migrations ORDER BY applied_at"
            )
            .fetch_all(&*conn)
            .await {
                Ok(rows) => rows,
                Err(e) => return Err(CustomMigrationError::SqlxError(e)),
            }
        }
        DatabaseConnection::Sqlite(conn) => {
            match sqlx::query_as::<_, (String, String, String)>(
                "SELECT name, applied_at, direction FROM _rustyroad_migrations ORDER BY applied_at"
            )
            .fetch_all(&*conn)
            .await {
                Ok(rows) => rows,
                Err(e) => return Err(CustomMigrationError::SqlxError(e)),
            }
        }
    };

    println!("Migrations:");
    println!("{:<30} {:<20} {:<10}", "Name", "Applied At", "Status");
    println!("{:-<30} {:-<20} {:-<10}", "", "", "");

    // Print all migration files with their status
    for migration in &migration_files {
        let applied = applied_migrations.iter()
            .find(|(name, _, dir)| name == migration && dir == "up");
        let rolled_back = applied_migrations.iter()
            .find(|(name, _, dir)| name == migration && dir == "down");

        let status = match (applied, rolled_back) {
            (Some((_, applied_at, _)), None) => format!("Applied at {}", applied_at),
            (None, Some(_)) => "Rolled back".to_string(),
            (None, None) => "Pending".to_string(),
            _ => "Unknown".to_string(),
        };

        println!("{:<30} {:<20} {:<10}", migration, "", status);
    }

    Ok(())
}
