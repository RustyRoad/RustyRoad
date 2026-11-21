//! Whole-row statement acceptance and conservative SQL rejection fixtures.
/// A projection, join, CTE, or compound query cannot prove row ownership.
#[test]
fn rejects_non_rows() {
    for sql in [
        "SELECT id FROM rows",
        "SELECT * FROM rows JOIN other USING (id)",
        "WITH rows AS (SELECT 1) SELECT * FROM rows",
        "SELECT * FROM a UNION SELECT * FROM b",
        "SELECT * FROM public.rows",
        "SELECT * FROM rows; SELECT * FROM other",
        "INSERT INTO rows (id) VALUES (1) RETURNING id",
        "SELECT * FROM rows AS r (renamed)",
        "SELECT * FROM rows TABLESAMPLE BERNOULLI (10)",
    ] {
        assert_eq!(super::target(sql), None, "{sql}");
    }
}

/// All supported statement forms return their whole-row target.
#[test]
fn recognizes_row_targets() {
    for sql in [
        "SELECT * FROM Rows WHERE id = $1",
        "SELECT * FROM rows AS r",
        "INSERT INTO rows (id) VALUES ($1) RETURNING *",
        "UPDATE rows SET id = $1 RETURNING *",
        "DELETE FROM rows RETURNING *",
    ] {
        assert_eq!(super::target(sql).as_deref(), Some("rows"), "{sql}");
    }
}
