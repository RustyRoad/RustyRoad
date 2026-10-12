//! Scaffolding the crate-root `db_pool` module that generated models query through.
//!
//! Every generated CRUD method calls `crate::db_pool::fetch_one(...)` and friends, so the
//! models only compile once the application has that module. RustyRoad provides the pool
//! itself; the scaffold is a one-line re-export, written once and then owned by the
//! developer, so an application that wants its own pool can replace the body freely.

use crate::generators::ownership::{write_owned, Outcome, Ownership};
use std::io;
use std::path::{Path, PathBuf};

/// The module name the generated models reference.
pub const MODULE: &str = "db_pool";

/// The scaffold: RustyRoad's shared pool, re-exported under the name models expect.
const CONTENTS: &str = "\
//! The database pool generated models query through, as `crate::db_pool::fetch_one(...)`.
//!
//! Scaffolded by `rustyroad pull --models` and then yours: `pull` never overwrites it
//! without `--force`. The pool connects on first use from `DATABASE_URL`, the
//! `DATABASE_*` variables, or `rustyroad.toml`; call `init_shared_pool()` at startup to
//! fail fast, or `set_pool(...)` to install a pool you built yourself.

pub use rustyroad::database::pool::*;
";

/// What scaffolding the pool module did.
pub struct PoolModule {
    pub outcome: Outcome,
    /// The crate root that should declare `mod db_pool;` but does not, if any.
    pub undeclared_in: Option<PathBuf>,
}

/// Writes `db_pool.rs` beside the crate root that owns the models folder `out`.
///
/// The crate source folder is taken to be `out`'s parent, which holds for the default
/// `./src/models` and for any `--models-out` one level below `src`.
pub fn write(out: &Path, force: bool) -> io::Result<PoolModule> {
    let source = out.parent().unwrap_or(Path::new("."));
    let outcome = write_owned(
        source,
        &format!("{MODULE}.rs"),
        CONTENTS,
        Ownership::Scaffold,
        force,
    )?;

    let undeclared_in = crate_root(source).filter(|root| {
        !std::fs::read_to_string(root)
            .is_ok_and(|contents| super::writer::declares(&contents, MODULE))
    });

    Ok(PoolModule {
        outcome,
        undeclared_in,
    })
}

/// Returns the crate root in `source`: `lib.rs` when the crate has one, else `main.rs`.
fn crate_root(source: &Path) -> Option<PathBuf> {
    ["lib.rs", "main.rs"]
        .into_iter()
        .map(|name| source.join(name))
        .find(|path| path.is_file())
}
