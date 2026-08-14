//! A table without a usable single-column key.

use super::support::{composite, from_tables, resolve};
use crate::generators::tetherscript::render;

#[test]
fn a_composite_key_table_gets_a_listing_but_no_keyed_operations() {
    let schema = from_tables(vec![composite()]);
    let model = resolve(&composite(), &schema);
    let files = render::files(&model);

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
    assert!(named("read.tether").contains("fn all()"));
    assert!(!named("read.tether").contains("fn find(key)"));
    assert!(named("update.tether").contains("No update was generated"));
    assert!(named("delete.tether").contains("No delete was generated"));

    // The root must not export or forward to functions that were not generated.
    assert!(!named("mod.tether").contains("export find"));
    assert!(!named("mod.tether").contains("export delete"));
}
