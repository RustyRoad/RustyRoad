//! The CRUD submodules and their statements.

use super::rendered::file;

#[test]
fn the_insert_binds_only_caller_supplied_columns() {
    let create = file("create.tether");

    assert!(create.contains("INSERT INTO products ("));
    assert!(create.contains("RETURNING *"));
    // Nine columns minus the serial key and the two timestamps leaves six.
    assert!(
        create.contains("VALUES ($1, $2, $3, $4, $5, $6)"),
        "got: {create}"
    );
    assert!(!create.contains("row[\"id\"]"));
    assert!(!create.contains("row[\"created_at\"]"));
}

#[test]
fn parameters_are_bound_rather_than_spliced() {
    let create = file("create.tether");

    // A spliced value could terminate the statement; a bound one is inert.
    assert!(create.contains("db.query(sql, ["));
    assert!(!create.contains("+ row["));
}

#[test]
fn the_lookup_returns_nil_rather_than_erroring_on_a_missing_row() {
    let read = file("read.tether");

    assert!(read.contains("WHERE id = $1"));
    assert!(read.contains("if rows.len() == 0 {"));
    assert!(read.contains("return Ok(nil)"));
}

#[test]
fn the_listing_is_ordered_so_results_do_not_shuffle() {
    assert!(file("read.tether").contains("ORDER BY created_at DESC"));
}

#[test]
fn the_update_numbers_the_key_after_the_assignments() {
    let update = file("update.tether");

    // Six writable columns, so the key must bind as $7; an off-by-one here targets the
    // wrong row rather than failing loudly.
    assert!(update.contains("WHERE id = $7"), "got: {update}");
    assert!(update.contains("updated_at = CURRENT_TIMESTAMP"));
    // updated_at is set by the statement, so binding it as well would assign the column
    // twice in one SET, which Postgres rejects.
    assert!(!update.contains("row[\"updated_at\"]"));
}

#[test]
fn the_delete_reports_whether_a_row_was_removed() {
    let delete = file("delete.tether");

    assert!(delete.contains("DELETE FROM products WHERE id = $1 RETURNING id"));
    assert!(delete.contains("return Ok(rows.len() > 0)"));
}
