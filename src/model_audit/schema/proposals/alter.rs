//! Atomic ALTER statements: a failing cast or NOT NULL check leaves no partial repair.
use super::keys::Change;
use super::{
    super::{
        rust_types,
        types::{Column, TableAudit},
    },
    quote,
};

/// Render forward contracts and reverse observed contracts; never discard extra columns.
pub(super) fn sql(table: &str, audit: &TableAudit, key: Option<&Change>) -> (String, String) {
    let mut up = Vec::new();
    let mut down = Vec::new();
    if let Some(key) = key {
        up.extend(key.drop_clause(false));
        down.extend(key.drop_clause(true));
    }
    for column in &audit.missing_columns {
        up.push(format!(
            "ADD COLUMN {} {}{}",
            quote(&column.name),
            column.sql_type,
            if column.nullable { "" } else { " NOT NULL" }
        ));
    }
    for change in &audit.column_changes {
        contract(&change.before, &change.after, &mut up);
    }
    for change in audit.column_changes.iter().rev() {
        contract(&change.after, &change.before, &mut down);
    }
    for column in audit.missing_columns.iter().rev() {
        down.push(format!("DROP COLUMN {}", quote(&column.name)));
    }
    if let Some(key) = key {
        up.extend(key.add_clause(false));
        down.extend(key.add_clause(true));
    }
    (statement(table, up), statement(table, down))
}

/// A cast is explicit, not a promise of lossless conversion or a backfill policy.
fn contract(before: &Column, after: &Column, actions: &mut Vec<String>) {
    let column = quote(&after.name);
    if rust_types::normalized(&before.sql_type) != rust_types::normalized(&after.sql_type) {
        actions.push(format!(
            "ALTER COLUMN {column} TYPE {} USING ({column})::{}",
            after.sql_type, after.sql_type
        ));
    }
    if before.nullable != after.nullable {
        actions.push(format!(
            "ALTER COLUMN {column} {} NOT NULL",
            if after.nullable { "DROP" } else { "SET" }
        ));
    }
}

/// Keep all operations for one direction in one atomic PostgreSQL statement.
fn statement(table: &str, actions: Vec<String>) -> String {
    format!("ALTER TABLE {table}\n  {};\n", actions.join(",\n  "))
}
