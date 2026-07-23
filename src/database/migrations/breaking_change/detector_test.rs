use super::{scan_sql, BreakingChangeKind};
use std::path::Path;

#[test]
fn postgres_type_cast_is_flagged() {
    let sql =
        "ALTER TABLE orders\nALTER COLUMN customer_id\nTYPE BIGINT\nUSING customer_id::BIGINT;";
    let findings = scan_sql(Path::new("up.sql"), sql);
    assert!(findings
        .iter()
        .any(|finding| { finding.kind == BreakingChangeKind::ColumnTypeChange }));
    assert!(findings
        .iter()
        .any(|finding| finding.kind == BreakingChangeKind::ExplicitCast));
}

#[test]
fn mysql_modify_and_change_are_flagged() {
    for sql in [
        "ALTER TABLE orders MODIFY COLUMN customer_id BIGINT;",
        "ALTER TABLE orders CHANGE COLUMN customer_id customer_id BIGINT;",
    ] {
        let findings = scan_sql(Path::new("up.sql"), sql);
        assert_eq!(findings[0].kind, BreakingChangeKind::ColumnTypeChange);
    }
}

#[test]
fn foreign_key_constraint_drop_is_flagged() {
    let findings = scan_sql(
        Path::new("up.sql"),
        "ALTER TABLE orders DROP CONSTRAINT orders_customer_id_fkey;",
    );
    assert_eq!(findings[0].kind, BreakingChangeKind::ConstraintDrop);
}

#[test]
fn additive_migration_is_not_flagged() {
    let findings = scan_sql(
        Path::new("up.sql"),
        "ALTER TABLE orders ADD COLUMN note TEXT;",
    );
    assert!(findings.is_empty());
}

#[test]
fn commented_breaking_sql_is_not_flagged() {
    let findings = scan_sql(
        Path::new("up.sql"),
        "-- ALTER COLUMN customer_id TYPE BIGINT USING customer_id::BIGINT;",
    );
    assert!(findings.is_empty());
}
