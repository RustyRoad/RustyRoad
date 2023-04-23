// Import the tokio::test macro
use tokio::test;

use rustyroad::database::Database;
use rustyroad::Project;
use std::fs;
#[cfg(test)]
use std::fs::read_to_string;

#[tokio::test]
async fn test_integration_load_sql_for_new_project() {
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
    let project = Project::create_new_project("example".to_string(), database_data);
    assert!(&project.await.is_ok(), "Expected valid project");

    // Verify the up migration file is present
    let project = &project.await.unwrap();
    let up_migration_file = project.initial_migration_up;
    // check that the file exists
    let is_up_present = fs::metadata(&up_migration_file).is_ok();
    assert!(is_up_present, "Expected up migration file to be present");

    let sql = read_to_string(up_migration_file.clone()).unwrap();
    // parse the sql file and verify the content
    assert!(sql.contains("CREATE TABLE Users ("));

    // Verify the down migration file is present
    let down_migration_file = project.initial_migration_down;
    // check that the file exists
    let is_down_present = fs::metadata(down_migration_file).is_ok();
    assert!(
        is_down_present,
        "Expected down migration file to be present"
    );
}
