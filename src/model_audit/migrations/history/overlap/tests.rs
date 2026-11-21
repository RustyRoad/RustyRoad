//! Identifier regressions use local SQL strings, never database execution.

use super::{conflicts, sql};
use crate::model_audit::schema::types::Proposal;

/// Check one parsed history statement against a catalog-backed column proposal.
fn overlaps(history: &str, column: &str) -> bool {
    let proposal = Proposal {
        model_id: "Row".into(),
        table: "rows".into(),
        operation: "add_columns".into(),
        columns: vec![column.into()],
        up: String::new(),
        down: String::new(),
        warnings: vec![],
    };
    sql::operations(history)
        .unwrap()
        .iter()
        .any(|operation| conflicts(operation, "public", &proposal).unwrap())
}

#[test]
fn quoted_and_unquoted_lowercase_columns_block_duplicate_work() {
    assert!(overlaps(
        "ALTER TABLE \"public\".\"rows\" ADD COLUMN \"note\" text;",
        "note"
    ));
    assert!(overlaps(
        "ALTER TABLE public.rows ADD COLUMN NOTE text;",
        "note"
    ));
}

#[test]
fn quoted_case_is_preserved_in_overlap_checks() {
    let history = "ALTER TABLE public.rows ADD COLUMN \"Note\" text;";
    assert!(overlaps(history, "Note"));
    assert!(!overlaps(history, "note"));
    assert!(!overlaps(
        "ALTER TABLE public.rows ADD COLUMN note text;",
        "Note"
    ));
}

#[test]
fn escaped_quotes_and_dots_are_part_of_a_column_name() {
    assert!(sql::same_column("\"a\"\"b.c\"", "a\"b.c"));
    assert!(!sql::same_column("\"a\"\"b.c\"", "a.b.c"));
}

#[test]
fn sql_stored_as_string_data_is_not_pending_ddl() {
    assert!(!overlaps(
        "INSERT INTO templates VALUES ('ALTER TABLE public.rows ADD COLUMN note text;');",
        "note"
    ));
    assert!(!overlaps(
        "-- ALTER TABLE public.rows ADD COLUMN note text;",
        "note"
    ));
}
