//! Fixture-backed ALTER generation retains observed rollback contracts.
use super::{model, rows};
use crate::model_audit::schema::{matching, proposals, rust_types, types::Column};

#[test]
fn type_and_nullability_repairs_preserve_unrelated_columns() {
    let desired = model("struct Rows { id: i64, note: Option<String> }");
    let mut catalog = rows("table");
    for name in ["note", "unrelated"] {
        catalog.tables[0].columns.push(Column {
            name: name.into(),
            sql_type: "text".into(),
            nullable: false,
        });
    }
    let mut audit = matching::audit(&desired, &catalog, None);
    proposals::build(&desired, &catalog, &mut audit, &[]);
    let sql = audit.proposal.unwrap();
    assert!(sql.up.contains("ALTER COLUMN \"id\" TYPE bigint"));
    assert!(sql.up.contains("ALTER COLUMN \"note\" DROP NOT NULL"));
    assert!(sql.down.contains("ALTER COLUMN \"id\" TYPE integer"));
    assert!(sql.down.contains("ALTER COLUMN \"note\" SET NOT NULL"));
    assert!(!sql.up.contains("unrelated"));
    assert!(!sql.down.contains("unrelated"));
}

#[test]
fn comparison_normalizes_aliases_without_losing_modifiers_or_quoted_case() {
    for (left, right) in [
        ("int8", "bigint"),
        ("timestamptz", "timestamp with time zone"),
    ] {
        assert_eq!(rust_types::normalized(left), rust_types::normalized(right));
    }
    assert_ne!(
        rust_types::normalized("numeric(10,2)"),
        rust_types::normalized("numeric")
    );
    assert_ne!(
        rust_types::normalized("\"Money\""),
        rust_types::normalized("\"money\"")
    );
}
