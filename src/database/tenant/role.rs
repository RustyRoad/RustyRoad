use super::{Identity, ProvisionError};
use crate::database::Database;
use sqlx::{AssertSqlSafe, Connection, PgConnection};

/// Creates a scoped login and ownership marker in one transaction, or verifies it.
/// Existing roles are never adopted, elevated or given a replacement password.
/// # Examples
/// ```no_run
/// # async fn example(c: &mut sqlx::PgConnection, a: &rustyroad::database::Database,
/// # t: &rustyroad::database::Database) -> Result<(), rustyroad::database::tenant::ProvisionError> {
/// let id = rustyroad::database::tenant::Identity::validate(a, t)?;
/// rustyroad::database::tenant::role::ensure(c, &id, t).await?;
/// # Ok(()) }
/// ```
pub async fn ensure(c: &mut PgConnection, id: &Identity, tenant: &Database)
    -> Result<(), ProvisionError>
{
    let found: Option<(String, bool)> = sqlx::query_as(
        "SELECT COALESCE(shobj_description(oid, 'pg_authid'), ''), \
         rolcanlogin AND NOT rolsuper AND NOT rolcreatedb AND NOT rolcreaterole \
         AND NOT rolreplication AND NOT rolbypassrls AND NOT rolinherit \
         AND NOT EXISTS(SELECT 1 FROM pg_auth_members m WHERE m.member = pg_roles.oid) \
         FROM pg_roles WHERE rolname = $1",
    ).bind(&id.role).fetch_optional(&mut *c).await
        .map_err(|e| ProvisionError::database("role_inspection", &e))?;
    if let Some((marker, restricted)) = found {
        return if marker == id.marker && restricted { Ok(()) }
            else { Err(ProvisionError::invalid("existing_role_conflict")) };
    }
    let mut transaction = c.begin().await
        .map_err(|e| ProvisionError::database("role_transaction", &e))?;
    // These must succeed before any credential-bearing statement is submitted.
    sqlx::raw_sql(
        "SET LOCAL log_statement = 'none'; SET LOCAL log_min_duration_statement = -1; \
         SET LOCAL log_min_duration_sample = -1; SET LOCAL log_statement_sample_rate = 0; \
         SET LOCAL log_min_error_statement = 'panic'; \
         SET LOCAL log_parameter_max_length = 0; SET LOCAL log_parameter_max_length_on_error = 0",
    ).execute(&mut *transaction).await
        .map_err(|e| ProvisionError::database("credential_log_guard", &e))?;
    let statement = format!(
        "CREATE ROLE \"{}\" LOGIN PASSWORD '{}' NOSUPERUSER NOCREATEDB \
         NOCREATEROLE NOINHERIT NOREPLICATION NOBYPASSRLS",
        id.role, tenant.password,
    );
    sqlx::query(AssertSqlSafe(statement)).execute(&mut *transaction).await
        .map_err(|e| ProvisionError::database("role_creation", &e))?;
    let marker = format!("COMMENT ON ROLE \"{}\" IS '{}'", id.role, id.marker);
    sqlx::query(AssertSqlSafe(marker)).execute(&mut *transaction).await
        .map_err(|e| ProvisionError::database("role_marker", &e))?;
    transaction.commit().await.map_err(|e| ProvisionError::database("role_commit", &e))
}
