use super::{Identity, ProvisionError};
use sqlx::{AssertSqlSafe, PgConnection};

/// Ensures the durable database belongs to the verified, provisioner-marked role.
/// Never reassigns an existing database or deletes resources on failure.
/// # Examples
/// ```no_run
/// # async fn example(c: &mut sqlx::PgConnection, id: &rustyroad::database::tenant::Identity)
/// # -> Result<(), rustyroad::database::tenant::ProvisionError> {
/// rustyroad::database::tenant::database::ensure(c, id).await?;
/// # Ok(()) }
/// ```
pub async fn ensure(c: &mut PgConnection, id: &Identity) -> Result<(), ProvisionError> {
    let found: Option<(String, String)> = sqlx::query_as(
        "SELECT pg_get_userbyid(datdba), COALESCE(shobj_description(oid, 'pg_database'), '') \
         FROM pg_database WHERE datname = $1",
    ).bind(&id.database).fetch_optional(&mut *c).await
        .map_err(|e| ProvisionError::database("database_inspection", &e))?;
    match found {
        Some((owner, marker)) => {
            // Empty marker is the recoverable gap between CREATE DATABASE and COMMENT.
            if owner != id.role || (!marker.is_empty() && marker != id.marker) {
                return Err(ProvisionError::invalid("existing_database_conflict"));
            }
        }
        None => {
            let statement = format!(
                "CREATE DATABASE \"{}\" OWNER \"{}\" TEMPLATE template0", id.database, id.role,
            );
            sqlx::query(AssertSqlSafe(statement)).execute(&mut *c).await
                .map_err(|e| ProvisionError::database("database_creation", &e))?;
        }
    }
    let marker = format!("COMMENT ON DATABASE \"{}\" IS '{}'", id.database, id.marker);
    sqlx::query(AssertSqlSafe(marker)).execute(&mut *c).await
        .map_err(|e| ProvisionError::database("database_marker", &e))?;
    let restrict = format!("REVOKE ALL ON DATABASE \"{}\" FROM PUBLIC", id.database);
    sqlx::query(AssertSqlSafe(restrict)).execute(&mut *c).await
        .map_err(|e| ProvisionError::database("database_public_privileges", &e))?;
    Ok(())
}
