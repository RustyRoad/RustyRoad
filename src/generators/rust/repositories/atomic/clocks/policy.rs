//! Scalar timestamp eligibility and validation for explicit database clocks.
use crate::database::introspection::{Column, Table};
use crate::generators::rust::allocation::Names;
use crate::generators::rust::repositories::inputs;

/// Accept timestamp fields without changing primary keys or generated identities.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn eligible(table: &Table, column: &Column) -> bool {
    let kind = column.sql_type.trim().to_ascii_lowercase();
    !column.auto_increment && !table.primary_key.contains(&column.name) && scalar_timestamp(&kind)
}

/// Recognize catalog timestamp spellings without accepting domains or arrays.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
fn scalar_timestamp(kind: &str) -> bool {
    let (mut tail, zoned) = if let Some(tail) = kind.strip_prefix("timestamptz") {
        (tail, true)
    } else if let Some(tail) = kind.strip_prefix("timestamp") {
        (tail, false)
    } else {
        return false;
    };
    if let Some(precision) = tail.strip_prefix('(') {
        let Some((digits, suffix)) = precision.split_once(')') else {
            return false;
        };
        if digits.is_empty() || !digits.bytes().all(|digit| digit.is_ascii_digit()) {
            return false;
        }
        tail = suffix;
    }
    tail.is_empty() || (!zoned && matches!(tail, " with time zone" | " without time zone"))
}

/// Reject unsupported clocks and omitted mandatory timestamps before querying.
/// Conflict-key timestamps may be supplied explicitly but cannot be clocks.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn validation(table: &Table, key: &[String], names: &Names) -> String {
    let columns: Vec<_> = table
        .columns
        .iter()
        .filter(|c| eligible(table, c))
        .collect();
    let allowed = columns
        .iter()
        .filter(|c| !key.contains(&c.name))
        .map(|c| format!("{:?}", c.name))
        .collect::<Vec<_>>()
        .join(" | ");
    let valid = if allowed.is_empty() {
        "false".to_string()
    } else {
        format!("matches!(*clock, {allowed})")
    };
    let mut output = format!(
        "for clock in insert_clocks.iter().chain(update_clocks.iter()) {{\n\
             if !({valid}) {{ return Err(sqlx::Error::Protocol(format!(\"Invalid database timestamp column: {{clock}}\"))); }}\n\
         }}\n"
    );
    for column in columns.iter().filter(|c| !inputs::optional(c)) {
        let field = names.field(table, &column.name);
        output.push_str(&format!(
            "if input.{field}.is_none() && !insert_clocks.contains(&{column_name:?}) {{\n\
                 return Err(sqlx::Error::Protocol({message:?}.into()));\n\
             }}\n",
            column_name = column.name,
            message = format!("Missing required timestamp: {}", column.name),
        ));
    }
    output
}
