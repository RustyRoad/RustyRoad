//! Output for `rustyroad pull`.

mod summary;

use super::notices;
use crate::database::introspection::Schema;
use crate::generators::layout::Layout;
use crate::generators::report::Outcome;
use crate::generators::typescript::writer::Report;

/// Reports what was written, what was kept, and anything left unreachable.
pub(super) fn written(schema: &Schema, report: &Report) {
    summary::schema(schema);
    summary::list(&report.outcomes);

    if report.outcomes.iter().any(Outcome::is_preserved) {
        notices::preserved();
    }

    notices::unwired(&report.unwired);
    notices::next_steps();
}

/// Reports what one model generator produced.
///
/// Shared by every language because the shape is the same: a list of files, then a warning
/// about any that were written but left undeclared.
pub(super) fn models(label: &str, layout: &Layout) {
    println!("\n{label}:");
    summary::list(&layout.outcomes);
    notices::undeclared(&layout.undeclared);
}

/// Reports a flat companion output with no developer-owned parent module.
pub(super) fn artifacts(label: &str, outcomes: &[Outcome]) {
    println!("\n{label}:");
    summary::list(outcomes);
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
