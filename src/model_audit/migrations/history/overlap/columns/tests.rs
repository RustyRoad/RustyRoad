//! Column-scope guards exercise parsed SQL locally without executing it.
use super::super::{conflicts, sql};
use crate::model_audit::schema::types::Proposal;

mod checks;

/// Exercise the real history parser, e.g. `overlaps(sql, "id", "repair_columns")`.
pub(super) fn overlaps(source: &str, column: &str, operation: &str) -> bool {
    let identifier = format!("\"{}\"", column.replace('"', "\"\""));
    let action = if operation == "add_columns" {
        format!("ADD COLUMN {identifier} text")
    } else {
        format!("ALTER COLUMN {identifier} DROP NOT NULL")
    };
    let proposal = Proposal {
        model_id: "Row".into(),
        table: "rows".into(),
        operation: operation.into(),
        columns: vec![column.into()],
        up: format!("ALTER TABLE public.rows {action};"),
        down: String::new(),
        warnings: vec![],
    };
    sql::operations(source)
        .unwrap()
        .iter()
        .any(|history| conflicts(history, "public", &proposal).unwrap())
}

#[test]
fn simple_repairs_only_conflict_on_the_same_column() {
    for action in ["SET NOT NULL", "DROP NOT NULL", "DROP DEFAULT", "TYPE text"] {
        let source = format!("ALTER TABLE public.rows ALTER COLUMN old {action};");
        assert!(!overlaps(&source, "new", "repair_columns"), "{source}");
        assert!(overlaps(&source, "old", "repair_columns"), "{source}");
        assert!(overlaps(&source, "new", "create_table"), "{source}");
    }
}

#[test]
fn narrowing_requires_every_action_to_be_understood() {
    for source in [
        "ALTER TABLE rows ALTER COLUMN old SET DEFAULT other;",
        "ALTER TABLE rows ALTER COLUMN old TYPE text USING other::text;",
        "ALTER TABLE rows ALTER COLUMN old DROP NOT NULL, ADD CHECK (custom(other));",
        "ALTER TABLE rows DROP CONSTRAINT rows_pkey;",
        "DO $$ BEGIN ALTER TABLE rows ALTER COLUMN old DROP NOT NULL; END $$;",
    ] {
        assert!(overlaps(source, "new", "repair_columns"), "{source}");
    }
}

#[test]
fn column_identity_preserves_quotes_and_schema_scope() {
    let source = "ALTER TABLE public.rows ALTER COLUMN \"Old\" DROP NOT NULL;";
    assert!(overlaps(source, "Old", "repair_columns"));
    assert!(!overlaps(source, "old", "repair_columns"));
    assert!(!overlaps(
        "ALTER TABLE other.rows ALTER COLUMN old DROP NOT NULL;",
        "old",
        "repair_columns"
    ));
}

#[test]
fn mixed_disjoint_actions_are_permitted_but_duplicates_still_block() {
    let source = "ALTER TABLE rows ALTER COLUMN first DROP DEFAULT, \
        ALTER COLUMN second SET NOT NULL;";
    assert!(!overlaps(source, "third", "repair_columns"));
    assert!(overlaps(source, "second", "repair_columns"));
}

#[test]
fn original_creation_does_not_block_catalog_backed_repairs() {
    let source = "CREATE TABLE public.rows (old text);";
    assert!(!overlaps(source, "old", "repair_columns"));
    assert!(overlaps(source, "old", "add_columns"));
    assert!(overlaps(source, "new", "create_table"));
    assert!(!overlaps(source, "new", "add_columns"));
}

#[test]
fn subsequent_duplicate_repair_still_blocks_after_creation() {
    let source = "CREATE TABLE rows (old text); \
        ALTER TABLE rows ALTER COLUMN old DROP NOT NULL;";
    assert!(overlaps(source, "old", "repair_columns"));
    assert!(!overlaps(source, "new", "repair_columns"));
}
