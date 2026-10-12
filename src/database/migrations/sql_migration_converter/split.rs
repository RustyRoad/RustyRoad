//! Quote-aware SQL splitting helpers.
mod body;
mod quote;
mod separator;
pub(super) use body::dollar_body;
pub(super) use separator::split_outside;
