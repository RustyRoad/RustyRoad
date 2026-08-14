//! The CRUD submodules and their statements.

use super::rendered::file;

#[test]
fn the_insert_binds_only_caller_supplied_columns_in_order() {
    let create = file("create.rs");

    assert!(create.contains("INSERT INTO products ("));
    assert!(create.contains("RETURNING *"));
    // Ten columns minus the serial key and the two timestamps leaves seven.
    assert!(create.contains("VALUES ($1, $2, $3, $4, $5, $6, $7)"));
    assert!(!create.contains(".bind(row.id)"));
    assert!(!create.contains(".bind(row.created_at)"));
}

#[test]
fn the_lookup_returns_an_option_rather_than_erroring() {
    let read = file("read.rs");

    // RowNotFound forces every caller to destructure an error to learn nothing matched.
    assert!(read.contains("pub async fn find(id: i32) -> Result<Option<Self>, sqlx::Error>"));
    assert!(read.contains("fetch_optional"));
    assert!(read.contains("WHERE id = $1"));
}

#[test]
fn the_listing_is_ordered_so_results_do_not_shuffle() {
    let read = file("read.rs");

    assert!(read.contains("ORDER BY created_at DESC"));
    assert!(read.contains("pub async fn all() -> Result<Vec<Self>, sqlx::Error>"));
}

#[test]
fn the_update_numbers_the_key_after_the_assignments() {
    let update = file("update.rs");

    // Seven writable columns, so the key must bind as $8; an off-by-one here targets the
    // wrong row rather than failing loudly.
    assert!(update.contains("WHERE id = $8"), "got: {update}");
    assert!(update.contains("updated_at = CURRENT_TIMESTAMP"));
    assert!(update.contains("RETURNING *"));
    // updated_at is set by the statement, so binding it too would assign it twice.
    assert!(!update.contains(".bind(row.updated_at)"));
}

#[test]
fn the_delete_reports_whether_a_row_was_removed() {
    let delete = file("delete.rs");

    assert!(delete.contains("pub async fn delete(id: i32) -> Result<bool, sqlx::Error>"));
    assert!(delete.contains("rows_affected() > 0"));
}
