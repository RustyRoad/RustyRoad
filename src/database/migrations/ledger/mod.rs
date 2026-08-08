//! Idempotency ledger for applied migrations.
//!
//! RustyRoad records every migration it runs in the `_rustyroad_migrations` table.
//! This module owns that table end to end: creating it ([`schema`]), deciding a
//! migration's identity ([`identity`]), reading current state ([`state`]), and
//! recording an executed migration ([`record`]).
//!
//! Reading the ledger before executing is what makes `rustyroad migration all`
//! idempotent. Without it, every migration on disk is re-executed on each run,
//! which fails on any long-lived database where later migrations supersede earlier
//! ones.

mod dialect;
mod exec;
mod identity;
mod record;
mod repair;
mod schema;
mod sql;
mod state;

/// Direction recorded for a migration that has been applied.
const DIRECTION_UP: &str = "up";
/// Direction recorded for a migration that has been rolled back.
const DIRECTION_DOWN: &str = "down";

pub use identity::{display_name, identities_match, latest_status, ledger_id_for_dir};
pub use record::{direction_label, record};
pub use repair::{repair, RepairReport};
pub use schema::{ensure_table, LEDGER_TABLE};
pub use state::{is_applied, should_skip};
