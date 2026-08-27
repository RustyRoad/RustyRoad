//! Introspected database schema model.
//!
//! Mirrors the shape Drizzle Kit builds before emitting TypeScript: tables with
//! columns, primary keys, foreign keys, unique constraints, and indexes. Code
//! generation reads only this model, so generators stay backend-agnostic.

mod constraints;

pub use constraints::{ForeignKey, Index, Unique};

/// A user-defined enum type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: String,
    /// Allowed values, in declaration order.
    pub values: Vec<String>,
}

/// A column as it exists in the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,
    /// SQL type as reported by the database, e.g. `character varying(255)`.
    pub sql_type: String,
    /// Opt-in JSON Schema read from an `@rustyroad-json-schema` column comment.
    ///
    /// This remains `None` for ordinary columns and unannotated JSON values.
    pub json_schema: Option<serde_json::Value>,
    pub nullable: bool,
    pub default: Option<String>,
    /// True when the column is `serial`/`identity` backed.
    pub auto_increment: bool,
}

/// A table and everything generation needs to know about it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    /// Primary key columns, in key order. Empty when the table has none.
    pub primary_key: Vec<String>,
    pub foreign_keys: Vec<ForeignKey>,
    pub uniques: Vec<Unique>,
    pub indexes: Vec<Index>,
    /// True for a view or materialized view.
    ///
    /// A view has no insert, update, or delete, so generators emit only the reads;
    /// writing to one fails at runtime, and offering the methods would advertise it.
    pub view: bool,
}

impl Table {
    /// Returns the column with `name`, if present.
    pub fn column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|column| column.name == name)
    }

    /// Returns `true` when the primary key is a single column.
    pub fn has_simple_key(&self) -> bool {
        self.primary_key.len() == 1
    }
}

/// A full introspected schema.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Schema {
    pub tables: Vec<Table>,
    /// Enum types the columns may reference.
    pub enums: Vec<Enum>,
}

impl Schema {
    /// Returns the table with `name`, if present.
    pub fn table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|table| table.name == name)
    }

    /// Returns the enum type with `name`, if present.
    pub fn enum_type(&self, name: &str) -> Option<&Enum> {
        self.enums.iter().find(|item| item.name == name)
    }
}
