//! Catalog iteration order must not choose which object receives a suffix.

use super::fixture;
use crate::generators::rust::allocation::Names;

#[test]
fn allocation_is_independent_of_catalog_iteration_order() {
    let schema = fixture::schema();
    let original = Names::new(&schema);
    let mut reversed = schema.clone();
    reversed.tables.reverse();
    reversed.enums.reverse();
    for table in &mut reversed.tables {
        table.columns.reverse();
        table.uniques.reverse();
    }
    let reordered = Names::new(&reversed);
    for table in &schema.tables {
        assert_eq!(original.row(table), reordered.row(table));
        assert_eq!(original.create(table), reordered.create(table));
        assert_eq!(original.patch(table), reordered.patch(table));
        for column in &table.columns {
            assert_eq!(
                original.field(table, &column.name),
                reordered.field(table, &column.name)
            );
            assert_eq!(
                original.selector(table, "latest", std::slice::from_ref(&column.name)),
                reordered.selector(table, "latest", std::slice::from_ref(&column.name))
            );
        }
    }
    for item in &schema.enums {
        assert_eq!(
            original.enum_type(&item.name),
            reordered.enum_type(&item.name)
        );
    }
}
