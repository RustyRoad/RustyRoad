//! RustyRoad MCP Server
//!
//! This MCP (Model Context Protocol) server exposes RustyRoad database operations
//! as tools that AI agents can use. This prevents agents from:
//! - Using raw `psql` commands
//! - Mixing up ENV vs ENVIRONMENT variables  
//! - Connecting to the wrong database
//!
//! ## Usage
//!
//! ```bash
//! # Start the MCP server (reads from stdin, writes to stdout)
//! rustyroad-mcp
//!
//! # With custom project directory
//! RUSTYROAD_PROJECT_DIR=/path/to/project rustyroad-mcp
//!
//! # With specific environment
//! ENV=prod rustyroad-mcp
//! ```
//!
//! ## Auto-registration
//!
//! ```bash
//! # Register with OpenCode
//! rustyroad-mcp --register
//! ```

use rustyroad::database::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Column, Row};
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

/// MCP Protocol version
const PROTOCOL_VERSION: &str = "2024-11-05";

/// Server info
const SERVER_NAME: &str = "rustyroad-mcp";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// Tool definition for MCP
#[derive(Debug, Serialize)]
struct Tool {
    name: String,
    description: String,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
}

/// Server state
struct McpServer {
    project_dir: PathBuf,
    environment: String,
}

impl McpServer {
    fn new() -> Self {
        let project_dir = env::var("RUSTYROAD_PROJECT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        let environment = env::var("ENV")
            .or_else(|_| env::var("ENVIRONMENT"))
            .unwrap_or_else(|_| "dev".to_string());

        Self {
            project_dir,
            environment,
        }
    }

    fn get_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "rustyroad_query".to_string(),
                description: "Execute a SQL query against the RustyRoad database. Returns results as JSON. Use this instead of psql or direct database connections. Always use this for database queries.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "sql": {
                            "type": "string",
                            "description": "The SQL query to execute"
                        },
                        "env": {
                            "type": "string",
                            "description": "Environment to use (dev, prod, test). Defaults to current environment.",
                            "enum": ["dev", "prod", "test"]
                        }
                    },
                    "required": ["sql"]
                }),
            },
            Tool {
                name: "rustyroad_schema".to_string(),
                description: "Get the database schema (tables and columns). Use this to understand what tables exist before writing queries.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "table": {
                            "type": "string",
                            "description": "Optional: Get schema for a specific table only"
                        },
                        "env": {
                            "type": "string",
                            "description": "Environment to use (dev, prod, test)",
                            "enum": ["dev", "prod", "test"]
                        }
                    }
                }),
            },
            Tool {
                name: "rustyroad_migrate".to_string(),
                description: "Run database migrations. Use 'up' to apply pending migrations, 'down' to rollback.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "direction": {
                            "type": "string",
                            "description": "Migration direction",
                            "enum": ["up", "down", "status"]
                        },
                        "name": {
                            "type": "string",
                            "description": "Optional: Run a specific migration by name"
                        },
                        "env": {
                            "type": "string",
                            "description": "Environment to use (dev, prod, test)",
                            "enum": ["dev", "prod", "test"]
                        }
                    },
                    "required": ["direction"]
                }),
            },
            Tool {
                name: "rustyroad_migration_generate".to_string(),
                description: "Generate a new migration file with up.sql and down.sql. Use this to create database schema changes.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Migration name (e.g., 'create_users', 'add_email_to_customers')"
                        },
                        "columns": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Column definitions in format name:type[:constraints] (e.g., 'email:string:not_null,unique')"
                        }
                    },
                    "required": ["name"]
                }),
            },
            Tool {
                name: "rustyroad_config".to_string(),
                description: "Get current RustyRoad configuration including database connection info. Use this to verify which database you're connected to.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "env": {
                            "type": "string",
                            "description": "Environment to show config for",
                            "enum": ["dev", "prod", "test"]
                        }
                    }
                }),
            },
            Tool {
                name: "rustyroad_convert_migrations".to_string(),
                description: "Detect and convert rogue SQL migrations (files in ./migrations/ instead of ./config/database/migrations/) to RustyRoad format. Use this when you've created migrations in the wrong location.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "dry_run": {
                            "type": "boolean",
                            "description": "If true, only show what would be converted without making changes",
                            "default": false
                        },
                        "remove_source": {
                            "type": "boolean",
                            "description": "If true, remove original files after conversion",
                            "default": false
                        }
                    }
                }),
            },
        ]
    }

    async fn handle_tool_call(&self, name: &str, arguments: Value) -> Result<Value, String> {
        match name {
            "rustyroad_query" => self.handle_query(arguments).await,
            "rustyroad_schema" => self.handle_schema(arguments).await,
            "rustyroad_migrate" => self.handle_migrate(arguments).await,
            "rustyroad_migration_generate" => self.handle_migration_generate(arguments).await,
            "rustyroad_config" => self.handle_config(arguments),
            "rustyroad_convert_migrations" => self.handle_convert_migrations(arguments),
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }

    async fn handle_query(&self, args: Value) -> Result<Value, String> {
        let sql = args
            .get("sql")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'sql' parameter")?;

        let env = args
            .get("env")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.environment);

        // Set environment and change to project dir
        env::set_var("ENVIRONMENT", env);
        let _guard = self.change_to_project_dir()?;

        // Get database connection
        let database = Database::get_database_from_rustyroad_toml()
            .map_err(|e| format!("Failed to get database config: {}", e))?;

        let connection = Database::create_database_connection(&database)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        // Execute query
        let result = execute_query_internal(&connection, sql).await?;

        Ok(json!({
            "success": true,
            "environment": env,
            "database": database.name,
            "results": result
        }))
    }

    async fn handle_schema(&self, args: Value) -> Result<Value, String> {
        let env = args
            .get("env")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.environment);

        let table_filter = args.get("table").and_then(|v| v.as_str());

        env::set_var("ENVIRONMENT", env);
        let _guard = self.change_to_project_dir()?;

        let database = Database::get_database_from_rustyroad_toml()
            .map_err(|e| format!("Failed to get database config: {}", e))?;

        let connection = Database::create_database_connection(&database)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let schema = get_schema_internal(&connection, table_filter).await?;

        Ok(json!({
            "success": true,
            "environment": env,
            "database": database.name,
            "schema": schema
        }))
    }

    async fn handle_migrate(&self, args: Value) -> Result<Value, String> {
        let direction = args
            .get("direction")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'direction' parameter")?;

        let env = args
            .get("env")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.environment);

        let name = args.get("name").and_then(|v| v.as_str());

        env::set_var("ENVIRONMENT", env);
        let _guard = self.change_to_project_dir()?;

        match direction {
            "status" => {
                Ok(json!({
                    "success": true,
                    "message": "Use 'rustyroad migration list' to see migration status",
                    "hint": "Migration status checking via MCP coming soon"
                }))
            }
            "up" => {
                if let Some(migration_name) = name {
                    rustyroad::database::run_migration(
                        migration_name.to_string(),
                        rustyroad::database::MigrationDirection::Up,
                    )
                    .await
                    .map_err(|e| format!("Migration failed: {}", e))?;

                    Ok(json!({
                        "success": true,
                        "message": format!("Migration '{}' applied successfully", migration_name),
                        "environment": env
                    }))
                } else {
                    rustyroad::database::run_all_migrations(
                        rustyroad::database::MigrationDirection::Up,
                    )
                    .await
                    .map_err(|e| format!("Migrations failed: {}", e))?;

                    Ok(json!({
                        "success": true,
                        "message": "All pending migrations applied",
                        "environment": env
                    }))
                }
            }
            "down" => {
                if let Some(migration_name) = name {
                    rustyroad::database::run_migration(
                        migration_name.to_string(),
                        rustyroad::database::MigrationDirection::Down,
                    )
                    .await
                    .map_err(|e| format!("Rollback failed: {}", e))?;

                    Ok(json!({
                        "success": true,
                        "message": format!("Migration '{}' rolled back successfully", migration_name),
                        "environment": env
                    }))
                } else {
                    Err("Rolling back all migrations requires specifying a migration name for safety. Use 'rustyroad migration reset' CLI for full reset.".to_string())
                }
            }
            _ => Err(format!("Invalid direction: {}. Use 'up', 'down', or 'status'", direction)),
        }
    }

    async fn handle_migration_generate(&self, args: Value) -> Result<Value, String> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' parameter")?;

        let columns: Vec<String> = args
            .get("columns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let _guard = self.change_to_project_dir()?;

        rustyroad::database::create_migration(name, columns)
            .await
            .map_err(|e| format!("Failed to create migration: {}", e))?;

        Ok(json!({
            "success": true,
            "message": format!("Migration '{}' created", name),
            "location": format!("./config/database/migrations/*-{}/", name),
            "files": ["up.sql", "down.sql"],
            "next_step": "Edit the migration files if needed, then use rustyroad_migrate with direction 'up'"
        }))
    }

    fn handle_config(&self, args: Value) -> Result<Value, String> {
        let env = args
            .get("env")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.environment);

        env::set_var("ENVIRONMENT", env);
        let _guard = self.change_to_project_dir()?;

        let database = Database::get_database_from_rustyroad_toml()
            .map_err(|e| format!("Failed to get database config: {}", e))?;

        let config_file = if env == "dev" {
            "rustyroad.toml".to_string()
        } else {
            format!("rustyroad.{}.toml", env)
        };

        Ok(json!({
            "environment": env,
            "config_file": config_file,
            "project_dir": self.project_dir.display().to_string(),
            "database": {
                "name": database.name,
                "type": database.database_type.to_string().to_lowercase(),
                "host": database.host,
                "port": database.port,
                "user": database.username
            }
        }))
    }

    fn handle_convert_migrations(&self, args: Value) -> Result<Value, String> {
        let dry_run = args
            .get("dry_run")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let remove_source = args
            .get("remove_source")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let _guard = self.change_to_project_dir()?;

        let detected = rustyroad::database::detect_rogue_migrations();

        if detected.is_empty() {
            return Ok(json!({
                "success": true,
                "message": "No rogue migrations detected",
                "converted": 0
            }));
        }

        if dry_run {
            let migrations: Vec<Value> = detected
                .iter()
                .map(|m| {
                    json!({
                        "name": m.name,
                        "source": m.source_path.display().to_string(),
                        "operations": m.operations.iter().map(|op| format!("{:?}", op)).collect::<Vec<_>>()
                    })
                })
                .collect();

            return Ok(json!({
                "success": true,
                "dry_run": true,
                "message": format!("Found {} rogue migration(s) that would be converted", detected.len()),
                "migrations": migrations
            }));
        }

        let count = rustyroad::database::detect_and_convert_rogue_migrations(true, remove_source);

        Ok(json!({
            "success": true,
            "message": format!("Converted {} migration(s) to RustyRoad format", count),
            "converted": count,
            "removed_source": remove_source
        }))
    }

    fn change_to_project_dir(&self) -> Result<DirGuard, String> {
        let original = env::current_dir().map_err(|e| format!("Failed to get current dir: {}", e))?;
        env::set_current_dir(&self.project_dir)
            .map_err(|e| format!("Failed to change to project dir: {}", e))?;
        Ok(DirGuard { original })
    }

    fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let id = request.id.clone().unwrap_or(Value::Null);

        let result = match request.method.as_str() {
            "initialize" => {
                Ok(json!({
                    "protocolVersion": PROTOCOL_VERSION,
                    "serverInfo": {
                        "name": SERVER_NAME,
                        "version": SERVER_VERSION
                    },
                    "capabilities": {
                        "tools": {}
                    }
                }))
            }
            "initialized" => Ok(json!({})),
            "tools/list" => {
                let tools = self.get_tools();
                Ok(json!({ "tools": tools }))
            }
            "tools/call" => {
                let name = request
                    .params
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let arguments = request
                    .params
                    .get("arguments")
                    .cloned()
                    .unwrap_or(json!({}));

                // Use tokio runtime for async operations
                let rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(self.handle_tool_call(name, arguments)) {
                    Ok(result) => Ok(json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result).unwrap()
                        }]
                    })),
                    Err(e) => Ok(json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Error: {}", e)
                        }],
                        "isError": true
                    })),
                }
            }
            _ => Err(format!("Unknown method: {}", request.method)),
        };

        match result {
            Ok(r) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(r),
                error: None,
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: e,
                    data: None,
                }),
            },
        }
    }
}

