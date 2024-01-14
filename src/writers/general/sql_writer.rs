use crate::{database::Database, writers::write_to_file};
use color_eyre::eyre::Result;
use futures_util::future::FutureExt;
use sqlparser::dialect::SQLiteDialect;
use sqlx::{mysql::MySqlConnectOptions, MySqlPool};
use std::str::FromStr;
use tokio_postgres::{Config, NoTls};

/// # Name: write_to_sql
/// # Description: Writes to a sql file and creates the file if it does not exist
/// # Arguments:
/// ## * `name` - The name of the file -```String```
/// ## * `sql` - The sql to be written to the file - ```String```
/// This function will be use to generate sql files.
/// This function will take two arguments, the name of the file, and the sql that will be written to the file.
/// The function will create a template that gets passed to the writer.
/// The sql will be written to the file
/// # Example
/// ```
/// use rustyroad::writers::write_to_sql;
///
/// write_to_sql(&"test.sql".to_string(), &"SELECT * FROM test;".to_string());
///
/// ```
/// # Result
/// ```sql
/// SELECT * FROM test;
/// ```
pub fn write_to_sql(file_name: &String, sql: &str) -> Result<(), std::io::Error> {
    // parse the sql to make sure it is valid
    let _ = sqlparser::parser::Parser::parse_sql(&SQLiteDialect {}, sql).unwrap();
    // if the sql is not valid, the parser will throw an error

    let mut template = String::new();

    template.push_str(&format!("{}", sql));

    // write the template to the file
    write_to_file(file_name, template.as_bytes())
}

pub async fn create_database_if_not_exists(
    admin_database_url: &str,
    database: Database,
) -> Result<(), Box<dyn std::error::Error>> {
    match database.database_type {
        crate::database::DatabaseType::Postgres => {
            // Parse the connection string
            let config = Config::from_str(admin_database_url).unwrap();

            // Connect to the default "postgres" database to check for existence and create a new database
            let (client, connection) = config.connect(NoTls).await.unwrap();

            // The connection object performs the actual communication with the database,
            // so spawn it off to run on its own.
            let connection_task = connection.map(|r| {
                if let Err(e) = r {
                    eprintln!("Connection error: {}", e);
                }
            });
            tokio::spawn(connection_task); // Spawn the connection task

            // Check if the specified database exists
            let row = client
                .query_opt(
                    "SELECT 1 FROM pg_database WHERE datname = $1",
                    &[&database.name],
                )
                .await
                .unwrap();

            if row.is_none() {
                // If the database does not exist, create it
                client
                    .batch_execute(&format!("CREATE DATABASE \"{}\"", &database.name))
                    .await
                    .unwrap();
            }
        }
        crate::database::DatabaseType::Mysql => {
            // Parse the connection string
            let mut options = MySqlConnectOptions::from_str(admin_database_url).unwrap();

            // Set the options to connect to the default "mysql" database
            options = options.database("mysql");

            // Create a connection pool
            let pool = MySqlPool::connect_with(options).await.unwrap();

            // Check if the specified database exists
            let row: Option<(String,)> = sqlx::query_as(
                "SELECT SCHEMA_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = ?",
            )
            .bind(&database.name)
            .fetch_optional(&pool)
            .await
            .unwrap();

            if row.is_none() {
                // If the database does not exist, create it
                sqlx::query(&format!("CREATE DATABASE {}", &database.name))
                    .execute(&pool)
                    .await
                    .unwrap();
            }
        }
        crate::database::DatabaseType::Sqlite => todo!(),
        crate::database::DatabaseType::Mongo => todo!(),
    }

    Ok(())
}
