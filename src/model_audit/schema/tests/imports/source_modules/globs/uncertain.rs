//! Extracted fixtures retain their original assertions and inputs.
use super::storage;

#[test]
fn type_exports_and_uncertain_helper_contents_stay_unknown() {
    for helper in [
        "pub type String = external::String;",
        "pub use external::String;",
        "pub use external::*;",
        "macro_rules! helper { () => {} }",
        "#[expand] pub fn helper() {}",
        "#![expand] pub fn helper() {}",
        "mod nested {}",
        "#[cfg(feature = \"x\")] pub fn helper() {}",
    ] {
        assert_eq!(
            storage(
                &[
                    ("src/lib.rs", "mod rows; mod helper;"),
                    ("src/helper.rs", helper)
                ],
                "use crate::helper::*;",
                "String"
            ),
            None,
            "{helper}"
        );
    }
}
