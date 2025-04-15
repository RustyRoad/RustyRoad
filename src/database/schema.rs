use sqlx::Row;
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