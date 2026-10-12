//! A composite unique key keeps the catalog's column order in its generated functions.
//!
//! Sorting the columns made `UNIQUE (tenant_id, slug)` take `slug` first, so same-typed
//! arguments could be swapped without the compiler noticing.

use super::support::{from_tables, products, required};
use crate::database::introspection::Unique;
use crate::generators::rust::write_api;

#[test]
fn composite_key_functions_follow_catalog_column_order() {
    let mut table = products();
    table.columns.push(required("tenant_id", "text"));
    table.columns.push(required("slug", "text"));
    table.uniques.push(Unique {
        name: "products_tenant_slug_key".to_string(),
        columns: vec!["tenant_id".to_string(), "slug".to_string()],
    });
    // The same key declared in the other order is one target, not two.
    table.uniques.push(Unique {
        name: "products_slug_tenant_key".to_string(),
        columns: vec!["slug".to_string(), "tenant_id".to_string()],
    });

    let out = super::support::scratch("key-order");
    write_api(&out, &from_tables(vec![table]), false).expect("write should succeed");
    let repositories = std::fs::read_to_string(out.join("repositories.rs")).unwrap();

    assert!(repositories.contains("find_products_by_tenant_id_and_slug"));
    assert!(!repositories.contains("by_slug_and_tenant_id"));

    let _ = std::fs::remove_dir_all(&out);
}
