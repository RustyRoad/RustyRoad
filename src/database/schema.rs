use sqlx::{Row, Column, ValueRef};
use crate::database::{Database, DatabaseConnection};
use crate::database::migrations::CustomMigrationError;

/// Inspects and prints the database schema
pub async fn inspect_schema() -> Result<(), CustomMigrationError> {
    let database = Database::get_database_from_rustyroad_toml()
        .expect("Couldn't parse the rustyroad.toml file");

    let connection = Database::create_database_connection(&database)
        .await
        .map_err(|e| CustomMigrationError::SendError(e))?;

    match connection {
        DatabaseConnection::Pg(conn) => {
            let tables = sqlx::query(
                "SELECT table_name FROM information_schema.tables 
                 WHERE table_schema = 'public'"
            )
            .fetch_all(&*conn)
            .await?;

            println!("Database Schema:");
            println!("{:-<30}", "");
            
            for table in tables {
                let table_name: String = table.get("table_name");
                println!("Table: {}", table_name);
                
                let columns = sqlx::query(
                    "SELECT column_name, data_type 
                     FROM information_schema.columns 
                     WHERE table_name = $1"
                )
                .bind(&table_name)
                .fetch_all(&*conn)
                .await?;

                for col in columns {
                    let name: String = col.get("column_name");
                    let dtype: String = col.get("data_type");
                    println!("  - {}: {}", name, dtype);
                }
                println!("{:-<30}", "");
            }
        }
        DatabaseConnection::MySql(conn) => {
            let tables = sqlx::query(
                "SELECT table_name FROM information_schema.tables 
                 WHERE table_schema = DATABASE()"
            )
            .fetch_all(&*conn)
            .await?;

            println!("Database Schema:");
            println!("{:-<30}", "");
            
            for table in tables {
                let table_name: String = table.get("table_name");
                println!("Table: {}", table_name);
                
                let columns = sqlx::query(
                    "SELECT column_name, data_type 
                     FROM information_schema.columns 
                     WHERE table_name = ?"
                )
                .bind(&table_name)
                .fetch_all(&*conn)
                .await?;

                for col in columns {
                    let name: String = col.get("column_name");
                    let dtype: String = col.get("data_type");
                    println!("  - {}: {}", name, dtype);
                }
                println!("{:-<30}", "");
            }
        }
        DatabaseConnection::Sqlite(conn) => {
            let tables = sqlx::query(
                "SELECT name FROM sqlite_master 
                 WHERE type='table'"
            )
            .fetch_all(&*conn)
            .await?;

            println!("Database Schema:");
            println!("{:-<30}", "");
            
            for table in tables {
                let table_name: String = table.get("name");
                println!("Table: {}", table_name);
                
                let columns = sqlx::query(
                    &format!("PRAGMA table_info({})", table_name)
                )
                .fetch_all(&*conn)
                .await?;

                for col in columns {
                    let name: String = col.get("name");
                    let dtype: String = col.get("type");
                    println!("  - {}: {}", name, dtype);
                }
                println!("{:-<30}", "");
            }
        }
    }

    Ok(())
}

