//! Configuration for schema-owned narrow result tuples.
pub use super::insert::configuration::InsertInput;
/// Ordered result columns for a filtered read or an atomic projected insert.
/// Inserts default to New; insert_input opts into a dedicated tri-state input.
/// Timestamp overrides apply only to omitted eligible fields, never supplied values.
/// # Examples
/// ```toml
/// [models.rust_projections.insert_consent]
/// table = "sms_consent_audit"
/// columns = ["id", "consent_timestamp", "ip_address"]
/// insert = true
/// database_timestamps = ["consent_timestamp"]
/// network_host = ["ip_address"]
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Projection {
    pub table: String,
    pub columns: Vec<String>,
    /// Emit an atomic INSERT RETURNING operation instead of a filtered read.
    #[serde(default)]
    pub insert: bool,
    /// Opt into a generated tri-state input with independent SQL-side input casts.
    pub insert_input: Option<InsertInput>,
    /// Omitted timestamp inputs assigned transaction time during insertion.
    #[serde(default)]
    pub database_timestamps: Vec<String>,
    /// Selected JSON columns returned using PostgreSQL's text representation.
    #[serde(default)]
    pub json_text: Vec<String>,
    /// Selected numeric columns returned using PostgreSQL's float8 conversion.
    #[serde(default)]
    pub numeric_float: Vec<String>,
    /// Selected inet/cidr columns returned with PostgreSQL HOST formatting.
    #[serde(default)]
    pub network_host: Vec<String>,
    /// Accept generated typed comparisons alongside equality predicates.
    #[serde(default)]
    pub comparisons: bool,
    /// Database ordering for unjoined projections; applied before the row limit.
    #[serde(default)]
    pub order: Vec<ProjectionOrder>,
    #[serde(default)]
    pub join: Option<JoinProjection>,
}
/// Additional selected columns from a relation identified by an existing foreign key.
/// The join is inner; NULL or absent relationships yield no matching result.
/// # Examples
/// ```toml
/// [models.rust_projections.campaign_trigger_summary]
/// table = "campaign_triggers"
/// columns = []
/// join = { foreign_key = "campaign_triggers_trigger_id_fkey", columns = ["trigger_id", "trigger_name"], order = [{column = "trigger_name"}] }
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JoinProjection {
    pub foreign_key: String,
    pub columns: Vec<String>,
    #[serde(default)]
    pub order: Vec<ProjectionOrder>,
}
/// Ordering on a catalog column, retaining database collation and NULL rules.
/// # Examples
/// ```toml
/// [models.rust_projections.campaign_trigger_summary]
/// table = "campaign_triggers"
/// columns = []
/// join = { foreign_key = "campaign_triggers_trigger_id_fkey", columns = ["trigger_id"], order = [{column = "trigger_name", descending = false, nulls_first = false}] }
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectionOrder {
    pub column: String,
    #[serde(default)]
    pub descending: bool,
    #[serde(default)]
    pub nulls_first: Option<bool>,
}
