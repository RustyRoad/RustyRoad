//! Conflicting type declarations and reexports cannot supply codecs.
use super::{storage, CODEC};

#[test]
fn conflicting_or_external_types_stay_unknown() {
    let files = [
        ("src/lib.rs", "mod rows; mod codec;"),
        ("src/codec.rs", CODEC),
    ];
    assert_eq!(storage(&files, "use external::State;", "State"), None);
    assert_eq!(
        storage(&files, "use crate::codec::State; struct State;", "State"),
        None
    );
    assert_eq!(
        storage(
            &[
                ("src/lib.rs", "mod rows; mod codec;"),
                ("src/codec.rs", "pub use crate::codec::State;")
            ],
            "",
            "crate::codec::State"
        ),
        None
    );
    assert_eq!(
        storage(
            &[
                ("src/lib.rs", "mod rows; mod codec;"),
                ("src/codec.rs", CODEC),
                ("src/codec/mod.rs", CODEC)
            ],
            "",
            "crate::codec::State"
        ),
        None
    );
}