/// Guard to restore directory on drop
struct DirGuard {
    original: PathBuf,
}

impl Drop for DirGuard {
    fn drop(&mut self) {
        let _ = env::set_current_dir(&self.original);
    }
}

/// Execute a query and return results as JSON
async fn execute_query_internal(
    connection: &DatabaseConnection,
    sql: &str,
) -> Result<Value, String> {
    match connection {
        DatabaseConnection::Pg(pool) => {
            let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(sql)
                .fetch_all(pool.as_ref())
                .await
                .map_err(|e| format!("Query failed: {}", e))?;

            let mut results = Vec::new();
            for row in rows {
                let mut row_map = serde_json::Map::new();
                for (i, column) in row.columns().iter().enumerate() {
                    let value: Value = if let Ok(v) = row.try_get::<String, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<i32, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<i64, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<f64, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<bool, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<serde_json::Value, _>(i) {
                        v
                    } else {
                        json!(null)
                    };
                    row_map.insert(column.name().to_string(), value);
                }
                results.push(Value::Object(row_map));
            }
            Ok(json!(results))
        }
        DatabaseConnection::Sqlite(pool) => {
            let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(sql)
                .fetch_all(pool.as_ref())
                .await
                .map_err(|e| format!("Query failed: {}", e))?;

            let mut results = Vec::new();
            for row in rows {
                let mut row_map = serde_json::Map::new();
                for (i, column) in row.columns().iter().enumerate() {
                    let value: Value = if let Ok(v) = row.try_get::<String, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<i32, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<i64, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<f64, _>(i) {
                        json!(v)
                    } else {
                        json!(null)
                    };
                    row_map.insert(column.name().to_string(), value);
                }
                results.push(Value::Object(row_map));
            }
            Ok(json!(results))
        }
        DatabaseConnection::MySql(pool) => {
            let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(sql)
                .fetch_all(pool.as_ref())
                .await
                .map_err(|e| format!("Query failed: {}", e))?;

            let mut results = Vec::new();
            for row in rows {
                let mut row_map = serde_json::Map::new();
                for (i, column) in row.columns().iter().enumerate() {
                    let value: Value = if let Ok(v) = row.try_get::<String, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<i32, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<i64, _>(i) {
                        json!(v)
                    } else if let Ok(v) = row.try_get::<f64, _>(i) {
                        json!(v)
                    } else {
                        json!(null)
                    };
                    row_map.insert(column.name().to_string(), value);
                }
                results.push(Value::Object(row_map));
            }
            Ok(json!(results))
        }
    }
}

