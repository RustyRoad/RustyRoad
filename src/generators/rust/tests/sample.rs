//! Writes a sample generated model to /tmp for inspection. Run with:
//!   cargo test --lib generators::rust::tests::sample -- --ignored --nocapture

use super::support::{enum_type, required, serial_key, with_enums};
use crate::database::introspection::Table;
use crate::generators::rust::write;

#[test]
#[ignore = "writes a durable sample for manual inspection"]
fn write_sample_to_tmp() {
    let table = Table {
        name: "products".to_string(),
        columns: vec![
            serial_key("id"),
            required("name", "text"),
            required("status", "product_status"),
            crate::generators::rust::tests::support::column("prior_status", "product_status"),
        ],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    };
    let schema = with_enums(
        vec![table],
        vec![enum_type("product_status", &["active", "back_ordered"])],
    );

    let out = std::path::PathBuf::from("/tmp/rustyroad-rust-sample");
    let _ = std::fs::remove_dir_all(&out);

    write(&out, &schema, false).expect("write should succeed");

    println!("wrote sample to {}", out.display());
}
