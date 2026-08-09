#[cfg(test)]
mod tests {

    use rustyroad::database::Database;
    use rustyroad::Project;
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::ConnectOptions;
    use std::path::PathBuf;

    struct CurrentDirGuard(PathBuf);

    impl Drop for CurrentDirGuard {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.0).expect("failed to restore test working directory");
        }
    }

    // Define a new struct to hold the table names
    #[derive(Debug, sqlx::FromRow, PartialEq)]
    struct TableResult {
        name: String,
    }

    #[tokio::test]
    async fn test_run_migration() -> Result<(), Box<dyn std::error::Error>> {
        let original_dir = std::env::current_dir()?;
        let _guard = CurrentDirGuard(original_dir);
        let test_dir = tempfile::tempdir()?;
        std::env::set_current_dir(test_dir.path())?;

        // SQLite databases do not require a username, password, host, or port, so we leave these fields empty
        let database: Database = Database::new(
            "test".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            0,
            "sqlite".to_string().as_str(),
        );

        // Create a new project with the desired name
        let project_name = String::from("example");
        let project = Project::create_new_project(project_name, database.clone())
            .await
            .expect("Error creating new project");

        // Verify that the migration was applied successfully
        let mut connection = SqliteConnectOptions::new()
            // Use the config_dev_db field as the SQLite database file path
            .filename(test_dir.path().join(&project.config_dev_db))
            .connect()
            .await?;
        println!("{:?}", connection);
        println!("{:?}", project.config_dev_db);
        // Query to list all tables in the SQLite database
        let query = "SELECT name FROM sqlite_master WHERE type='table';";
        // Run the query and load the result into a Vec<TableResult>
        let result = sqlx::query_as::<_, TableResult>(query)
            .fetch_all(&mut connection)
            .await?;

        // Print the list of tables
        println!("List of tables in the database: {:?}", result);

        // Assert that the 'users' table was created
        assert!(
            result.contains(&TableResult {
                name: "Users".to_string()
            }),
            "The 'Users' table should have been created by the migration"
        );

        Ok(())
    }
}