/// Get database schema as JSON
async fn get_schema_internal(
    connection: &DatabaseConnection,
    table_filter: Option<&str>,
) -> Result<Value, String> {
    let schema_query = match connection {
        DatabaseConnection::Pg(_) => {
            if let Some(table) = table_filter {
                format!(
                    "SELECT table_name, column_name, data_type, is_nullable, column_default 
                     FROM information_schema.columns 
                     WHERE table_schema = 'public' AND table_name = '{}'
                     ORDER BY table_name, ordinal_position",
                    table
                )
            } else {
                "SELECT table_name, column_name, data_type, is_nullable, column_default 
                 FROM information_schema.columns 
                 WHERE table_schema = 'public'
                 ORDER BY table_name, ordinal_position".to_string()
            }
        }
        DatabaseConnection::Sqlite(_) => {
            "SELECT name as table_name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'".to_string()
        }
        DatabaseConnection::MySql(_) => {
            if let Some(table) = table_filter {
                format!(
                    "SELECT TABLE_NAME as table_name, COLUMN_NAME as column_name, DATA_TYPE as data_type, IS_NULLABLE as is_nullable, COLUMN_DEFAULT as column_default 
                     FROM INFORMATION_SCHEMA.COLUMNS 
                     WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = '{}'
                     ORDER BY TABLE_NAME, ORDINAL_POSITION",
                    table
                )
            } else {
                "SELECT TABLE_NAME as table_name, COLUMN_NAME as column_name, DATA_TYPE as data_type, IS_NULLABLE as is_nullable, COLUMN_DEFAULT as column_default 
                 FROM INFORMATION_SCHEMA.COLUMNS 
                 WHERE TABLE_SCHEMA = DATABASE()
                 ORDER BY TABLE_NAME, ORDINAL_POSITION".to_string()
            }
        }
    };

    execute_query_internal(connection, &schema_query).await
}

