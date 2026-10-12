//! The `db_pool` scaffold the generated models query through.

use super::support::scratch;
use crate::generators::rust::write_pool_module;
use std::fs;

#[test]
fn scaffolds_a_rustyroad_reexport_beside_the_models_folder() {
    let src = scratch("pool-scaffold");
    fs::write(src.join("main.rs"), "mod models;\nfn main() {}\n").unwrap();

    let module = write_pool_module(&src.join("models"), false).expect("write should succeed");

    assert!(!module.outcome.is_preserved());
    let contents = fs::read_to_string(src.join("db_pool.rs")).unwrap();
    assert!(
        contents.contains("pub use rustyroad::database::pool::*;"),
        "{contents}"
    );
    assert_eq!(module.undeclared_in, Some(src.join("main.rs")));
}

#[test]
fn keeps_an_existing_pool_module_and_accepts_its_declaration() {
    let src = scratch("pool-kept");
    fs::write(src.join("lib.rs"), "pub mod db_pool;\npub mod models;\n").unwrap();
    fs::write(src.join("main.rs"), "fn main() {}\n").unwrap();
    fs::write(src.join("db_pool.rs"), "// hand-written pool\n").unwrap();

    let module = write_pool_module(&src.join("models"), false).expect("write should succeed");

    assert!(module.outcome.is_preserved());
    assert_eq!(
        fs::read_to_string(src.join("db_pool.rs")).unwrap(),
        "// hand-written pool\n"
    );
    assert_eq!(
        module.undeclared_in, None,
        "lib.rs declares it, so nothing is missing"
    );
}

#[test]
fn force_replaces_the_pool_module() {
    let src = scratch("pool-force");
    fs::write(src.join("db_pool.rs"), "// stale\n").unwrap();

    let module = write_pool_module(&src.join("models"), true).expect("write should succeed");

    assert!(!module.outcome.is_preserved());
    assert!(fs::read_to_string(src.join("db_pool.rs"))
        .unwrap()
        .contains("rustyroad::database::pool"));
    assert_eq!(
        module.undeclared_in, None,
        "no crate root, so nothing to report"
    );
}
