//! Reject ambiguous insertion settings before any generated output is written.
use super::super::Projection;
use crate::database::introspection::Table;
use crate::generators::rust::repositories::create_clock;
use std::collections::BTreeSet;

/// Checks insertion-only settings against the same timestamp rules as New inputs.
/// Views and read-only options cannot be inserted; unknown or duplicate clocks fail.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust::projections) fn validate(
    table: &Table,
    projection: &Projection,
) -> Result<(), &'static str> {
    if !projection.insert {
        if projection.insert_input.is_some() {
            return Err("insert_input requires insert = true");
        }
        return if projection.database_timestamps.is_empty() {
            Ok(())
        } else {
            Err("database_timestamps requires insert = true")
        };
    }
    if table.view {
        return Err("insert projections require a writable table, not a view");
    }
    if projection.join.is_some() || projection.comparisons || !projection.order.is_empty() {
        return Err("insert projections cannot join, compare, or order rows");
    }
    let mut selected = BTreeSet::new();
    if let Some(input) = &projection.insert_input {
        super::codecs::validate(table, input)?;
    }
    for name in &projection.database_timestamps {
        if !selected.insert(name) {
            return Err("database_timestamps contains a duplicate column");
        }
        let column = table
            .column(name)
            .ok_or("database timestamp column is unknown")?;
        if !create_clock::policy::eligible(table, column) {
            return Err("database timestamps require omittable non-key scalar timestamp inputs");
        }
    }
    Ok(())
}