/// Register the MCP server with OpenCode
fn register_with_opencode() -> Result<(), String> {
    let config_path = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("opencode")
        .join("opencode.json");

    // Read existing config or create new one
    let mut config: Value = if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?
    } else {
        json!({
            "$schema": "https://opencode.ai/config.json"
        })
    };

    // Get the path to the rustyroad-mcp binary
    let binary_path = env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;

    // Add/update MCP server config
    let mcp = config
        .as_object_mut()
        .ok_or("Config is not an object")?
        .entry("mcp")
        .or_insert(json!({}));

    let mcp_obj = mcp
        .as_object_mut()
        .ok_or("MCP config is not an object")?;

    mcp_obj.insert(
        "rustyroad".to_string(),
        json!({
            "type": "local",
            "command": [binary_path.display().to_string()],
            "enabled": true
        }),
    );

    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // Write config back
    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&config_path, content)
        .map_err(|e| format!("Failed to write config: {}", e))?;

    println!("Registered RustyRoad MCP server with OpenCode!");
    println!("Config file: {}", config_path.display());
    println!("\nThe following tools are now available:");
    println!("  - rustyroad_query: Execute SQL queries");
    println!("  - rustyroad_schema: Get database schema");
    println!("  - rustyroad_migrate: Run migrations");
    println!("  - rustyroad_migration_generate: Create new migrations");
    println!("  - rustyroad_config: View configuration");
    println!("  - rustyroad_convert_migrations: Fix rogue migrations");
    println!("\nRestart OpenCode to use the new MCP server.");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for --register flag
    if args.iter().any(|a| a == "--register" || a == "-r") {
        match register_with_opencode() {
            Ok(()) => std::process::exit(0),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Check for --help flag
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("RustyRoad MCP Server v{}", SERVER_VERSION);
        println!();
        println!("USAGE:");
        println!("    rustyroad-mcp [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -r, --register    Register with OpenCode (~/.config/opencode/opencode.json)");
        println!("    -h, --help        Print help information");
        println!();
        println!("ENVIRONMENT VARIABLES:");
        println!("    RUSTYROAD_PROJECT_DIR    Path to RustyRoad project (default: current dir)");
        println!("    ENV, ENVIRONMENT         Database environment (dev, prod, test)");
        println!();
        println!("When run without --register, starts the MCP server (stdio mode).");
        std::process::exit(0);
    }

    // Run MCP server
    let server = McpServer::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    eprintln!("RustyRoad MCP Server v{} started", SERVER_VERSION);
    eprintln!("Project dir: {:?}", server.project_dir);
    eprintln!("Environment: {}", server.environment);

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                let error_response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: Value::Null,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                        data: None,
                    }),
                };
                let _ = writeln!(stdout, "{}", serde_json::to_string(&error_response).unwrap());
                let _ = stdout.flush();
                continue;
            }
        };

        let response = server.handle_request(request);
        let response_str = serde_json::to_string(&response).unwrap();
        let _ = writeln!(stdout, "{}", response_str);
        let _ = stdout.flush();
    }
}
