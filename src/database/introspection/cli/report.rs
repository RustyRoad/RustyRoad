//! Output for `rustyroad pull`.

use super::notices;
use crate::database::introspection::Schema;
use crate::generators::typescript::writer::{Outcome, Report};
use std::path::Path;

/// Reports what was written, what was kept, and anything left unreachable.
pub(super) fn written(schema: &Schema, report: &Report) {
    summary(schema);

    for outcome in &report.outcomes {
        let label = if outcome.is_preserved() {
            "kept "
        } else {
            "wrote"
        };
        println!("  {label} {}", display(outcome.path()));
    }

    if report.outcomes.iter().any(Outcome::is_preserved) {
        notices::preserved();
    }

    notices::unwired(&report.unwired);
    notices::next_steps();
}

/// Prints what was found in the database.
fn summary(schema: &Schema) {
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

/// Reports an empty schema, which is usually a wrong schema name.
pub(super) fn empty(schema_name: &str) {
    println!(
        "No tables found in schema '{schema_name}'. Nothing was written.\n\n\
         If your tables live elsewhere, pass --schema <name>."
    );
}

/// Reports a failure and exits nonzero so callers and CI can detect it.
pub(super) fn fail(message: &str) {
    eprintln!("Pull failed: {message}");
    std::process::exit(1);
}

/// Renders a path for display, preferring the relative form.
fn display(path: &Path) -> String {
    path.strip_prefix(std::env::current_dir().unwrap_or_default())
        .unwrap_or(path)
        .display()
        .to_string()
}
