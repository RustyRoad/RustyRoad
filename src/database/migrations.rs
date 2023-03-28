use chrono::prelude::*;
use rustyline::DefaultEditor;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;

use crate::generators::create_file;
use crate::writers::write_to_file;

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
pub fn create_migration(name: &str) -> Result<(), Error> {
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
            return Err(Error::new(
                ErrorKind::Other,
                "This is not a rustyroad project",
            ));
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
            return Ok(());
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
    Ok(())
}
// Write the user-entered SQL queries to the up.sql file

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
pub fn run_migration(name: &str) -> Result<(), std::io::Error> {
    // Find the name of the migration in the migrations folder
    // Then run the migration

    let output = fs::read_dir("src/database/migrations")?;

    for entry in output {
        let entry = entry?;
        let path = entry.path();
        let path = path.to_str().unwrap();

        if path.contains(name) {
            let output = std::process::Command::new("diesel")
                .arg("migration")
                .arg("run")
                .arg("--migration-dir")
                .arg(path)
                .output()
                .expect("failed to execute process");

            println!("output: {}", String::from_utf8_lossy(&output.stdout));
            println!("error: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    Ok(())
}
