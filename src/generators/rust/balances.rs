//! Signed ledger totals keep conditional arithmetic and expiration inside PostgreSQL.
mod query;
mod validation;
use super::allocation::Names;
use crate::database::introspection::Schema;
use std::{collections::BTreeMap, io};

/// A signed integer balance over two polarity labels and optional expiration.
/// Additional ownership/active-state predicates use the generated table filter.
/// # Examples
/// ```toml
/// [models.rust_balances.available_credit_balance]
/// table = "credit_ledgers"
/// polarity = "transaction_type"
/// amount = "amount_cents"
/// positive = "credit"
/// negative = "debit"
/// expires = "expires_at"
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub table: String,
    pub polarity: String,
    pub amount: String,
    pub positive: String,
    pub negative: String,
    #[serde(default)]
    pub expires: Option<String>,
}

/// Validates balance contracts before emitting schema-derived, bound operations.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    balances: &BTreeMap<String, Balance>,
) -> io::Result<String> {
    let mut output = String::new();
    for (name, balance) in balances {
        validation::validate(schema, names, name, balance)?;
        output.push_str(&query::render(schema, names, name, balance));
    }
    Ok(output)
}
