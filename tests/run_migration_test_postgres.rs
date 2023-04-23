#[cfg(test)]
mod tests {

    use sqlx::postgres::PgConnectOptions;
    use sqlx::{ConnectOptions, PgConnection};
    // Import the tokio::test macro
    use tokio::test;

    use diesel::{prelude::*, sql_query, QueryableByName};
    use rustyroad::database::{run_migration, Database};
    use rustyroad::writers::templates::new;
    use rustyroad::Project;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_run_migration() -> Result<(), Box<dyn std::error::Error>> {
        // Create a new project with the desired name
        let project_name = String::from("test_project");

        let database_data = Database::new(
            "test".to_owned(),
            "postgres".to_owned(),
            "postgres".to_owned(),
            "localhost".to_owned(),
            "5432".to_owned(),
            "postgres".to_owned(),
        );

        // Call the create_new_project function to create the new project and database
        let project = Project::create_new_project(project_name, database_data.clone())
            .await
            .expect("Error creating new project");

        // Write a dummy migration file to the migrations directory
        let migration_name = "20230409_create_table_users";
        let migrations_dir = PathBuf::from(&project.migrations);
        let migration_dir = migrations_dir.join(migration_name);
        std::fs::create_dir_all(&migration_dir)?;
        let migration_path = migration_dir.join("up.sql");
        let mut migration_file = File::create(migration_path.clone())?;
        writeln!(
            migration_file,
            "CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT);"
        )?;

        // Construct the database URL for the newly created database
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            database_data.username,
            database_data.password,
            database_data.host,
            database_data.port,
            database_data.name
        );

        // Establish a connection to the new database
        let mut connection = PgConnectOptions::new()
            .username(&database_data.username)
            .password(&database_data.password)
            .host(&database_data.host)
            .port(database_data.port.parse::<u16>()?)
            .database(&database_data.name)
            .connect()
            .await?;

        // Run the migration from the dummy migration file
        run_migration(&project, migration_name.to_string(), database_data.clone())
            .await
            .expect("Error running migration");

        // Query to list all tables in the PostgreSQL database
        // Query to list all tables in the PostgreSQL database
        let query =
            "SELECT tablename as name FROM pg_catalog.pg_tables WHERE schemaname = 'public';";

        // Define the TableResult struct with the sqlx::FromRow derive macro
        #[derive(Debug, sqlx::FromRow, PartialEq)]
        struct TableResult {
            name: String,
        }

        std::env::set_var("DATABASE_URL", database_url);

        // Run the query and load the result into a Vec<TableResult>
        let result: Vec<TableResult> = sqlx::query_as!(
            TableResult,
            r#"SELECT tablename as name FROM pg_catalog.pg_tables WHERE schemaname = 'public';"#
        )
        .fetch_all(&mut connection)
        .await?;

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
