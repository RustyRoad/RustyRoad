//! Relative module-self aliases retain source-backed codec ownership.
use super::{storage, CODEC};

#[test]
fn relative_module_self_aliases_keep_their_source_contract() {
    let files = [
        ("src/lib.rs", "mod rows; mod codec;"),
        ("src/codec.rs", CODEC),
    ];
    for (source, ty) in [
        ("use crate::codec::{self};", "codec::State"),
        ("use crate::codec::{self as local};", "local::State"),
        (
            "use crate::codec::{self as local}; use local::State;",
            "State",
        ),
    ] {
        assert_eq!(
            storage(&files, source, ty).as_deref(),
            Some("\"state\""),
            "{source}"
        );
    }
}
