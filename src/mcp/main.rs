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

use regex::Regex;
use rustyroad::database::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Column, Row};
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;

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
            Tool {
                name: "rustyroad_project_info".to_string(),
                description: "Get RustyRoad project structure and configuration. Returns project directories, rustyroad.toml config, available migrations, and src/ structure (controllers, models, views). Use this to understand the project layout.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            Tool {
                name: "rustyroad_routes".to_string(),
                description: "Scan src/controllers/ for Actix route definitions and return all routes as structured JSON. Parses #[get], #[post], #[put], #[delete] attributes and .route()/.service() calls.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "controller": {
                            "type": "string",
                            "description": "Optional: Filter to a specific controller file (e.g., 'user' or 'user.rs')"
                        }
                    }
                }),
            },
            Tool {
                name: "rustyroad_build".to_string(),
                description: "Build the RustyRoad project using cargo. Returns structured JSON with success/failure status, errors with file:line:col locations, and warnings. Use check_only=true for faster feedback (type checking only).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "release": {
                            "type": "boolean",
                            "description": "Build in release mode (optimized). Default: false",
                            "default": false
                        },
                        "check_only": {
                            "type": "boolean",
                            "description": "Run 'cargo check' instead of 'cargo build' for faster feedback. Default: false",
                            "default": false
                        }
                    }
                }),
            },
            Tool {
                name: "rustyroad_test".to_string(),
                description: "Run cargo test in the project directory. Returns structured JSON with test results including passed/failed/ignored counts, names of failed tests, and error messages.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "filter": {
                            "type": "string",
                            "description": "Optional filter to run specific tests (passed to cargo test as argument)"
                        }
                    }
                }),
            },
            Tool {
                name: "rustyroad_models".to_string(),
                description: "Scan src/models/ directory for Rust struct definitions. Returns struct names and their fields (name and type) as structured JSON. Use this to understand the data models in the project.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            Tool {
                name: "rustyroad_recent_changes".to_string(),
                description: "Get recent git changes in the project. Shows recent commits, uncommitted changes, and modified files summary. Useful for understanding what changed recently.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "limit": {
                            "type": "integer",
                            "description": "Number of recent commits to show (default: 10)",
                            "default": 10
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
            "rustyroad_project_info" => self.handle_project_info().await,
            "rustyroad_routes" => self.handle_routes(arguments).await,
            "rustyroad_build" => self.handle_build(arguments).await,
            "rustyroad_test" => self.handle_test(arguments).await,
            "rustyroad_models" => self.handle_models().await,
            "rustyroad_recent_changes" => self.handle_recent_changes(arguments).await,
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

    async fn handle_build(&self, args: Value) -> Result<Value, String> {
        let release = args
            .get("release")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let check_only = args
            .get("check_only")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let _guard = self.change_to_project_dir()?;

        // Build the cargo command
        let cargo_cmd = if check_only { "check" } else { "build" };
        let mut cmd = Command::new("cargo");
        cmd.arg(cargo_cmd);
        cmd.arg("--message-format=json");

        if release {
            cmd.arg("--release");
        }

        // Run cargo and capture output
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to run cargo: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Parse the JSON output from cargo
        let mut errors: Vec<Value> = Vec::new();
        let mut warnings: Vec<Value> = Vec::new();
        let mut other_messages: Vec<String> = Vec::new();

        for line in stdout.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(msg) = serde_json::from_str::<Value>(line) {
                if let Some(reason) = msg.get("reason").and_then(|v| v.as_str()) {
                    if reason == "compiler-message" {
                        if let Some(message) = msg.get("message") {
                            let level = message
                                .get("level")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");

                            let rendered = message
                                .get("rendered")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");

                            let msg_text = message
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");

                            // Extract span information (file:line:col)
                            let mut spans: Vec<Value> = Vec::new();
                            if let Some(spans_arr) = message.get("spans").and_then(|v| v.as_array()) {
                                for span in spans_arr {
                                    let file = span
                                        .get("file_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    let line_start = span
                                        .get("line_start")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0);
                                    let line_end = span
                                        .get("line_end")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0);
                                    let col_start = span
                                        .get("column_start")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0);
                                    let col_end = span
                                        .get("column_end")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0);
                                    let is_primary = span
                                        .get("is_primary")
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false);
                                    let label = span
                                        .get("label")
                                        .and_then(|v| v.as_str())
                                        .map(String::from);

                                    spans.push(json!({
                                        "file": file,
                                        "line_start": line_start,
                                        "line_end": line_end,
                                        "col_start": col_start,
                                        "col_end": col_end,
                                        "is_primary": is_primary,
                                        "label": label,
                                        "location": format!("{}:{}:{}", file, line_start, col_start)
                                    }));
                                }
                            }

                            let error_code = message
                                .get("code")
                                .and_then(|v| v.get("code"))
                                .and_then(|v| v.as_str())
                                .map(String::from);

                            let diagnostic = json!({
                                "level": level,
                                "message": msg_text,
                                "code": error_code,
                                "spans": spans,
                                "rendered": rendered
                            });

                            match level {
                                "error" => errors.push(diagnostic),
                                "warning" => warnings.push(diagnostic),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        // Also capture any non-JSON stderr output (like linker errors)
        let stderr_re = Regex::new(r"error(?:\[E\d+\])?:").unwrap();
        for line in stderr.lines() {
            if stderr_re.is_match(line) {
                other_messages.push(line.to_string());
            }
        }

        let success = output.status.success();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok(json!({
            "success": success,
            "command": if check_only { "cargo check" } else { "cargo build" },
            "release": release,
            "exit_code": exit_code,
            "error_count": errors.len(),
            "warning_count": warnings.len(),
            "errors": errors,
            "warnings": warnings,
            "other_messages": other_messages,
            "summary": if success {
                format!("Build succeeded with {} warning(s)", warnings.len())
            } else {
                format!("Build failed with {} error(s) and {} warning(s)", errors.len(), warnings.len())
            }
        }))
    }

    async fn handle_project_info(&self) -> Result<Value, String> {
        let _guard = self.change_to_project_dir()?;

        // Parse rustyroad.toml config
        let config = self.parse_rustyroad_toml();

        // List top-level directories and files
        let root_structure = self.list_directory_contents(&self.project_dir);

        // List available migrations
        let migrations = self.list_migrations();

        // List src/ structure with subdirectories
        let src_path = self.project_dir.join("src");
        let src_structure = if src_path.exists() {
            self.get_src_structure(&src_path)
        } else {
            json!({ "exists": false })
        };

        Ok(json!({
            "success": true,
            "project_dir": self.project_dir.display().to_string(),
            "environment": self.environment,
            "config": config,
            "root_structure": root_structure,
            "migrations": migrations,
            "src_structure": src_structure
        }))
    }

    fn parse_rustyroad_toml(&self) -> Value {
        let toml_path = self.project_dir.join("rustyroad.toml");
        if !toml_path.exists() {
            return json!({ "exists": false, "error": "rustyroad.toml not found" });
        }

        match fs::read_to_string(&toml_path) {
            Ok(content) => {
                match toml::from_str::<toml::Value>(&content) {
                    Ok(parsed) => json!({
                        "exists": true,
                        "content": parsed
                    }),
                    Err(e) => json!({
                        "exists": true,
                        "raw": content,
                        "parse_error": e.to_string()
                    })
                }
            }
            Err(e) => json!({
                "exists": true,
                "error": format!("Failed to read: {}", e)
            })
        }
    }

    fn list_directory_contents(&self, path: &PathBuf) -> Value {
        let mut directories: Vec<String> = Vec::new();
        let mut files: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                // Skip hidden files/dirs
                if name.starts_with('.') {
                    continue;
                }
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        directories.push(name);
                    } else {
                        files.push(name);
                    }
                }
            }
        }

        directories.sort();
        files.sort();

        json!({
            "directories": directories,
            "files": files
        })
    }

    fn list_migrations(&self) -> Value {
        let migrations_path = self.project_dir.join("config/database/migrations");
        if !migrations_path.exists() {
            return json!({
                "path": "config/database/migrations",
                "exists": false,
                "migrations": []
            });
        }

        let mut migrations: Vec<Value> = Vec::new();

        if let Ok(entries) = fs::read_dir(&migrations_path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let migration_path = entry.path();

                        let has_up = migration_path.join("up.sql").exists();
                        let has_down = migration_path.join("down.sql").exists();

                        migrations.push(json!({
                            "name": name,
                            "has_up_sql": has_up,
                            "has_down_sql": has_down
                        }));
                    }
                }
            }
        }

        // Sort by name (which includes timestamp)
        migrations.sort_by(|a, b| {
            let name_a = a.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let name_b = b.get("name").and_then(|v| v.as_str()).unwrap_or("");
            name_a.cmp(name_b)
        });

        json!({
            "path": "config/database/migrations",
            "exists": true,
            "count": migrations.len(),
            "migrations": migrations
        })
    }

    fn get_src_structure(&self, src_path: &PathBuf) -> Value {
        let mut structure = serde_json::Map::new();
        structure.insert("exists".to_string(), json!(true));

        // List top-level src/ contents
        let top_level = self.list_directory_contents(src_path);
        structure.insert("contents".to_string(), top_level);

        // Get details for key subdirectories
        let key_dirs = ["controllers", "models", "views"];

        for dir_name in key_dirs {
            let dir_path = src_path.join(dir_name);
            if dir_path.exists() {
                structure.insert(dir_name.to_string(), self.get_subdirectory_structure(&dir_path));
            }
        }

        Value::Object(structure)
    }

    fn get_subdirectory_structure(&self, path: &PathBuf) -> Value {
        let mut result = serde_json::Map::new();
        let mut files: Vec<String> = Vec::new();
        let mut subdirs: serde_json::Map<String, Value> = serde_json::Map::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    continue;
                }
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        // Get files in subdirectory
                        let subdir_files = self.list_files_in_dir(&entry.path());
                        subdirs.insert(name, json!(subdir_files));
                    } else {
                        files.push(name);
                    }
                }
            }
        }

        files.sort();
        result.insert("files".to_string(), json!(files));

        if !subdirs.is_empty() {
            result.insert("subdirectories".to_string(), Value::Object(subdirs));
        }

        Value::Object(result)
    }

    fn list_files_in_dir(&self, path: &PathBuf) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    continue;
                }
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        files.push(name);
                    }
                }
            }
        }

        files.sort();
        files
    }

    async fn handle_routes(&self, args: Value) -> Result<Value, String> {
        let controller_filter = args.get("controller").and_then(|v| v.as_str());

        let _guard = self.change_to_project_dir()?;

        let controllers_dir = self.project_dir.join("src").join("controllers");
        if !controllers_dir.exists() {
            return Err(format!(
                "Controllers directory not found: {}",
                controllers_dir.display()
            ));
        }

        let mut routes: Vec<Value> = Vec::new();

        // Recursively find all .rs files in src/controllers/
        fn collect_rs_files(dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<(), String> {
            let entries = fs::read_dir(dir)
                .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
                let path = entry.path();
                if path.is_dir() {
                    collect_rs_files(&path, files)?;
                } else if path.extension().map_or(false, |ext| ext == "rs") {
                    // Skip mod.rs files as they typically don't contain routes
                    if path.file_name().map_or(false, |n| n != "mod.rs") {
                        files.push(path);
                    }
                }
            }
            Ok(())
        }

        let mut rs_files: Vec<PathBuf> = Vec::new();
        collect_rs_files(&controllers_dir, &mut rs_files)?;

        // Filter by controller name if specified
        if let Some(filter) = controller_filter {
            let filter_name = filter.trim_end_matches(".rs");
            rs_files.retain(|path| {
                path.file_stem()
                    .map_or(false, |stem| stem.to_string_lossy().contains(filter_name))
            });
        }

        // Regex patterns for Actix route attributes
        // Matches: #[get("/path")], #[post("/path")], etc.
        let attr_route_re = Regex::new(r#"#\[(get|post|put|delete|patch|head|options)\("([^"]+)"\)\]"#)
            .map_err(|e| format!("Regex error: {}", e))?;

        // Matches: .route("/path", web::get().to(...))
        let route_method_re = Regex::new(r#"\.route\(\s*"([^"]+)"\s*,\s*web::(get|post|put|delete|patch|head|options)\(\)"#)
            .map_err(|e| format!("Regex error: {}", e))?;

        // Matches: web::resource("/path").route(web::get().to(...))
        let resource_re = Regex::new(r#"web::resource\(\s*"([^"]+)"\s*\)"#)
            .map_err(|e| format!("Regex error: {}", e))?;

        // Matches: web::scope("/prefix")
        let scope_re = Regex::new(r#"web::scope\(\s*"([^"]+)"\s*\)"#)
            .map_err(|e| format!("Regex error: {}", e))?;

        // Matches function name after route attribute
        let fn_name_re = Regex::new(r#"(?:pub\s+)?async\s+fn\s+(\w+)"#)
            .map_err(|e| format!("Regex error: {}", e))?;

        for file_path in rs_files {
            let content = fs::read_to_string(&file_path)
                .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

            let relative_path = file_path
                .strip_prefix(&self.project_dir)
                .unwrap_or(&file_path)
                .display()
                .to_string();

            let lines: Vec<&str> = content.lines().collect();

            // Parse attribute-based routes (#[get("/path")])
            for (line_num, line) in lines.iter().enumerate() {
                if let Some(caps) = attr_route_re.captures(line) {
                    let method = caps.get(1).map_or("", |m| m.as_str()).to_uppercase();
                    let path = caps.get(2).map_or("", |m| m.as_str());

                    // Look for the function name in the next few lines
                    let mut handler = String::new();
                    for i in 1..=5 {
                        if line_num + i < lines.len() {
                            if let Some(fn_caps) = fn_name_re.captures(lines[line_num + i]) {
                                handler = fn_caps.get(1).map_or("", |m| m.as_str()).to_string();
                                break;
                            }
                        }
                    }

                    routes.push(json!({
                        "method": method,
                        "path": path,
                        "handler": handler,
                        "file": relative_path,
                        "line": line_num + 1,
                        "type": "attribute"
                    }));
                }
            }

            // Parse .route() calls
            for caps in route_method_re.captures_iter(&content) {
                let path = caps.get(1).map_or("", |m| m.as_str());
                let method = caps.get(2).map_or("", |m| m.as_str()).to_uppercase();

                routes.push(json!({
                    "method": method,
                    "path": path,
                    "handler": "",
                    "file": relative_path,
                    "line": 0,
                    "type": "route_call"
                }));
            }

            // Parse web::resource() calls
            for caps in resource_re.captures_iter(&content) {
                let path = caps.get(1).map_or("", |m| m.as_str());

                routes.push(json!({
                    "method": "RESOURCE",
                    "path": path,
                    "handler": "",
                    "file": relative_path,
                    "line": 0,
                    "type": "resource"
                }));
            }

            // Parse web::scope() calls
            for caps in scope_re.captures_iter(&content) {
                let path = caps.get(1).map_or("", |m| m.as_str());

                routes.push(json!({
                    "method": "SCOPE",
                    "path": path,
                    "handler": "",
                    "file": relative_path,
                    "line": 0,
                    "type": "scope"
                }));
            }
        }

        Ok(json!({
            "success": true,
            "route_count": routes.len(),
            "routes": routes
        }))
    }

    fn change_to_project_dir(&self) -> Result<DirGuard, String> {
        let original = env::current_dir().map_err(|e| format!("Failed to get current dir: {}", e))?;
        env::set_current_dir(&self.project_dir)
            .map_err(|e| format!("Failed to change to project dir: {}", e))?;
        Ok(DirGuard { original })
    }

    async fn handle_test(&self, args: Value) -> Result<Value, String> {
        let filter = args.get("filter").and_then(|v| v.as_str());

        let _guard = self.change_to_project_dir()?;

        // Build cargo test command
        let mut cmd = Command::new("cargo");
        cmd.arg("test");

        // Add filter if provided
        if let Some(f) = filter {
            cmd.arg(f);
        }

        // Add -- --nocapture to get full output
        cmd.arg("--").arg("--nocapture");

        // Run the command
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to run cargo test: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let combined_output = format!("{}\n{}", stdout, stderr);

        // Parse test results using regex
        let mut passed = 0;
        let mut failed = 0;
        let mut ignored = 0;
        let mut failed_tests: Vec<Value> = Vec::new();

        // Regex patterns for parsing cargo test output
        // Match summary line: "test result: ok. X passed; Y failed; Z ignored"
        let summary_re = Regex::new(r"test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored")
            .map_err(|e| format!("Failed to compile regex: {}", e))?;

        // Match failed test names: "---- test_name stdout ----" followed by error
        let failed_test_re = Regex::new(r"---- ([^\s]+) stdout ----")
            .map_err(|e| format!("Failed to compile regex: {}", e))?;

        // Match individual test results: "test module::test_name ... FAILED"
        let test_result_re = Regex::new(r"test ([^\s]+) \.\.\. (ok|FAILED|ignored)")
            .map_err(|e| format!("Failed to compile regex: {}", e))?;

        // Parse summary
        if let Some(caps) = summary_re.captures(&combined_output) {
            passed = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap_or(0));
            failed = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
            ignored = caps.get(3).map_or(0, |m| m.as_str().parse().unwrap_or(0));
        }

        // Find failed tests and their error messages
        let mut current_failed_test: Option<String> = None;
        let mut current_error: Vec<String> = Vec::new();
        let mut in_failure_block = false;

        for line in combined_output.lines() {
            // Check for start of failure output
            if let Some(caps) = failed_test_re.captures(line) {
                // Save previous failed test if any
                if let Some(test_name) = current_failed_test.take() {
                    failed_tests.push(json!({
                        "name": test_name,
                        "error": current_error.join("\n")
                    }));
                    current_error.clear();
                }
                current_failed_test = Some(caps.get(1).unwrap().as_str().to_string());
                in_failure_block = true;
                continue;
            }

            // Check for end of failure block
            if in_failure_block && (line.starts_with("---- ") || line.starts_with("failures:") || line.starts_with("test result:")) {
                if let Some(test_name) = current_failed_test.take() {
                    failed_tests.push(json!({
                        "name": test_name,
                        "error": current_error.join("\n")
                    }));
                    current_error.clear();
                }
                in_failure_block = line.starts_with("---- ");
                if in_failure_block {
                    if let Some(caps) = failed_test_re.captures(line) {
                        current_failed_test = Some(caps.get(1).unwrap().as_str().to_string());
                    }
                }
                continue;
            }

            // Collect error message lines
            if in_failure_block && current_failed_test.is_some() {
                current_error.push(line.to_string());
            }
        }

        // Don't forget the last failed test
        if let Some(test_name) = current_failed_test.take() {
            failed_tests.push(json!({
                "name": test_name,
                "error": current_error.join("\n")
            }));
        }

        // If we couldn't parse any summary, try to count from individual results
        if passed == 0 && failed == 0 && ignored == 0 {
            for caps in test_result_re.captures_iter(&combined_output) {
                match caps.get(2).map(|m| m.as_str()) {
                    Some("ok") => passed += 1,
                    Some("FAILED") => failed += 1,
                    Some("ignored") => ignored += 1,
                    _ => {}
                }
            }
        }

        let success = output.status.success() && failed == 0;

        Ok(json!({
            "success": success,
            "summary": {
                "passed": passed,
                "failed": failed,
                "ignored": ignored,
                "total": passed + failed + ignored
            },
            "failed_tests": failed_tests,
            "stdout": stdout,
            "stderr": stderr
        }))
    }

    async fn handle_models(&self) -> Result<Value, String> {
        let _guard = self.change_to_project_dir()?;

        let models_dir = self.project_dir.join("src").join("models");

        if !models_dir.exists() {
            return Ok(json!({
                "success": true,
                "message": "No src/models/ directory found",
                "models": []
            }));
        }

        // Regex patterns for parsing Rust structs
        // Match struct definition: "pub struct Name {" or "struct Name {"
        let struct_re = Regex::new(r"(?:pub\s+)?struct\s+(\w+)\s*\{")
            .map_err(|e| format!("Failed to compile struct regex: {}", e))?;

        // Match field definition: "field_name: Type," or "pub field_name: Type,"
        let field_re = Regex::new(r"(?:pub\s+)?(\w+)\s*:\s*([^,\n]+)[,\n]")
            .map_err(|e| format!("Failed to compile field regex: {}", e))?;

        let mut models: Vec<Value> = Vec::new();

        // Read all .rs files in src/models/
        let entries = fs::read_dir(&models_dir)
            .map_err(|e| format!("Failed to read models directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            // Skip non-.rs files and mod.rs
            if path.extension().map_or(true, |ext| ext != "rs") {
                continue;
            }

            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            // Read file content
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {}: {}", file_name, e))?;

            // Find all struct definitions
            let mut file_structs: Vec<Value> = Vec::new();

            for cap in struct_re.captures_iter(&content) {
                let struct_name = cap.get(1).unwrap().as_str().to_string();
                let match_end = cap.get(0).unwrap().end();

                // Find the struct body by counting braces
                let mut brace_count = 1;
                let body_start = match_end;
                let mut body_end = match_end;

                let remaining = &content[match_end..];
                for (i, c) in remaining.chars().enumerate() {
                    match c {
                        '{' => brace_count += 1,
                        '}' => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                body_end = match_end + i;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                let struct_body = &content[body_start..body_end];

                // Parse fields from the struct body
                let mut fields: Vec<Value> = Vec::new();
                for field_cap in field_re.captures_iter(struct_body) {
                    let field_name = field_cap.get(1).unwrap().as_str().trim().to_string();
                    let field_type = field_cap.get(2).unwrap().as_str().trim().to_string();

                    // Skip derive attributes or other non-field content
                    if field_name.starts_with('#') || field_name.is_empty() {
                        continue;
                    }

                    fields.push(json!({
                        "name": field_name,
                        "type": field_type
                    }));
                }

                file_structs.push(json!({
                    "name": struct_name,
                    "fields": fields
                }));
            }

            if !file_structs.is_empty() {
                models.push(json!({
                    "file": file_name,
                    "path": path.display().to_string(),
                    "structs": file_structs
                }));
            }
        }

        Ok(json!({
            "success": true,
            "models_dir": models_dir.display().to_string(),
            "file_count": models.len(),
            "models": models
        }))
    }

    async fn handle_recent_changes(&self, args: Value) -> Result<Value, String> {
        let limit = args
            .get("limit")
            .and_then(|v| v.as_i64())
            .unwrap_or(10) as usize;

        let _guard = self.change_to_project_dir()?;

        // Get recent commits
        let log_output = Command::new("git")
            .args([
                "log",
                &format!("-{}", limit),
                "--pretty=format:%H|%s|%an|%ai",
            ])
            .current_dir(&self.project_dir)
            .output()
            .map_err(|e| format!("Failed to run git log: {}", e))?;

        let log_stdout = String::from_utf8_lossy(&log_output.stdout);
        let commits: Vec<Value> = log_stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.splitn(4, '|').collect();
                json!({
                    "hash": parts.first().unwrap_or(&""),
                    "message": parts.get(1).unwrap_or(&""),
                    "author": parts.get(2).unwrap_or(&""),
                    "date": parts.get(3).unwrap_or(&"")
                })
            })
            .collect();

        // Get git status for uncommitted changes
        let status_output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&self.project_dir)
            .output()
            .map_err(|e| format!("Failed to run git status: {}", e))?;

        let status_stdout = String::from_utf8_lossy(&status_output.stdout);
        let mut staged_files: Vec<String> = Vec::new();
        let mut unstaged_files: Vec<String> = Vec::new();
        let mut untracked_files: Vec<String> = Vec::new();

        for line in status_stdout.lines() {
            if line.len() < 3 {
                continue;
            }
            let index_status = line.chars().next().unwrap_or(' ');
            let worktree_status = line.chars().nth(1).unwrap_or(' ');
            let file_path = line[3..].to_string();

            match (index_status, worktree_status) {
                ('?', '?') => untracked_files.push(file_path),
                (_, ' ') if index_status != ' ' => staged_files.push(file_path),
                (' ', _) if worktree_status != ' ' => unstaged_files.push(file_path),
                (i, w) if i != ' ' && w != ' ' => {
                    staged_files.push(file_path.clone());
                    unstaged_files.push(file_path);
                }
                _ => {}
            }
        }

        // Get diff stat for modified files summary
        let diff_output = Command::new("git")
            .args(["diff", "--stat", "HEAD"])
            .current_dir(&self.project_dir)
            .output()
            .map_err(|e| format!("Failed to run git diff: {}", e))?;

        let diff_stdout = String::from_utf8_lossy(&diff_output.stdout);
        let mut modified_files: Vec<Value> = Vec::new();

        for line in diff_stdout.lines() {
            // Parse lines like: " src/main.rs | 10 ++++-----"
            if line.contains('|') && !line.contains("file") {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 2 {
                    let file = parts[0].trim().to_string();
                    let changes = parts[1].trim().to_string();
                    modified_files.push(json!({
                        "file": file,
                        "changes": changes
                    }));
                }
            }
        }

        Ok(json!({
            "success": true,
            "project_dir": self.project_dir.display().to_string(),
            "recent_commits": commits,
            "uncommitted_changes": {
                "staged": staged_files,
                "unstaged": unstaged_files,
                "untracked": untracked_files
            },
            "modified_files_summary": modified_files
        }))
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
