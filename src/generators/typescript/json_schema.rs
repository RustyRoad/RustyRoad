//! Rendering the supported JSON Schema subset into generated TypeScript assets.
//!
//! The schema itself lives in PostgreSQL column metadata. These renderers keep
//! Drizzle, Zod, and OpenAPI on that single source of truth.

use serde_json::Value;
use std::collections::BTreeSet;

/// Renders a JSON Schema as a TypeScript type expression.
pub(super) fn typescript(schema: &Value) -> String {
    if let Some(values) = variants(schema) {
        return union(values.iter().map(typescript).collect());
    }
    if let Some(value) = schema.get("const") {
        return literal(value);
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        return union(values.iter().map(literal).collect());
    }

    match schema.get("type") {
        Some(Value::Array(types)) => union(
            types
                .iter()
                .filter_map(Value::as_str)
                .map(|kind| typescript_kind(kind, schema))
                .collect(),
        ),
        Some(Value::String(kind)) => typescript_kind(kind, schema),
        _ if schema.get("properties").is_some() => object_type(schema),
        _ => "unknown".to_string(),
    }
}

/// Renders a JSON Schema as a Zod expression.
pub(super) fn zod(schema: &Value) -> String {
    if let Some(values) = variants(schema) {
        return zod_union(values.iter().map(zod).collect());
    }
    if let Some(value) = schema.get("const") {
        return format!("z.literal({})", literal(value));
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        return zod_union(
            values
                .iter()
                .map(|value| format!("z.literal({})", literal(value)))
                .collect(),
        );
    }

    match schema.get("type") {
        Some(Value::Array(types)) => zod_union(
            types
                .iter()
                .filter_map(Value::as_str)
                .map(|kind| zod_kind(kind, schema))
                .collect(),
        ),
        Some(Value::String(kind)) => zod_kind(kind, schema),
        _ if schema.get("properties").is_some() => zod_object(schema),
        _ => "z.unknown()".to_string(),
    }
}

/// Renders the original schema for OpenAPI 3.1, adding SQL nullability.
pub(super) fn openapi(schema: &Value, nullable: bool) -> String {
    let value = if nullable && !accepts_null(schema) {
        serde_json::json!({ "anyOf": [schema, { "type": "null" }] })
    } else {
        schema.clone()
    };
    serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string())
}

fn variants(schema: &Value) -> Option<&Vec<Value>> {
    schema
        .get("oneOf")
        .or_else(|| schema.get("anyOf"))
        .and_then(Value::as_array)
}

fn typescript_kind(kind: &str, schema: &Value) -> String {
    match kind {
        "object" => object_type(schema),
        "array" => format!(
            "Array<{}>",
            schema
                .get("items")
                .map(typescript)
                .unwrap_or_else(|| "unknown".to_string())
        ),
        "string" => "string".to_string(),
        "number" | "integer" => "number".to_string(),
        "boolean" => "boolean".to_string(),
        "null" => "null".to_string(),
        _ => "unknown".to_string(),
    }
}

fn object_type(schema: &Value) -> String {
    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let required = required(schema);
    let mut fields = properties
        .iter()
        .map(|(name, property)| {
            let optional = if required.contains(name) { "" } else { "?" };
            format!("{}{}: {}", quoted(name), optional, typescript(property))
        })
        .collect::<Vec<_>>();

    match schema.get("additionalProperties") {
        Some(Value::Bool(false)) => {}
        Some(value) if value.is_object() => {
            if properties.is_empty() {
                fields.push(format!("[key: string]: {}", typescript(value)));
            } else {
                // TypeScript index signatures also constrain named properties,
                // unlike JSON Schema's additional-properties-only semantics.
                fields.push("[key: string]: unknown".to_string());
            }
        }
        _ => fields.push("[key: string]: unknown".to_string()),
    }

    if properties.is_empty() && fields.len() == 1 {
        return match schema.get("additionalProperties") {
            Some(value) if value.is_object() => format!("Record<string, {}>", typescript(value)),
            _ => "Record<string, unknown>".to_string(),
        };
    }
    format!("{{ {} }}", fields.join("; "))
}

fn zod_kind(kind: &str, schema: &Value) -> String {
    match kind {
        "object" => zod_object(schema),
        "array" => format!(
            "z.array({})",
            schema
                .get("items")
                .map(zod)
                .unwrap_or_else(|| "z.unknown()".to_string())
        ),
        "string" => "z.string()".to_string(),
        "number" => "z.number()".to_string(),
        "integer" => "z.number().int()".to_string(),
        "boolean" => "z.boolean()".to_string(),
        "null" => "z.null()".to_string(),
        _ => "z.unknown()".to_string(),
    }
}

fn zod_object(schema: &Value) -> String {
    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let required = required(schema);
    let fields = properties
        .iter()
        .map(|(name, property)| {
            let optional = if required.contains(name) {
                ""
            } else {
                ".optional()"
            };
            format!("{}: {}{}", quoted(name), zod(property), optional)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let base = format!("z.object({{{fields}}})");

    match schema.get("additionalProperties") {
        Some(Value::Bool(false)) => format!("{base}.strict()"),
        Some(value) if value.is_object() => format!("{base}.catchall({})", zod(value)),
        _ => format!("{base}.passthrough()"),
    }
}

fn required(schema: &Value) -> BTreeSet<String> {
    schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn accepts_null(schema: &Value) -> bool {
    match schema.get("type") {
        Some(Value::String(kind)) => kind == "null",
        Some(Value::Array(types)) => types.iter().any(|kind| kind == "null"),
        _ => variants(schema).into_iter().flatten().any(accepts_null),
    }
}

fn union(mut values: Vec<String>) -> String {
    values.sort();
    values.dedup();
    match values.len() {
        0 => "unknown".to_string(),
        1 => values.remove(0),
        _ => values.join(" | "),
    }
}

fn zod_union(mut values: Vec<String>) -> String {
    values.sort();
    values.dedup();
    match values.len() {
        0 => "z.unknown()".to_string(),
        1 => values.remove(0),
        _ => format!("z.union([{}])", values.join(", ")),
    }
}

fn literal(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "undefined".to_string())
}

fn quoted(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

#[cfg(test)]
mod tests {
    use super::{openapi, typescript, zod};

    fn fixture() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "city": { "type": "string" },
                "population": { "type": ["integer", "null"] }
            },
            "required": ["city"],
            "additionalProperties": false
        })
    }

    #[test]
    fn object_shapes_render_for_typescript_and_zod() {
        assert_eq!(
            typescript(&fixture()),
            "{ \"city\": string; \"population\"?: null | number }"
        );
        assert_eq!(
            zod(&fixture()),
            "z.object({\"city\": z.string(), \"population\": z.union([z.null(), z.number().int()]).optional()}).strict()"
        );
    }

    #[test]
    fn openapi_preserves_schema_and_adds_sql_nullability() {
        let parsed: serde_json::Value =
            serde_json::from_str(&openapi(&fixture(), true)).expect("valid JSON");
        assert_eq!(parsed["anyOf"][0]["properties"]["city"]["type"], "string");
        assert_eq!(parsed["anyOf"][1]["type"], "null");
    }
}
