//! Narrowing a pull to the tables that matter.
//!
//! A live database accumulates junk — abandoned experiments, singular/plural duplicates,
//! framework ledgers — and generating a model for every one of them buries the models a
//! project actually wants. `--tables` keeps only what is named; `--exclude` drops what is
//! named; both accept a trailing `*` so a family like `ab_test_*` is one argument.

use crate::database::introspection::Schema;
use clap::ArgMatches;

/// Applies `--tables` and `--exclude` to the schema, in that order.
///
/// Enums are left untouched: they are filtered later by reference, so an enum used only by
/// an excluded table simply stops being emitted.
pub(super) fn apply(matches: &ArgMatches, mut schema: Schema) -> Schema {
    let keep: Vec<&String> = matches
        .get_many::<String>("tables")
        .map(|values| values.collect())
        .unwrap_or_default();
    let drop: Vec<&String> = matches
        .get_many::<String>("exclude")
        .map(|values| values.collect())
        .unwrap_or_default();

    if !keep.is_empty() {
        schema
            .tables
            .retain(|table| keep.iter().any(|p| matches_pattern(p, &table.name)));
    }
    schema
        .tables
        .retain(|table| !drop.iter().any(|p| matches_pattern(p, &table.name)));

    schema
}

/// Returns `true` when `name` matches `pattern`.
///
/// Exact match, or prefix match when the pattern ends in `*`. Nothing more: a fuller glob
/// would invite patterns whose misses are silent, and a prefix covers the real cases —
/// table families share a prefix by convention.
fn matches_pattern(pattern: &str, name: &str) -> bool {
    match pattern.strip_suffix('*') {
        Some(prefix) => name.starts_with(prefix),
        None => pattern == name,
    }
}
