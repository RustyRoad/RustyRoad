//! Export exact writer output for a separate real-dependency compiler check.

use super::{fixture, syntax};
use crate::generators::rust::write_api;

#[test]
fn generate_collision_fixture_for_compiler_check() {
    let scratch = tempfile::tempdir().unwrap();
    let out = std::env::var_os("RUSTYROAD_COLLISION_FIXTURE_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| scratch.path().join("generated"));
    let db = out.join("src/db");
    write_api(&db, &fixture::schema(), false).unwrap();
    std::fs::write(out.join("Cargo.toml"), include_str!("compiler.toml")).unwrap();
    std::fs::write(out.join("src/lib.rs"), include_str!("compiler_lib.rs.txt")).unwrap();
    for file in ["models.rs", "repositories.rs", "procedures.rs"] {
        syntax::assert_unique(&std::fs::read_to_string(db.join(file)).unwrap());
    }
}
