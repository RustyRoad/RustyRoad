//! Collision handling must preserve ordinary existing API spellings.

use crate::generators::rust::allocation::Names;
use crate::generators::rust::tests::support;

#[test]
fn ordinary_identifiers_keep_their_existing_spelling() {
    let schema = support::schema();
    let names = Names::new(&schema);
    let table = &schema.tables[0];
    assert_eq!(names.row(table), "Products");
    assert_eq!(names.create(table), "NewProducts");
    assert_eq!(names.patch(table), "PatchProducts");
    assert_eq!(names.function(table, "create"), "create_products");
}
