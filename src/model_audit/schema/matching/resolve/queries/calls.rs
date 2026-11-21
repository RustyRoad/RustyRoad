//! Recognize source-resolved SQLx constructors with literal whole-row SQL.
mod constructor;
mod database;
mod function;
mod literal;
mod macro_call;
mod row;

pub(super) use constructor::query;
pub(super) use function::function;
pub(super) use macro_call::macro_call;
pub(super) use row::row;
