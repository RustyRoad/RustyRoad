//! Offline CLI workspace fixtures with no build-script or toolchain execution.
use std::process::{Command, Output};

/// Create a dependency-free workspace; e.g. `fixture()` has two unrelated Row types.
pub(super) fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src/db")).unwrap();
    std::fs::write(
        dir.path().join("build.rs"),
        "fn main() { std::fs::write(\"BUILD_SCRIPT_RAN\", \"unexpected\").unwrap(); }",
    )
    .unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "[package]\nname = \"model-audit-fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n").unwrap();
    std::fs::write(dir.path().join("src/lib.rs"), "pub mod models;\npub mod db { pub mod models; }\nuse models::Row as Renamed;\npub fn consume(row: Renamed) -> i32 { row.id }\nmod unrelated { pub struct Row { pub value: bool } }\npub type Input = Renamed;\npub fn consume_alias(row: Input) -> i32 { row.id }\n").unwrap();
    std::fs::write(
        dir.path().join("src/models.rs"),
        "pub struct Row { pub id: i32 }\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("src/db/models.rs"),
        "pub struct Rows { pub id: i32, pub active: bool }\n",
    )
    .unwrap();
    dir
}

/// Audit offline; e.g. `run(dir.path(), &["--all-sources"])` cannot invoke Cargo.
pub(super) fn run(root: &std::path::Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .args(["--format", "json", "model", "audit", "--root"])
        .arg(root)
        .arg("--offline")
        .env("PATH", root.join("empty-tool-path"))
        .args(args)
        .output()
        .unwrap()
}
