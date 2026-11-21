//! Module ownership and attributes must not erase wildcard uncertainty.
use super::storage;

#[test]
fn conditional_custom_or_ambiguous_ownership_cannot_clear_globs() {
    for root in [
        "mod rows; #[cfg(feature = \"x\")] mod helper;",
        "mod rows; #[expand] mod helper;",
        "#[expand] mod rows; mod helper;",
        "#![expand] mod rows; mod helper;",
        "mod rows; mod helper; mod helper;",
        "mod rows; #[path = \"helper.rs\"] mod helper;",
    ] {
        assert_eq!(
            storage(
                &[
                    ("src/lib.rs", root),
                    ("src/helper.rs", "pub fn helper() {}")
                ],
                "use crate::helper::*;",
                "String"
            ),
            None,
            "{root}"
        );
    }
}

#[test]
fn inline_helpers_cannot_bypass_unproven_owning_modules() {
    for root in [
        "#[cfg(feature = \"x\")] mod rows;",
        "mod rows; mod rows;",
        "#[path = \"rows.rs\"] mod rows;",
        "mod unrelated;",
    ] {
        assert_eq!(
            storage(
                &[("src/lib.rs", root)],
                "mod helper { pub fn value() {} } use self::helper::*;",
                "Option<String>"
            ),
            None,
            "{root}"
        );
    }
}
