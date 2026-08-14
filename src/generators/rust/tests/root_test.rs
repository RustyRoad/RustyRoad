//! The module root: the struct, its derives, and its constructors.

use super::rendered::file;

#[test]
fn the_module_root_declares_the_struct_and_its_submodules() {
    let root = file("mod.rs");

    assert!(root.contains("pub struct Product {"));
    assert!(root.contains("#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, TS)]"));
    assert!(root.contains("#[ts(export)]"));
    for verb in ["create", "read", "update", "delete"] {
        assert!(root.contains(&format!("mod {verb};")), "missing mod {verb}");
    }
}

#[test]
fn fields_carry_their_resolved_types() {
    let root = file("mod.rs");

    assert!(root.contains("pub id: i32,"));
    assert!(root.contains("pub stripe_product_id: String,"));
    assert!(root.contains("pub description: Option<String>,"));
    assert!(root.contains("pub attributes: Vec<String>,"));
    assert!(root.contains("pub price: Option<f64>,"));
    assert!(root.contains("pub created_at: Option<chrono::NaiveDateTime>,"));
}

#[test]
fn the_constructor_omits_database_assigned_columns() {
    let root = file("mod.rs");
    let signature = root
        .split("pub fn new(")
        .nth(1)
        .expect("new should be generated");
    let parameters = signature.split(") -> Self").next().unwrap();

    // The database assigns these three, so accepting them would mislead the caller.
    // Matched with the leading tab so `stripe_product_id:` does not satisfy `id:`.
    assert!(!parameters.contains("\tid:"));
    assert!(!parameters.contains("\tcreated_at:"));
    assert!(!parameters.contains("\tupdated_at:"));
    assert!(parameters.contains("stripe_product_id: String,"));
}

#[test]
fn a_table_becomes_a_singular_module_and_type() {
    use super::support::{products, resolve, schema};

    let schema = schema();
    let model = resolve(&products(), &schema);

    assert_eq!(model.module, "product");
    assert_eq!(model.name, "Product");
    // SQL must still address the real table.
    assert_eq!(model.table, "products");
}
