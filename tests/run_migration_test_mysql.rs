#[cfg(test)]
mod tests {

    use sqlx::mysql::MySqlConnectOptions;
    use sqlx::{ConnectOptions, Row};

    use rustyroad::database::{Database, DatabaseType};
    use rustyroad::Project;

    #[tokio::test]
    async fn test_run_migration() -> Result<(), Box<dyn std::error::Error>> {
        let database_data = Database::new(
            "test".to_owned(),
            "admin".to_owned(),
            "admin".to_owned(),
            "localhost".to_owned(),
            3306,
            DatabaseType::Mysql.to_string().as_str(),
        );
        // Construct the database URL for the newly created database
        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            database_data.username,
            database_data.password,
            database_data.host,
            database_data.port,
            database_data.name
        );
        std::env::set_var("DATABASE_URL", &database_url);

        println!("{:?}", database_url.clone());

        // Create a new project with the desired name
        let project_name = String::from("example");

        // Call the create_new_project function to create the new project and database
        Project::create_new_project(project_name, database_data.clone())
            .await
            .expect("Error creating new project");

        // Establish a connection to the new database
        let mut connection = MySqlConnectOptions::new()
            .username(&database_data.username)
            .password(&database_data.password)
            .host(&database_data.host)
            .port(database_data.port)
            .database(&database_data.name)
            .connect()
            .await?;

        // Query to list all tables in the MySQL database
        let _query = "SHOW TABLES;";

        // Define the TableResult struct with the sqlx::FromRow derive macro
        #[derive(Debug, sqlx::FromRow, PartialEq)]
        struct TableResult {
            name: String,
        }

        // Run the query and load the result into a Vec<TableResult>
        let rows = sqlx::query(r#"SHOW TABLES;"#)
            .fetch_all(&mut connection)
            .await?;

        let result: Vec<TableResult> = rows
            .into_iter()
            .map(|row| TableResult {
                name: row
                    .try_get::<String, _>(0)
                    .unwrap_or_else(|_| String::new()),
            })
            .collect();

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
