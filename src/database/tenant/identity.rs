use super::ProvisionError;
use crate::database::{Database, DatabaseType};

/// Validated PostgreSQL resource names and their deterministic ownership marker.
/// No credential is stored or formatted by this type.
/// # Examples
/// ```no_run
/// # fn example(admin: &rustyroad::database::Database, tenant: &rustyroad::database::Database)
/// # -> Result<(), rustyroad::database::tenant::ProvisionError> {
/// let id = rustyroad::database::tenant::Identity::validate(admin, tenant)?;
/// assert!(id.marker.starts_with("rustyroad:tenant:v1:"));
/// # Ok(()) }
/// ```
pub struct Identity {
    pub database: String,
    pub role: String,
    pub marker: String,
}

/// Enforces the tenant namespace and prevents cross-cluster or admin-role reuse.
/// # Examples
/// ```no_run
/// # fn example(admin: &rustyroad::database::Database, tenant: &rustyroad::database::Database) {
/// let _result = rustyroad::database::tenant::Identity::validate(admin, tenant);
/// # }
/// ```
impl Identity {
    /// Accepts only rf_ plus 32 lowercase hex digits and matching tenant role names.
    /// Requires a hex credential of at least 32 bytes, suitable for generated secrets.
    /// # Examples
    /// ```no_run
    /// # fn example(admin: &rustyroad::database::Database, tenant: &rustyroad::database::Database)
    /// # -> Result<(), rustyroad::database::tenant::ProvisionError> {
    /// let id = rustyroad::database::tenant::Identity::validate(admin, tenant)?;
    /// assert_eq!(id.database, id.role);
    /// # Ok(()) }
    /// ```
    pub fn validate(admin: &Database, tenant: &Database) -> Result<Self, ProvisionError> {
        if admin.database_type != DatabaseType::Postgres
            || tenant.database_type != DatabaseType::Postgres
            || admin.host != tenant.host || admin.port != tenant.port
            || admin.name == tenant.name || admin.username == tenant.username
        {
            return Err(ProvisionError::invalid("postgres_target_boundary"));
        }
        let suffix = tenant.name.strip_prefix("rf_").unwrap_or("");
        if suffix.len() != 32 || !suffix.bytes().all(|b| b.is_ascii_digit()
            || (b'a'..=b'f').contains(&b)) || tenant.username != tenant.name
        {
            return Err(ProvisionError::invalid("tenant_identity"));
        }
        if tenant.password.len() < 32 || tenant.password.len() > 128
            || !tenant.password.bytes().all(|b| b.is_ascii_hexdigit())
            || tenant.password == admin.password
        {
            return Err(ProvisionError::invalid("tenant_password"));
        }
        Ok(Self {
            database: tenant.name.clone(), role: tenant.username.clone(),
            marker: format!("rustyroad:tenant:v1:{}", tenant.name),
        })
    }
}
