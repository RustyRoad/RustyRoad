//! Filtered input shapes must not independently reallocate their field names.

use super::syntax;
use crate::database::introspection::Table;
use crate::generators::rust::tests::support::{from_tables, required};
use crate::generators::rust::{allocation::Names, models, repositories};

#[test]
fn filtered_inputs_reuse_the_row_field_allocation() {
    let mut key = required("type", "integer");
    key.auto_increment = true;
    let table = Table {
        name: "filtered".into(),
        columns: vec![key, required("type_", "text")],
        primary_key: vec!["type".into()],
        ..Table::default()
    };
    let schema = from_tables(vec![table]);
    let names = Names::new(&schema);
    let models = models::render(&schema, &names);
    let repositories = repositories::render(&schema, &names);
    let field = names.field(&schema.tables[0], "type_");
    assert_eq!(models.matches(&format!("pub {field}:")).count(), 3);
    assert!(repositories.contains(&format!("values.push_bind(&input.{field})")));
    assert!(repositories.contains(&format!("input.{field}.as_ref()")));
    syntax::assert_unique(&models);
}
