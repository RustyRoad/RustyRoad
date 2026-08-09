// Import the tokio::test macro

use rustyroad::database::Database;
use rustyroad::Project;
use std::fs;
use std::path::PathBuf;

struct CurrentDirGuard(PathBuf);

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.0).expect("failed to restore test working directory");
    }
}

#[tokio::test]
async fn test_integration_load_sql_for_new_project() {
    let original_dir = std::env::current_dir().unwrap();
    let _guard = CurrentDirGuard(original_dir);
    let test_dir = tempfile::tempdir().unwrap();
    std::env::set_current_dir(test_dir.path()).unwrap();

    // define sql ite database data
    let database_data = Database::new(
        "pwned".to_owned(),
        "test_user".to_owned(),
        "password".to_owned(),
        "localhost".to_owned(),
        3306,
        "sqlite".to_string().as_str(),
    );

    // Create a sample project
    let project = Project::create_new_project("example".to_string(), database_data).await;
    assert!(&project.is_ok(), "Expected valid project");

    // Verify the up migration file is present
    let project = &project.unwrap();
    let up_migration_file = &project.initial_migration_up;
    let up_migration_path = test_dir.path().join(up_migration_file);
    // check that the file exists
    let is_up_present = fs::metadata(&up_migration_path).is_ok();
    assert!(is_up_present, "Expected up migration file to be present");

    // Verify the down migration file is present
    let down_migration_file = &project.initial_migration_down;
    let down_migration_path = test_dir.path().join(down_migration_file);
    // check that the file exists
    let is_down_present = fs::metadata(down_migration_path).is_ok();
    assert!(
        is_down_present,
        "Expected down migration file to be present"
    );
}
