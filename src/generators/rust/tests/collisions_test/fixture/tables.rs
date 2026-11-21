//! Common collision columns and input defaults for the catalog fixture.

use super::selectors;
use crate::database::introspection::Table;
use crate::generators::rust::tests::support::required;

pub(super) fn build(name: &str) -> Table {
    let mut id = required("id", "integer");
    id.auto_increment = true;
    let mut table = Table {
        name: name.into(),
        columns: vec![
            id,
            required("type", "text"),
            required("type_", "text"),
            required("type__2", "text"),
            required("updated_at", "timestamp"),
            required("state", "proposal"),
            required("data", "jsonb"),
            required("data_", "jsonb"),
        ],
        primary_key: vec!["id".into()],
        ..Table::default()
    };
    selectors::append(&mut table);
    table
}
