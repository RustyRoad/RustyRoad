//! Output for `rustyroad pull`.

use crate::database::introspection::Schema;
use std::path::Path;

/// Reports what was written.
pub(super) fn written(schema: &Schema, files: &[std::path::PathBuf]) {
    let columns: usize = schema.tables.iter().map(|t| t.columns.len()).sum();
    let keys: usize = schema.tables.iter().map(|t| t.foreign_keys.len()).sum();

    println!(
        "Introspected {} table(s), {columns} column(s), {keys} foreign key(s).\n",
        schema.tables.len()
    );

    for file in files {
        println!("  wrote {}", display(file));
    }

    println!("\nNext:\n  npm install drizzle-orm pg\n  npm install -D drizzle-kit");
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
