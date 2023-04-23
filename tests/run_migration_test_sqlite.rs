#[cfg(test)]
mod tests {
    use diesel::{prelude::*, QueryableByName};

    use rustyroad::database::{run_migration, Database};
    use rustyroad::writers::templates::new;
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::{ConnectOptions, SqliteConnection};
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tokio::test;

    // Define a new struct to hold the table names
    #[derive(Debug, sqlx::FromRow, PartialEq)]
    struct TableResult {
        name: String,
    }

    #[tokio::test]
    async fn test_run_migration() -> Result<(), Box<dyn std::error::Error>> {
        let database: Database = Database::new(
            "test".to_owned(),
            "sqlite".to_owned(),
            "test".to_owned(),
            "localhost".to_owned(),
            "5432".to_owned(),
            "test".to_owned(),
        );

        // Create a new project with the desired name
        let project_name = String::from("test_project");
        let project = new(project_name);

        // Write a dummy migration file to the migrations directory
        let migration_name = "20230409_create_table_users";
        let migrations_dir = PathBuf::from(&project.migrations);
        let migration_dir = migrations_dir.join(migration_name);
        std::fs::create_dir_all(&migration_dir)?;
        let migration_path = migration_dir.join("up.sql");
        let mut migration_file = File::create(migration_path.clone())?; // Clone the path before use
        writeln!(
            migration_file,
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);"
        )?;

        // Print migration information
        println!("Migration directory path: {:?}", migration_dir);
        println!("Migration name: {:?}", migration_name);
        println!(
            "Content of 'up.sql':\n{}",
            std::fs::read_to_string(migration_path)?
        );

        std::env::set_var("DATABASE_URL", &project.config_dev_db);
        // Call the run_migration function with the migration_name
        match run_migration(&project, migration_name.to_string(), database).await {
            Ok(_) => println!("Migration ran successfully"),
            Err(e) => println!("Migration failed with error: {:?}", e),
        }
        // Verify that the migration was applied successfully
        let mut connection = SqliteConnectOptions::new()
            .filename(&project.config_dev_db)
            .connect()
            .await?;

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
                name: "users".to_string()
            }),
            "The 'users' table should have been created by the migration"
        );

        Ok(())
    }
}
