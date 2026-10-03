//! Printing what was found and what was written.

use crate::database::introspection::Schema;
use crate::generators::report::Outcome;
use std::path::Path;

/// Prints what was found in the database.
pub(super) fn schema(schema: &Schema) {
    let columns: usize = schema.tables.iter().map(|t| t.columns.len()).sum();
    let keys: usize = schema.tables.iter().map(|t| t.foreign_keys.len()).sum();

    let enums = if schema.enums.is_empty() {
        String::new()
    } else {
        format!(", {} enum type(s)", schema.enums.len())
    };

    println!(
        "Introspected {} table(s), {columns} column(s), {keys} foreign key(s){enums}.\n",
        schema.tables.len()
    );
}

/// Prints one line per file, saying whether it was written or kept.
pub(super) fn list(outcomes: &[Outcome]) {
    for outcome in outcomes {
        let label = if outcome.is_preserved() {
            "kept "
        } else {
            "wrote"
        };
        println!("  {label} {}", display(outcome.path()));
    }
}

/// Renders a path for display, preferring the relative form.
fn display(path: &Path) -> String {
    path.strip_prefix(std::env::current_dir().unwrap_or_default())
        .unwrap_or(path)
        .display()
        .to_string()
}
