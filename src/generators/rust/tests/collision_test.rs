//! Module naming when singularization would collide.
//!
//! A schema holding both `user` and `users` is legal and real — the spotlessbinco database has
//! eight such pairs. Singularizing each table independently mapped both to `user`, so the
//! second model silently overwrote the first: one table lost its model entirely, and the
//! survivor carried the other's SQL. 761 tables produced only 753 modules.

use super::collision_fixture::{directories, pair};
use super::support::scratch;
use crate::generators::rust::write;

#[test]
fn a_singular_and_plural_pair_both_get_their_own_module() {
    let out = scratch("collision");
    let layout = write(&out, &pair(), false).expect("write should succeed");

    // Two tables must produce two modules. Collapsing them loses a table's model.
    assert_eq!(directories(&out).len(), 2, "expected one module per table");
    assert!(layout.undeclared.is_empty());

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn every_module_is_declared_exactly_once() {
    let out = scratch("collision-decl");
    write(&out, &pair(), false).expect("write should succeed");

    let parent = std::fs::read_to_string(out.join("mod.rs")).unwrap();
    let mut declarations: Vec<&str> = parent
        .lines()
        .filter(|line| line.starts_with("pub mod "))
        .collect();

    declarations.sort_unstable();
    let count = declarations.len();
    declarations.dedup();

    assert_eq!(count, declarations.len(), "duplicate declaration: {parent}");
    assert_eq!(count, 2, "expected two declarations: {parent}");

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn each_module_addresses_its_own_table() {
    let out = scratch("collision-sql");
    write(&out, &pair(), false).expect("write should succeed");

    // Whatever the modules are named, between them they must target both tables exactly once.
    // A collision leaves one table unreachable while the other is queried twice.
    let mut targeted: Vec<String> = directories(&out)
        .iter()
        .map(|path| {
            let read = std::fs::read_to_string(path.join("read.rs")).unwrap();
            if read.contains("FROM users") {
                "users".to_string()
            } else {
                "user".to_string()
            }
        })
        .collect();
    targeted.sort();

    assert_eq!(targeted, vec!["user".to_string(), "users".to_string()]);
    let _ = std::fs::remove_dir_all(&out);
}
