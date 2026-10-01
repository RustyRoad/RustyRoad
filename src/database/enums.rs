//! Enum-type inspection backing `rustyroad db enums`.
//!
//! Postgres keeps user-defined enum types in the catalog; MySQL stores enum
//! value lists on the columns; SQLite has no enum type.

mod catalog;
mod model;
mod mysql;
mod parser;
mod postgres;

pub use catalog::read_enums;

use crate::database::migrations::CustomMigrationError;

/// Inspects and prints the user-defined enum types defined in the database.
pub async fn inspect_enums(format: &str) -> Result<(), CustomMigrationError> {
    let output = read_enums().await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    if output.enums.is_empty() {
        println!("No enums found.");
        return Ok(());
    }

    println!("Database Enums:");
    println!("{:-<30}", "");
    for enum_type in &output.enums {
        match (&enum_type.table, &enum_type.column) {
            (Some(table), Some(column)) => {
                println!("Enum: {}.{} ({})", table, column, enum_type.name)
            }
            _ => println!("Enum: {}", enum_type.name),
        }
        for value in &enum_type.values {
            println!("  - {}", value);
        }
        println!("{:-<30}", "");
    }

    Ok(())
}
