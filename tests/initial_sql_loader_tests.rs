use rustyroad::Project;
use rustyroad::database::Database;
#[cfg(test)]
use rustyroad::writers::migrations::initial_sql_loader::load_sql_for_new_project;
use std::fs::{read_to_string, remove_file};
use std::io::ErrorKind;

#[test]
fn test_integration_load_sql_for_new_project() {
    // define sql ite database data
    let database_data = Database::new(
        "pwned".to_owned(),
        "test_user".to_owned(),
        "password".to_owned(),
        "localhost".to_owned(),
        "3306".to_owned(),
        "sqlite".to_owned(),
    );

    // Create a sample project
    let project = Project::create_new_project("test_project".to_string(), database_data).unwrap_or_else(|e| {
        panic!("Failed to create new project: {}", e);
    });

    // Call the function to generate the SQL file
    let result = load_sql_for_new_project(&project);
    assert!(result.is_ok(), "Expected successful SQL file generation");

    // Verify the content of the generated SQL file
    let file_name = format!("{}_initial.sql", project.config_initializers_db);
    match read_to_string(&file_name) {
        Ok(content) => {
            // Check that the generated SQL contains the expected statements
            assert!(content.contains("CREATE DATABASE test_project DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"));
            assert!(content.contains("CREATE TABLE Users ("));
            assert!(content.contains("CREATE TABLE Roles ("));
            assert!(content.contains("CREATE TABLE Permissions ("));
            assert!(content.contains("CREATE TABLE Sessions ("));
            assert!(content.contains("INSERT INTO Roles (name) VALUES ('admin');"));
            assert!(content
                .contains("INSERT INTO Permissions (name, role_id) VALUES ('create_user', 1);"));
            assert!(content
                .contains("INSERT INTO Permissions (name, role_id) VALUES ('read_user', 1);"));
            assert!(content.contains(
                "INSERT INTO Users (password, username, role_id) VALUES ('admin', 'admin', 1);"
            ));

            // Clean up by removing the generated SQL file
            remove_file(&file_name).expect("Failed to clean up test file");
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Test file not found"),
            other_error => panic!("An unexpected error occurred: {:?}", other_error),
        },
    }
}
