//! Numeric disambiguation must reserve other objects' preferred spellings.

use crate::database::introspection::Table;
use crate::generators::rust::allocation::Names;
use crate::generators::rust::tests::support::{from_tables, required};

#[test]
fn allocated_suffixes_do_not_steal_other_preferred_names() {
    let schema = from_tables(
        ["foo", "foo_", "foo2"]
            .into_iter()
            .map(|name| Table {
                name: name.into(),
                columns: ["a-b", "a_b", "a_b_2"]
                    .into_iter()
                    .map(|name| required(name, "text"))
                    .collect(),
                ..Table::default()
            })
            .collect(),
    );
    let names = Names::new(&schema);
    assert_eq!(names.row(schema.table("foo").unwrap()), "Foo");
    assert_eq!(names.row(schema.table("foo_").unwrap()), "Foo3");
    assert_eq!(names.row(schema.table("foo2").unwrap()), "Foo2");
    let table = &schema.tables[0];
    assert_eq!(names.field(table, "a-b"), "a_b");
    assert_eq!(names.field(table, "a_b"), "a_b_3");
    assert_eq!(names.field(table, "a_b_2"), "a_b_2");
    let variants = Names::variants(&["active".into(), "active_".into(), "active_variant2".into()]);
    assert_eq!(variants["active_"], "ActiveVariant3");
    assert_eq!(variants["active_variant2"], "ActiveVariant2");
}
