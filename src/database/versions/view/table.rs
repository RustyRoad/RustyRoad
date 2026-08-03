//! View DDL projecting a physical table into a schema version.

use super::super::model::Table;
use super::super::naming::versioned_schema;
use super::super::quote::{quote_ident, quote_qualified};

/// Postgres version from which `security_invoker` views are available.
pub const SECURITY_INVOKER_MIN_MAJOR: u32 = 15;

/// Returns the statements that define `table` as a view in `version`.
///
/// Column defaults are restated because a view does not inherit the defaults of
/// its underlying table.
pub fn create_view(schema: &str, version: &str, table: &Table, major_version: u32) -> Vec<String> {
    let view = quote_qualified(&versioned_schema(schema, version), &table.name);

    let mut statements = vec![
        format!("DROP VIEW IF EXISTS {view}"),
        format!(
            "CREATE VIEW {view}{} AS SELECT {} FROM {}",
            options(major_version),
            projection(table),
            quote_qualified(schema, &table.physical_name)
        ),
    ];

    for column in table.visible_columns() {
        if let Some(default) = &column.default {
            statements.push(format!(
                "ALTER VIEW {view} ALTER {} SET DEFAULT {default}",
                quote_ident(&column.name)
            ));
        }
    }

    statements
}

/// Returns the `physical AS logical` projection list.
fn projection(table: &Table) -> String {
    table
        .visible_columns()
        .iter()
        .map(|column| {
            format!(
                "{} AS {}",
                quote_ident(&column.physical_name),
                quote_ident(&column.name)
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Returns view options for the server version.
///
/// `security_invoker` makes the view respect row-level security on the underlying
/// table. It is unavailable before Postgres 15.
fn options(major_version: u32) -> &'static str {
    if major_version >= SECURITY_INVOKER_MIN_MAJOR {
        " WITH (security_invoker = true)"
    } else {
        ""
    }
}