/// Executes a SQL query and prints the results
pub async fn execute_query(query: &str) -> Result<(), CustomMigrationError> {
    let database = Database::get_database_from_rustyroad_toml()
        .expect("Couldn't parse the rustyroad.toml file");

    let connection = Database::create_database_connection(&database)
        .await
        .map_err(|e| CustomMigrationError::SendError(e))?;

    println!("Executing query: {}", query);
    println!("{:-<50}", "");

    match connection {
        DatabaseConnection::Pg(conn) => {
            let rows = sqlx::query(query)
                .fetch_all(&*conn)
                .await?;

            if rows.is_empty() {
                println!("No results found.");
                return Ok(());
            }

            // Print column headers
            if let Some(first_row) = rows.first() {
                let columns = first_row.columns();
                for (i, column) in columns.iter().enumerate() {
                    if i > 0 { print!(" | "); }
                    print!("{:<15}", column.name());
                }
                println!();
                println!("{:-<50}", "");
            }

            // Print data rows
            for row in rows {
                let columns = row.columns();
                for (i, column) in columns.iter().enumerate() {
                    if i > 0 { print!(" | "); }
                    let value = match row.try_get_raw(column.name()) {
                        Ok(value) => {
                            if value.is_null() {
                                "NULL".to_string()
                            } else {
                                // Try to decode as common types
                                if let Ok(s) = row.try_get::<String, _>(column.name()) {
                                    s
                                } else if let Ok(i) = row.try_get::<i32, _>(column.name()) {
                                    i.to_string()
                                } else if let Ok(i) = row.try_get::<i64, _>(column.name()) {
                                    i.to_string()
                                } else if let Ok(b) = row.try_get::<bool, _>(column.name()) {
                                    b.to_string()
                                } else {
                                    "<unprintable>".to_string()
                                }
                            }
                        }
                        Err(_) => "<error>".to_string(),
                    };
                    print!("{:<15}", value);
                }
                println!();
            }
        }
        DatabaseConnection::MySql(conn) => {
            let rows = sqlx::query(query)
                .fetch_all(&*conn)
                .await?;

            if rows.is_empty() {
                println!("No results found.");
                return Ok(());
            }

            // Print column headers
            if let Some(first_row) = rows.first() {
                let columns = first_row.columns();
                for (i, column) in columns.iter().enumerate() {
                    if i > 0 { print!(" | "); }
                    print!("{:<15}", column.name());
                }
                println!();
                println!("{:-<50}", "");
            }

            // Print data rows
            for row in rows {
                let columns = row.columns();
                for (i, column) in columns.iter().enumerate() {
                    if i > 0 { print!(" | "); }
                    let value = match row.try_get_raw(column.name()) {
                        Ok(value) => {
                            if value.is_null() {
                                "NULL".to_string()
                            } else {
                                // Try to decode as common types
                                if let Ok(s) = row.try_get::<String, _>(column.name()) {
                                    s
                                } else if let Ok(i) = row.try_get::<i32, _>(column.name()) {
                                    i.to_string()
                                } else if let Ok(i) = row.try_get::<i64, _>(column.name()) {
                                    i.to_string()
                                } else if let Ok(b) = row.try_get::<bool, _>(column.name()) {
                                    b.to_string()
                                } else {
                                    "<unprintable>".to_string()
                                }
                            }
                        }
                        Err(_) => "<error>".to_string(),
                    };
                    print!("{:<15}", value);
                }
                println!();
            }
        }
        DatabaseConnection::Sqlite(conn) => {
            let rows = sqlx::query(query)
                .fetch_all(&*conn)
                .await?;

            if rows.is_empty() {
                println!("No results found.");
                return Ok(());
            }

            // Print column headers
            if let Some(first_row) = rows.first() {
                let columns = first_row.columns();
                for (i, column) in columns.iter().enumerate() {
                    if i > 0 { print!(" | "); }
                    print!("{:<15}", column.name());
                }
                println!();
                println!("{:-<50}", "");
            }

            // Print data rows
            for row in rows {
                let columns = row.columns();
                for (i, column) in columns.iter().enumerate() {
                    if i > 0 { print!(" | "); }
                    let value = match row.try_get_raw(column.name()) {
                        Ok(value) => {
                            if value.is_null() {
                                "NULL".to_string()
                            } else {
                                // Try to decode as common types
                                if let Ok(s) = row.try_get::<String, _>(column.name()) {
                                    s
                                } else if let Ok(i) = row.try_get::<i32, _>(column.name()) {
                                    i.to_string()
                                } else if let Ok(i) = row.try_get::<i64, _>(column.name()) {
                                    i.to_string()
                                } else if let Ok(b) = row.try_get::<bool, _>(column.name()) {
                                    b.to_string()
                                } else {
                                    "<unprintable>".to_string()
                                }
                            }
                        }
                        Err(_) => "<error>".to_string(),
                    };
                    print!("{:<15}", value);
                }
                println!();
            }
        }
    }

    Ok(())
}