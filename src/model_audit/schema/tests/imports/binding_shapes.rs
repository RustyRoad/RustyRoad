//! Closed aliases cannot acquire undeclared generic arguments or namespaces.
use super::audits;

/// Example: parent-qualified closed aliases retain their declared arguments.
#[test]
fn resolves_closed_aliases_from_parent_scopes() {
    let result = audits(
        r#"
        type Keys = Vec<uuid::Uuid>;
        mod inner {
            use super::Keys as ParentKeys;
            struct Row { id: i32, keys: ParentKeys }
        }
        "#,
    );
    let row = result.last().unwrap();
    assert!(row.blockers.is_empty(), "{:?}", row.blockers);
    assert_eq!(row.desired_columns[1].sql_type, "uuid[]");
    assert!(row.proposal.is_some());
}

mod rejected;

mod containers;
