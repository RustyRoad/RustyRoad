use chrono::prelude::*;
use rustyline::history::MemHistory;
use rustyline::Editor;
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

    let file = format!("{}/up.sql", folder_name).to_string();

    // Initialize the rustyline Editor
    // let mut rl = Editor::<(), MemHistory>::load_history("history.txt").unwrap();

    // Prompt the user for the migration type
    let migration_type = match rl.readline("Enter the migration type (CRUD/Custom): ") {
        Ok(input) => input.trim().to_string(),
        Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to read input")),
    };

    // Define table name and columns based on migration type
    let (table_name, mut columns) = match migration_type.to_lowercase().as_str() {
        "crud" => {
            // Prompt the user for the table name
            let table_name = match rl.readline("Enter the table name: ") {
                Ok(input) => input.trim().to_string(),
                Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to read input")),
            };

            // Automatically add columns for CRUD operations
            let columns = vec![
                "id SERIAL PRIMARY KEY",
                "name VARCHAR(255) NOT NULL",
                "email VARCHAR(255) NOT NULL",
                "created_at TIMESTAMP NOT NULL",
                "updated_at TIMESTAMP NOT NULL",
            ];

            (table_name, columns)
        }
        "custom" | _ => {
            // Prompt the user for the table name
            let table_name = match rl.readline("Enter the table name: ") {
                Ok(input) => input.trim().to_string(),
                Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to read input")),
            };

            // Prompt the user for the number of columns
            let num_columns: usize = match rl.readline("Enter the number of columns: ") {
                Ok(input) => match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(Error::new(ErrorKind::Other, "Invalid number of columns"))
                    }
                },
                Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to read input")),
            };

            // Prompt the user for column information
            let mut columns = Vec::new();
            for i in 0..num_columns {
                let column_name = match rl.readline(&format!("Enter name for column {}: ", i + 1)) {
                    Ok(input) => input.trim().to_string(),
                    Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to read input")),
                };

                // Show available column types
                println!("Choose type for column {}:", i + 1);
                for (index, column_type) in COLUMN_TYPES.iter().enumerate() {
                    println!("  [{}] {}", index + 1, column_type);
                }
                let column_type_index: usize = match rl.readline("Enter choice (1-3): ") {
                    Ok(input) => match input.trim().parse() {
                        Ok(index) => {
                            if index > 0 && index <= COLUMN_TYPES.len() {
                                index - 1
                            } else {
                                return Err(Error::new(ErrorKind::Other, "Invalid choice"));
                            }
                        }
                        Err(_) => return Err(Error::new(ErrorKind::Other, "Invalid choice")),
                    },
                    Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to read input")),
                };
                let column_type = COLUMN_TYPES[column_type_index];

                let constraints = match rl.readline(&format!(
                    "Enter constraints for column {} (e.g., PRIMARY KEY, NULL, etc.): ",
                    i + 1
                )) {
                    Ok(input) => input.trim().to_string(),
                    Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to read input")),
                };

                columns.push(format!("{} {} {}", column_name, column_type, constraints));
            }

            (table_name, columns)
        }
    };

    // Generate the SQL query based on user input
    let columns_definition = columns.join(",\n    ");
    let contents = format!(
        r#"
      CREATE TABLE IF NOT EXISTS {} (
    {}
  );
  "#,
        table_name, columns_definition
    );

    write_to_file(&file, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    // Create the down file
    create_file(&format!("{}/down.sql", folder_name).to_string())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    let file = format!("{}/down.sql", folder_name).to_string();

    let contents = format!(r#"DROP TABLE IF EXISTS {};"#, table_name);

    write_to_file(&file, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    Ok(())
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
