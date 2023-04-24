use crate::writers::write_to_file;
use futures_util::future::FutureExt;
use std::str::FromStr;

use sqlparser::dialect::SQLiteDialect;
use tokio_postgres::{Config, Error, NoTls};

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
/// use crate::writers::general::sql_writer::write_to_sql;
///
/// write_to_sql("test.sql", "SELECT * FROM test;".to_string());
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
    database_name: &str,
) -> Result<(), Error> {
    // Parse the connection string
    let config = Config::from_str(admin_database_url)?;

    // Connect to the default "postgres" database to check for existence and create a new database
    let (client, connection) = config.connect(NoTls).await?;

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
            &[&database_name],
        )
        .await?;

    if row.is_none() {
        // If the database does not exist, create it
        client
            .batch_execute(&format!("CREATE DATABASE {}", database_name))
            .await?;
    }

    Ok(())
}
