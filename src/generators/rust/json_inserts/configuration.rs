//! Declarative JSON-derived insert configuration; no arbitrary SQL is accepted.
use std::collections::BTreeMap;

/// Binds input columns and derives nullable integers from one JSONB input.
/// Conflict columns must have a catalog primary key or unconditional unique key.
/// # Examples
/// ```toml
/// [models.rust_json_inserts.queue_event]
/// table = "analytics_events"
/// columns = ["event_name", "event_id", "event_data"]
/// json_column = "event_data"
/// conflict = ["event_id"]
/// booleans = { processed = false }
/// integers = { customer_id = { key = "customer_id" } }
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonInsert {
    pub table: String,
    pub columns: Vec<String>,
    pub json_column: String,
    pub conflict: Vec<String>,
    #[serde(default)]
    pub booleans: BTreeMap<String, bool>,
    pub integers: BTreeMap<String, JsonInteger>,
}

/// Derives an integer only from a JSON number; other JSON types yield SQL NULL.
/// When requested, a non-null input for the same target takes precedence.
/// PostgreSQL performs the cast and reports fractional or out-of-range values.
/// # Examples
/// ```toml
/// [models.rust_json_inserts.queue_event]
/// table = "analytics_events"
/// columns = ["event_name", "event_id", "event_data", "crm_lead_id"]
/// json_column = "event_data"
/// conflict = ["event_id"]
/// integers = { crm_lead_id = { key = "crm_lead_id", prefer_input = true } }
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonInteger {
    pub key: String,
    #[serde(default)]
    pub prefer_input: bool,
}
