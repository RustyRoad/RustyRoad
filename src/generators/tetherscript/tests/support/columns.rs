//! Column builders for the fixture tables.

use crate::database::introspection::Column;

/// Builds a nullable column with no default.
pub(in crate::generators::tetherscript::tests) fn column(name: &str, sql_type: &str) -> Column {
    Column {
        name: name.to_string(),
        sql_type: sql_type.to_string(),
        json_schema: None,
        nullable: true,
        default: None,
        auto_increment: false,
    }
}

/// Builds a non-nullable column.
pub(in crate::generators::tetherscript::tests) fn required(name: &str, sql_type: &str) -> Column {
    Column {
        nullable: false,
        ..column(name, sql_type)
    }
}

/// Builds a serial primary key column.
pub(in crate::generators::tetherscript::tests) fn serial_key(name: &str) -> Column {
    Column {
        nullable: false,
        auto_increment: true,
        default: Some("nextval('products_id_seq'::regclass)".to_string()),
        ..column(name, "integer")
    }
}
