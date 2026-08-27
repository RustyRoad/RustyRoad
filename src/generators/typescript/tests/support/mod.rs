//! Shared fixtures for generation tests.

mod tables;

pub(super) use tables::{posts, users};

use crate::database::introspection::{Column, Enum, Schema, Table};

/// Builds a nullable column with no default.
pub(super) fn column(name: &str, sql_type: &str) -> Column {
    Column {
        name: name.to_string(),
        sql_type: sql_type.to_string(),
        json_schema: None,
        nullable: true,
        default: None,
        auto_increment: false,
    }
}

/// Builds a serial primary key column.
pub(super) fn serial_key(name: &str) -> Column {
    Column {
        nullable: false,
        auto_increment: true,
        default: Some("nextval('users_id_seq'::regclass)".to_string()),
        ..column(name, "integer")
    }
}

/// Wraps tables in a schema with no enum types.
pub(super) fn from_tables(tables: Vec<Table>) -> Schema {
    Schema {
        tables,
        enums: Vec::new(),
    }
}

/// Wraps tables and enum types in a schema.
pub(super) fn with_enums(tables: Vec<Table>, enums: Vec<Enum>) -> Schema {
    Schema { tables, enums }
}

/// Builds an enum type.
pub(super) fn enum_type(name: &str, values: &[&str]) -> Enum {
    Enum {
        name: name.to_string(),
        values: values.iter().map(|value| value.to_string()).collect(),
    }
}

/// A schema containing both fixture tables.
pub(super) fn schema() -> Schema {
    from_tables(vec![users(), posts()])
}

/// A schema with database-backed JSON Schema metadata on `users.metadata`.
pub(super) fn annotated_json_schema() -> Schema {
    let mut model = schema();
    let metadata = model.tables[0]
        .columns
        .iter_mut()
        .find(|column| column.name == "metadata")
        .expect("metadata fixture column");
    metadata.json_schema = Some(serde_json::json!({
        "type": "object",
        "properties": {
            "city": { "type": "string" },
            "population": { "type": "integer" }
        },
        "required": ["city"],
        "additionalProperties": false
    }));
    model
}

/// A schema whose single-column primary key is a PostgreSQL enum.
///
/// Mirrors the shape that exposed overly broad `string` repository IDs in
/// SpotlessBinCo's generated campaign workflow tables.
pub(super) fn enum_key_schema() -> Schema {
    let table = Table {
        name: "campaign_workflow_node_bindings".to_string(),
        columns: vec![Column {
            name: "node_kind".to_string(),
            sql_type: "campaign_node_kind".to_string(),
            json_schema: None,
            nullable: false,
            default: None,
            auto_increment: false,
        }],
        primary_key: vec!["node_kind".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    };

    with_enums(
        vec![table],
        vec![enum_type("campaign_node_kind", &["email", "sms"])],
    )
}
