//! Explicit scalar codec contracts for individual catalog columns.

/// A scoped storage assertion and external codec; never inherited by other columns.
/// The namespace must match the namespace actually supplied to catalog pull.
/// # Examples
/// ```toml
/// [[models.rust_column_types]]
/// schema = "public"
/// table = "parcels"
/// column = "geometry"
/// sql_type = "geometry(Polygon,4326)"
/// nullable = false
/// rust = "geography::DbPolygon"
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColumnOverride {
    /// Exact PostgreSQL namespace, not an OpenAPI representation.
    pub schema: String,
    /// Exact relation name in the selected pull scope.
    pub table: String,
    /// Exact catalog column name, before Rust identifier allocation.
    pub column: String,
    /// Expected scalar storage declaration, including all typmods and SRID.
    pub sql_type: String,
    /// External, non-generic Rust codec path.
    pub rust: String,
    /// Optional assertion against catalog nullability; does not change wrappers.
    pub nullable: Option<bool>,
    /// Optional Rust type describing actual serialization for OpenAPI only.
    pub openapi_type: Option<String>,
}
