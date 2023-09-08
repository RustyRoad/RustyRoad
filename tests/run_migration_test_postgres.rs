#[cfg(test)]
mod tests {
    use sqlx::postgres::PgConnectOptions;
    use sqlx::ConnectOptions;

    use rustyroad::database::{
        DataTypeCategory, Database, DatabaseType, DatabaseTypeTrait, PostgresDatabaseType,
        TypesForDatabase,
    };
    use rustyroad::Project;

    #[tokio::test]
    async fn test_run_migration() -> Result<(), Box<dyn std::error::Error>> {
        let database_data = Database::new(
            "test112".to_owned(),
            "postgres".to_owned(),
            "postgres".to_owned(),
            "localhost".to_owned(),
            5434,
            DatabaseType::Postgres.to_string().as_str(),
        );

        // Construct the database URL for the newly created database
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            database_data.username,
            database_data.password,
            database_data.host,
            database_data.port,
            database_data.name.clone()
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
        let mut connection = PgConnectOptions::new()
            .username(&database_data.username)
            .password(&database_data.password)
            .host(&database_data.host)
            .port(database_data.port)
            .database(&database_data.name.clone())
            .connect()
            .await?;

        // Query to list all tables in the PostgreSQL database
        // Query to list all tables in the PostgreSQL database
        let _query =
            "SELECT tablename as name FROM pg_catalog.pg_tables WHERE schemaname = 'public';";

        // Define the TableResult struct with the sqlx::FromRow derive macro
        #[derive(Debug, sqlx::FromRow, PartialEq)]
        struct TableResult {
            name: String,
        }

        // Run the query and load the result into a Vec<TableResult>
        let result: Vec<TableResult> = sqlx::query_as::<_, TableResult>(
            r#"SELECT tablename as name FROM pg_catalog.pg_tables WHERE schemaname = 'public';"#,
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

    #[tokio::test]
    async fn test_postgres_database_type_get_database_types() {
        let postgres_database_type = PostgresDatabaseType;
        let data_type_category = DataTypeCategory::Numeric;
        let data_types_for_category = TypesForDatabase::new();

        let result = postgres_database_type
            .get_database_types(&data_types_for_category, &data_type_category);

        assert_eq!(result.len(), 1);
        let types_for_database: &TypesForDatabase = &result[0];

        for postgres_type in types_for_database.clone().postgres.types {
            println!("{:?}", postgres_type);
        }

        println!("{:?}", types_for_database.postgres.types.len());

        assert_eq!(types_for_database.postgres.types.len(), 6);
        assert_eq!(types_for_database.mysql.types.len(), 0);
        assert_eq!(types_for_database.sqlite.types.len(), 0);

        let expected_data_types = vec![
            "smallint", "integer", "bigint", "decimal", "numeric", "real",
        ];

        for data_type in expected_data_types {
            assert!(types_for_database
                .postgres
                .types
                .contains_key(&data_type.to_string()));
        }
    }
}
