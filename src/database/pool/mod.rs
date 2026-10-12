//! The shared Postgres pool that generated models run their queries through.
//!
//! Generated models call `crate::db_pool::fetch_one(...)` and friends rather than taking a
//! pool argument, so every call has to reach one process-wide pool. Opening a pool per call,
//! which is what `Database::get_db_pool` does, exhausts connections under load, so this module
//! keeps a single lazily connected pool and exposes thin helpers over it.
//!
//! `rustyroad pull --models` scaffolds `src/db_pool.rs` as a re-export of this module, so the
//! generated code compiles without the application writing any pool code of its own.
//!
//! ```rust,no_run
//! # async fn demo() -> Result<(), sqlx::Error> {
//! let one: i64 =
//!     rustyroad::database::pool::scalar_one(sqlx::query_scalar("SELECT 1::bigint")).await?;
//! # Ok(()) }
//! ```

mod config;
mod locate;
mod query;
mod settings;
mod shared;
mod toml_source;

#[cfg(test)]
mod config_test;
#[cfg(test)]
mod locate_test;
#[cfg(test)]
mod query_test;

pub use config::connect_options;
pub use query::*;
pub use settings::PoolSettings;
pub use shared::{begin, get_or_init_pool, init_shared_pool, pool, set_pool};
