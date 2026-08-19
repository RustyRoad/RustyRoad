use crate::database::migrations::CustomMigrationError;
use crate::database::render;
use crate::database::statement::is_multi_statement;
use crate::database::values::DecodableRow;
use crate::database::{get_config_file_name, Database, DatabaseConnection};
use serde_json::json;
use sqlx::{Column, Row, ValueRef};

#[derive(serde::Serialize)]
struct SchemaColumn {
    name: String,
    r#type: String,
    nullable: bool,
}

#[derive(serde::Serialize)]
struct SchemaTable {
    name: String,
    columns: Vec<SchemaColumn>,
}

#[derive(serde::Serialize)]
struct SchemaOutput {
    database_type: String,
    config_file: String,
    tables: Vec<SchemaTable>,
}

/// Inspects and prints the database schema
pub async fn inspect_schema(format: &str) -> Result<(), CustomMigrationError> {
    let database = Database::get_database_from_rustyroad_toml()
        .expect("Couldn't parse the rustyroad.toml file");

    let config_file = get_config_file_name();

    let connection = Database::create_database_connection(&database)
        .await
        .map_err(CustomMigrationError::SendError)?;

    let mut all_tables: Vec<SchemaTable> = Vec::new();
    let db_type = database.database_type.to_string().to_ascii_lowercase();

    match connection {
        DatabaseConnection::Pg(conn) => {
            let tables = sqlx::query(
                "SELECT table_name FROM information_schema.tables 
                 WHERE table_schema = 'public'",
            )
            .fetch_all(&*conn)
            .await?;

            for table in tables {
                let table_name: String = table.get("table_name");

                let columns = sqlx::query(
                    "SELECT column_name, data_type, is_nullable
                     FROM information_schema.columns 
                     WHERE table_name = $1",
                )
                .bind(&table_name)
                .fetch_all(&*conn)
                .await?;

                let mut table_columns: Vec<SchemaColumn> = Vec::new();
                for col in columns {
                    let name: String = col.get("column_name");
                    let dtype: String = col.get("data_type");
                    let nullable: String = col.get("is_nullable");
                    table_columns.push(SchemaColumn {
                        name,
                        r#type: dtype,
                        nullable: nullable == "YES",
                    });
                }
                all_tables.push(SchemaTable {
                    name: table_name,
                    columns: table_columns,
                });
            }
        }
        DatabaseConnection::MySql(conn) => {
            let tables = sqlx::query(
                "SELECT table_name FROM information_schema.tables 
                 WHERE table_schema = DATABASE()",
            )
            .fetch_all(&*conn)
            .await?;

            for table in tables {
                let table_name: String = table.get("table_name");

                let columns = sqlx::query(
                    "SELECT column_name, data_type, is_nullable
                     FROM information_schema.columns 
                     WHERE table_name = ?",
                )
                .bind(&table_name)
                .fetch_all(&*conn)
                .await?;

                let mut table_columns: Vec<SchemaColumn> = Vec::new();
                for col in columns {
                    let name: String = col.get("column_name");
                    let dtype: String = col.get("data_type");
                    let nullable: String = col.get("is_nullable");
                    table_columns.push(SchemaColumn {
                        name,
                        r#type: dtype,
                        nullable: nullable == "YES",
                    });
                }
                all_tables.push(SchemaTable {
                    name: table_name,
                    columns: table_columns,
                });
            }
        }
        DatabaseConnection::Sqlite(conn) => {
            let tables = sqlx::query(
                "SELECT name FROM sqlite_master 
                 WHERE type='table'",
            )
            .fetch_all(&*conn)
            .await?;

            for table in tables {
                let table_name: String = table.get("name");

                let columns = sqlx::query(sqlx::AssertSqlSafe(format!(
                    "PRAGMA table_info({})",
                    table_name
                )))
                .fetch_all(&*conn)
                .await?;

                let mut table_columns: Vec<SchemaColumn> = Vec::new();
                for col in columns {
                    let name: String = col.get("name");
                    let dtype: String = col.get("type");
                    let not_null: i32 = col.get("notnull");
                    table_columns.push(SchemaColumn {
                        name,
                        r#type: dtype,
                        nullable: not_null == 0,
                    });
                }
                all_tables.push(SchemaTable {
                    name: table_name,
                    columns: table_columns,
                });
            }
        }
    }

    if format == "json" {
        let output = SchemaOutput {
            database_type: db_type,
            config_file,
            tables: all_tables,
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Database Schema:");
        println!("{:-<30}", "");
        for table in &all_tables {
            println!("Table: {}", table.name);
            for col in &table.columns {
                println!(
                    "  - {}: {}{}",
                    col.name,
                    col.r#type,
                    if col.nullable { "" } else { " NOT NULL" }
                );
            }
            println!("{:-<30}", "");
        }
    }

    Ok(())
}

/// Executes a SQL query and prints the results
/// Runs a multi-statement script using the unprepared protocol.
///
/// Row output is not available here, so this reports statement completion instead.
async fn execute_script(
    query: &str,
    connection: DatabaseConnection,
) -> Result<(), CustomMigrationError> {
    use sqlx::Executor;

    match connection {
        DatabaseConnection::Pg(conn) => conn
            .execute(sqlx::AssertSqlSafe(query.to_owned()))
            .await
            .map(|_| ())?,
        DatabaseConnection::MySql(conn) => conn
            .execute(sqlx::AssertSqlSafe(query.to_owned()))
            .await
            .map(|_| ())?,
        DatabaseConnection::Sqlite(conn) => conn
            .execute(sqlx::AssertSqlSafe(query.to_owned()))
            .await
            .map(|_| ())?,
    }

    println!("Script executed successfully.");
    Ok(())
}

pub async fn execute_query(query: &str, format: &str) -> Result<(), CustomMigrationError> {
    let database = Database::get_database_from_rustyroad_toml()
        .expect("Couldn't parse the rustyroad.toml file");

    let config_file = get_config_file_name();

    let connection = Database::create_database_connection(&database)
        .await
        .map_err(CustomMigrationError::SendError)?;

    if format == "json" {
        return execute_query_json(query, &database, &config_file, connection).await;
    }

    println!("Executing query: {}", query);
    println!("{:-<50}", "");

    // `sqlx::query` prepares the statement, and a prepared statement may hold only
    // one command. Route scripts through the unprepared path instead of failing with
    // "cannot insert multiple commands into a prepared statement".
    if is_multi_statement(query) {
        return execute_script(query, connection).await;
    }

    match connection {
        DatabaseConnection::Pg(conn) => {
            let rows = sqlx::query(sqlx::AssertSqlSafe(query.to_owned()))
                .fetch_all(&*conn)
                .await?;
            if !render::table(&rows, crate::database::values::pg_display) {
                println!("No results found.");
            }
        }
        DatabaseConnection::MySql(conn) => {
            let rows = sqlx::query(sqlx::AssertSqlSafe(query.to_owned()))
                .fetch_all(&*conn)
                .await?;
            if !render::table(&rows, DecodableRow::decode_display) {
                println!("No results found.");
            }
        }
        DatabaseConnection::Sqlite(conn) => {
            let rows = sqlx::query(sqlx::AssertSqlSafe(query.to_owned()))
                .fetch_all(&*conn)
                .await?;
            if !render::table(&rows, DecodableRow::decode_display) {
                println!("No results found.");
            }
        }
    }

    Ok(())
}

async fn execute_query_json(
    query: &str,
    database: &Database,
    config_file: &str,
    connection: DatabaseConnection,
) -> Result<(), CustomMigrationError> {
    // A prepared statement holds one command, so scripts must use the unprepared
    // path. Row output is unavailable there, so report completion instead.
    if is_multi_statement(query) {
        execute_script(query, connection).await?;
        let output = json!({
            "database_type": database.database_type.to_string().to_ascii_lowercase(),
            "config_file": config_file,
            "query": query,
            "multi_statement": true,
            "rows": [],
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    let rows_json = match connection {
        DatabaseConnection::Pg(conn) => {
            let rows = sqlx::query(sqlx::AssertSqlSafe(query.to_owned()))
                .fetch_all(&*conn)
                .await?;
            rows.iter().map(crate::database::values::pg_row).collect()
        }
        DatabaseConnection::MySql(conn) => {
            let rows = sqlx::query(sqlx::AssertSqlSafe(query.to_owned()))
                .fetch_all(&*conn)
                .await?;
            let mut rows_data: Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();

            for row in rows {
                let mut row_map: serde_json::Map<String, serde_json::Value> =
                    serde_json::Map::new();
                let columns = row.columns();
                for column in columns {
                    let col_name = column.name().to_string();
                    let value = match row.try_get_raw(column.name()) {
                        Ok(value) => {
                            if value.is_null() {
                                serde_json::Value::Null
                            } else {
                                if let Ok(s) = row.try_get::<String, _>(column.name()) {
                                    serde_json::Value::String(s)
                                } else if let Ok(i) = row.try_get::<i32, _>(column.name()) {
                                    serde_json::json!(i)
                                } else if let Ok(i) = row.try_get::<i64, _>(column.name()) {
                                    serde_json::json!(i)
                                } else if let Ok(b) = row.try_get::<bool, _>(column.name()) {
                                    serde_json::json!(b)
                                } else {
                                    serde_json::Value::String("<unprintable>".to_string())
                                }
                            }
                        }
                        Err(_) => serde_json::Value::String("<error>".to_string()),
                    };
                    row_map.insert(col_name, value);
                }
                rows_data.push(row_map);
            }
            rows_data
        }
        DatabaseConnection::Sqlite(conn) => {
            let rows = sqlx::query(sqlx::AssertSqlSafe(query.to_owned()))
                .fetch_all(&*conn)
                .await?;
            let mut rows_data: Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();

            for row in rows {
                let mut row_map: serde_json::Map<String, serde_json::Value> =
                    serde_json::Map::new();
                let columns = row.columns();
                for column in columns {
                    let col_name = column.name().to_string();
                    let value = match row.try_get_raw(column.name()) {
                        Ok(value) => {
                            if value.is_null() {
                                serde_json::Value::Null
                            } else {
                                if let Ok(s) = row.try_get::<String, _>(column.name()) {
                                    serde_json::Value::String(s)
                                } else if let Ok(i) = row.try_get::<i32, _>(column.name()) {
                                    serde_json::json!(i)
                                } else if let Ok(i) = row.try_get::<i64, _>(column.name()) {
                                    serde_json::json!(i)
                                } else if let Ok(b) = row.try_get::<bool, _>(column.name()) {
                                    serde_json::json!(b)
                                } else {
                                    serde_json::Value::String("<unprintable>".to_string())
                                }
                            }
                        }
                        Err(_) => serde_json::Value::String("<error>".to_string()),
                    };
                    row_map.insert(col_name, value);
                }
                rows_data.push(row_map);
            }
            rows_data
        }
    };

    let output = json!({
        "query": query,
        "database": database.name,
        "config_file": config_file,
        "row_count": rows_json.len(),
        "rows": rows_json
    });

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}
