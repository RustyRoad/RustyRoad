//! Wildcard helpers must prove they cannot export a type into the row's scope.
mod lints;
mod ownership;
use super::storage;

#[test]
fn declared_value_only_helpers_keep_prelude_codecs() {
    let helper = r#"
        use external::String;
        impl Other { fn helper() {} }
        pub fn helper() {} pub const VALUE: i32 = 1; pub static FLAG: bool = true;
    "#;
    for import in ["use crate::helper::*;", "pub use super::helper::*;"] {
        assert_eq!(
            storage(
                &[
                    ("src/lib.rs", "mod rows; mod helper;"),
                    ("src/helper.rs", helper)
                ],
                import,
                "Option<String>"
            )
            .as_deref(),
            Some("text")
        );
    }
}

mod uncertain;

mod siblings;
