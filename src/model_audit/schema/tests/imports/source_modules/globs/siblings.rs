//! Extracted fixtures retain their original assertions and inputs.
use super::storage;

#[test]
fn unknown_siblings_and_ancestor_globs_stay_unknown() {
    for source in [
        "use super::*;",
        "use crate::*;",
        "use self::*;",
        "use crate::helper::*; use external::*;",
        "#[cfg(feature = \"x\")] use crate::helper::*;",
        "use crate::helper::*; type String = external::String;",
    ] {
        assert_eq!(
            storage(
                &[
                    ("src/lib.rs", "mod rows; mod helper;"),
                    ("src/helper.rs", "pub fn helper() {}")
                ],
                source,
                "String"
            ),
            None,
            "{source}"
        );
    }
}
