//! Fixture selectors exercise keyword, enum-key and composite-name ambiguity.

use crate::database::introspection::{Table, Unique};
use crate::generators::rust::tests::support::required;

pub(super) fn append(table: &mut Table) {
    for column in ["type", "type_"] {
        table.uniques.push(Unique {
            name: format!("{}_{column}_key", table.name),
            columns: vec![column.into()],
        });
    }
    if table.name == "enum_key" {
        table.primary_key = vec!["state".into()];
        table.uniques.push(Unique {
            name: "state_key".into(),
            columns: vec!["state".into()],
        });
    }
    if table.name == "foo" {
        for column in ["a", "b", "a_and_b"] {
            table.columns.push(required(column, "text"));
        }
        // Different selectors share a suffix and collide with another table's CRUD.
        for columns in [vec!["a", "b"], vec!["a_and_b"]] {
            table.uniques.push(Unique {
                name: columns.join("_"),
                columns: columns.into_iter().map(str::to_string).collect(),
            });
        }
    }
}
