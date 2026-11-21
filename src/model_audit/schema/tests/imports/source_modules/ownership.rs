//! Only unambiguous, unconditional module declarations establish source ownership.
use super::{storage, CODEC};

#[test]
fn undeclared_conditional_or_ambiguous_modules_stay_unknown() {
    for root in [
        "mod rows;",
        "mod rows; #[cfg(feature = \"codec\")] mod codec;",
        "mod rows; #[path = \"codec.rs\"] mod codec;",
        "mod rows; mod codec; mod codec;",
        "mod rows; struct rows; mod codec;",
        "#[cfg(feature = \"rows\")] mod rows; mod codec;",
        "#![cfg(feature = \"library\")] mod rows; mod codec;",
    ] {
        assert_eq!(
            storage(
                &[("src/lib.rs", root), ("src/codec.rs", CODEC)],
                "",
                "crate::codec::State"
            ),
            None
        );
    }
}
