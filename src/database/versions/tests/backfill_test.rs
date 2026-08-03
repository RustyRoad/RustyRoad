//! Batched backfill SQL.

use crate::database::versions::ops::backfill::batch_sql;
use crate::database::versions::ops::trigger::NEEDS_BACKFILL_COLUMN;

/// Single-column primary key.
fn id_key() -> Vec<String> {
    vec!["id".to_string()]
}

#[test]
fn first_batch_selects_unprocessed_rows_only() {
    let sql = batch_sql("users", &id_key(), 1000, None);

    assert!(sql.contains(&format!(r#"WHERE "{NEEDS_BACKFILL_COLUMN}" = true"#)));
    assert!(sql.contains("LIMIT 1000"));
    // Without a cursor there is no lower bound yet.
    assert!(!sql.contains('>'));
}

#[test]
fn batch_locks_only_its_own_rows() {
    let sql = batch_sql("users", &id_key(), 1000, None);

    // A plain FOR UPDATE would block concurrent writers to unrelated columns.
    assert!(sql.contains("FOR NO KEY UPDATE"));
}

#[test]
fn later_batches_resume_after_the_cursor() {
    let last = vec!["500".to_string()];
    let sql = batch_sql("users", &id_key(), 1000, Some(&last));

    // Keyset pagination: cost stays flat as the backfill progresses, unlike OFFSET.
    assert!(sql.contains(r#"AND ("id") > ('500')"#));
    assert!(!sql.contains("OFFSET"));
}

#[test]
fn cursor_values_are_quoted() {
    let last = vec!["o'brien".to_string()];
    let sql = batch_sql("users", &id_key(), 10, Some(&last));

    // An embedded quote must not terminate the literal.
    assert!(sql.contains("'o''brien'"));
}

#[test]
fn composite_keys_are_compared_as_tuples() {
    let keys = vec!["tenant_id".to_string(), "id".to_string()];
    let last = vec!["7".to_string(), "500".to_string()];
    let sql = batch_sql("events", &keys, 100, Some(&last));

    assert!(sql.contains(r#"("tenant_id", "id") > ('7', '500')"#));
    assert!(sql.contains(r#""events"."tenant_id" = batch."tenant_id""#));
    assert!(sql.contains(r#""events"."id" = batch."id""#));
}

#[test]
fn update_returns_the_next_cursor() {
    let sql = batch_sql("users", &id_key(), 1000, None);

    // The last row of the batch becomes the next batch's lower bound.
    assert!(sql.contains("RETURNING"));
    assert!(sql.ends_with(r#"SELECT "id" FROM updated ORDER BY "id" DESC LIMIT 1"#));
}
