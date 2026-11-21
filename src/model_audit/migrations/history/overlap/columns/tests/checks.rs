//! CHECK history narrows only when every dependency is statically identifiable.
use super::overlaps;

#[test]
fn checks_and_literal_defaults_only_block_their_columns() {
    for source in [
        "ALTER TABLE rows ADD CHECK (old IN ('a', 'b'));",
        "ALTER TABLE rows ADD CHECK (old BETWEEN 1 AND 4);",
        "ALTER TABLE rows ADD CHECK (old IS NOT NULL AND NOT (old < 0));",
        "ALTER TABLE rows ALTER COLUMN old SET DEFAULT 'literal';",
    ] {
        assert!(overlaps(source, "old", "repair_columns"), "{source}");
        assert!(!overlaps(source, "new", "repair_columns"), "{source}");
        assert!(overlaps(source, "new", "create_table"), "{source}");
    }
}

#[test]
fn mixed_checks_collect_all_references_with_postgres_identity() {
    let source = "ALTER TABLE rows ALTER COLUMN first DROP DEFAULT, \
        ADD CHECK (\"Other\" >= second AND third IN (fourth));";
    for column in ["first", "Other", "second", "third", "fourth"] {
        assert!(overlaps(source, column, "repair_columns"), "{column}");
    }
    for column in ["new", "other"] {
        assert!(!overlaps(source, column, "repair_columns"), "{column}");
    }
    assert!(!overlaps(
        "ALTER TABLE other.rows ADD CHECK (old > 0);",
        "old",
        "repair_columns"
    ));
}

#[test]
fn unknown_dependencies_keep_the_table_wide_guard() {
    for source in [
        "ALTER TABLE rows ADD CHECK (custom(old));",
        "ALTER TABLE rows ADD CHECK (old IN (SELECT id FROM other));",
        "ALTER TABLE rows ADD FOREIGN KEY (old) REFERENCES other(id);",
        "ALTER TABLE rows ADD UNIQUE (old);",
        "ALTER TABLE rows DROP CONSTRAINT rows_check;",
        "ALTER TABLE rows ADD CHECK (old > 0), ADD CHECK (custom(other));",
        "ALTER TABLE rows ALTER COLUMN old SET DEFAULT custom();",
    ] {
        assert!(overlaps(source, "new", "repair_columns"), "{source}");
    }
}

#[test]
fn literals_are_not_column_dependencies() {
    let source = "ALTER TABLE rows ADD CHECK (old IN ('new', 'other'));";
    assert!(!overlaps(source, "new", "repair_columns"));
    assert!(!overlaps(source, "other", "repair_columns"));
}
