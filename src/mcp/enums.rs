//! MCP `rustyroad_enums` tool: enum types and their allowed values.

use super::{McpServer, Tool};
use serde_json::{json, Value};
use std::env;

/// Returns the `rustyroad_enums` tool catalog entry.
pub(super) fn tool_definition() -> Tool {
    Tool {
        name: "rustyroad_enums".to_string(),
        description: "Get the database enum types and their allowed values. Use this before writing queries or validations that involve enum columns.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "env": {
                    "type": "string",
                    "description": "Environment to use (dev, prod, test)",
                    "enum": ["dev", "prod", "test"]
                }
            }
        }),
    }
}

impl McpServer {
    /// Handles `tools/call` for `rustyroad_enums`.
    ///
    /// Reuses the library's enum reader so the MCP tool and the `db enums`
    /// CLI report the same data from one implementation.
    pub(super) async fn handle_enums(&self, args: Value) -> Result<Value, String> {
        let env = args
            .get("env")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.environment);

        env::set_var("ENVIRONMENT", env);
        let _guard = self.change_to_project_dir()?;

        let output = rustyroad::database::enums::read_enums()
            .await
            .map_err(|e| format!("Failed to read enums: {}", e))?;

        let enums: Vec<Value> = output
            .enums
            .iter()
            .map(|e| {
                json!({
                    "name": e.name,
                    "values": e.values,
                    "table": e.table,
                    "column": e.column,
                })
            })
            .collect();

        Ok(json!({
            "success": true,
            "environment": env,
            "database": output.database_type,
            "enums": enums
        }))
    }
}
