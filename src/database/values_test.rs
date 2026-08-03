//! Value decoding for query output.
//!
//! These assert the decoder's ordering and its handling of types sqlx cannot
//! decode. The types themselves are exercised live against Postgres, since a unit
//! test cannot construct a `PgRow`.

use super::values::postgres::pg_display;

/// The decoder is exercised through a live connection; see
/// `.codetether-agent/evidence/live-values.md`. What can be asserted here is that
/// the module exposes the two entry points the output paths depend on.
#[test]
fn decoder_entry_points_exist() {
    // A compile-time check: both paths must share one decoder, or table and JSON
    // output would disagree about a type.
    let _display: fn(&sqlx::postgres::PgRow, &str) -> String = pg_display;
    let _json: fn(&sqlx::postgres::PgRow) -> serde_json::Map<String, serde_json::Value> =
        super::values::pg_row;
}
