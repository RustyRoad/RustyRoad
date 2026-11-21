//! Opt-in input codecs, independent of projection RETURNING codecs.

/// Generate a projection-specific input without changing canonical New inputs.
/// All nullable fields use Option<Option<T>>: omit, SQL NULL, or a bound value.
/// Cast lists affect only input parameters; unlisted fields retain catalog codecs.
/// # Examples
/// ```toml
/// [models.rust_projections.insert_snapshot]
/// table = "route_optimization_snapshots"
/// columns = ["id", "planned_total_cost"]
/// insert = true
/// insert_input = { numeric_float = ["planned_total_cost"] }
/// numeric_float = ["planned_total_cost"]
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InsertInput {
    /// Bind f64 as float8, then cast to numeric in PostgreSQL before assignment.
    #[serde(default)]
    pub numeric_float: Vec<String>,
    /// Bind String as text, then cast to the catalog inet/cidr type in PostgreSQL.
    #[serde(default)]
    pub network_text: Vec<String>,
}
