//! AST parsing regressions; SQL is inspected locally, never executed.
use super::{parse, SqlOperation};

#[test]
fn guarded_additions_preserve_identifiers_and_multiple_columns() {
    let sql = "ALTER TABLE IF EXISTS \"public\".\"Rows\" \
        ADD COLUMN IF NOT EXISTS \"a\"\"b.c\" text, ADD COLUMN NOTE integer";
    let operations = parse(sql);
    let [SqlOperation::AlterTableAddColumn {
        table_name,
        columns,
        ..
    }] = &operations[..]
    else {
        panic!("{operations:?}")
    };
    assert_eq!(table_name, "\"public\".\"Rows\"");
    assert_eq!(columns, &["\"a\"\"b.c\"", "NOTE"]);
}

#[test]
fn create_table_constraints_are_not_columns() {
    let operations = parse("CREATE TABLE public.rows (\"Note\" text, PRIMARY KEY (\"Note\"))");
    let [SqlOperation::CreateTable { columns, .. }] = &operations[..] else {
        panic!("{operations:?}")
    };
    assert_eq!(columns, &["\"Note\""]);
}

#[test]
fn mixed_or_unsupported_alterations_retain_raw_sql_for_overlap_checks() {
    for sql in [
        "ALTER TABLE rows ADD COLUMN note text, ALTER COLUMN id TYPE bigint",
        "ALTER TABLE rows ADD COLUMN note text, ADD CONSTRAINT named CHECK (note <> '')",
        "ALTER TABLE rows unsupported syntax",
    ] {
        assert_eq!(parse(sql), vec![SqlOperation::RawSql { sql: sql.into() }]);
    }
}
