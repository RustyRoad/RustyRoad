//! Absolute imports must never borrow a same-named local module's SQLx contract.
use super::{storage, CODEC};

#[test]
fn absolute_imports_do_not_resolve_against_local_modules() {
    let files = [("src/lib.rs", "mod rows;")];
    for (import, ty) in [
        ("use ::codec::State;", "State"),
        ("use ::codec::{State};", "State"),
        ("use ::codec::{State as Status};", "Status"),
        ("use ::codec::{self};", "codec::State"),
        ("use ::codec::{self as external};", "external::State"),
        (
            "use ::codec::{self as external}; use external::State;",
            "State",
        ),
    ] {
        let source = format!("mod codec {{ {CODEC} }} {import}");
        assert_eq!(storage(&files, &source, ty), None, "{import}");
    }
}

#[test]
fn absolute_reexports_do_not_resolve_against_crate_modules() {
    let files = [
        ("src/lib.rs", "mod rows; mod codec; pub use ::codec::State;"),
        ("src/codec.rs", CODEC),
    ];
    assert_eq!(storage(&files, "use crate::State;", "State"), None);
}
