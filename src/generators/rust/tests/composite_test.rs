//! A table without a usable single-column key.

use super::support::{composite, from_tables, resolve};
use crate::generators::rust::render;

#[test]
fn a_composite_key_table_gets_a_listing_but_no_keyed_methods() {
    let schema = from_tables(vec![composite()]);
    let model = resolve(&composite(), &schema);
    let files = render::files(&model, &schema);

    let named = |name: &str| {
        files
            .iter()
            .find(|file| file.name == name)
            .unwrap()
            .contents
            .clone()
    };

    // A two-column key cannot address a row with one bind, so rather than emitting a
    // statement that targets the wrong rows, the reason is stated in the file.
    assert!(named("read.rs").contains("pub async fn all()"));
    assert!(!named("read.rs").contains("pub async fn find("));
    assert!(named("update.rs").contains("No update was generated"));
    assert!(named("delete.rs").contains("No delete was generated"));
}
