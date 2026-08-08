//! Batch SQL for the backfill loop.

use super::super::super::quote::quote_ident;
use super::super::trigger::NEEDS_BACKFILL_COLUMN;
use super::clauses;

/// Returns the statement advancing the backfill by one batch.
///
/// Rows are selected by keyset pagination over the primary key rather than OFFSET, so
/// cost does not grow with progress. `FOR NO KEY UPDATE` locks only the batch, leaving
/// writers to other rows unblocked, which is what keeps the backfill from taking the
/// table down.
///
/// The UPDATE is a no-op assignment: it exists to fire the trigger, which computes the
/// new value and clears the marker.
pub fn batch(
    table: &str,
    primary_key: &[String],
    batch_size: u32,
    last_value: Option<&[String]>,
) -> String {
    let quoted = quote_ident(table);
    let keys = clauses::key_list(primary_key);

    let mut selection = format!(
        "SELECT {keys} FROM {quoted} WHERE {} = true",
        quote_ident(NEEDS_BACKFILL_COLUMN)
    );
    if let Some(last) = last_value {
        selection.push_str(&clauses::cursor(primary_key, last));
    }
    selection.push_str(&format!(
        " ORDER BY {keys} LIMIT {batch_size} FOR NO KEY UPDATE"
    ));

    let first = primary_key
        .first()
        .map(|key| quote_ident(key))
        .unwrap_or_else(|| quote_ident("id"));

    format!(
        "WITH batch AS ({selection}), \
         updated AS (UPDATE {quoted} SET {first} = {quoted}.{first} FROM batch \
         WHERE {} RETURNING {}) \
         SELECT {} FROM updated ORDER BY {keys} DESC LIMIT 1",
        clauses::join(table, primary_key),
        clauses::returning(table, primary_key),
        clauses::cursor_projection(primary_key)
    )
}
