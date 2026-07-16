use super::cleanup;
use super::error::MigrationValidationError;
use super::execute_postgres;
use super::identity::Identity;
use super::model::MigrationFile;
use super::{postgres_create, postgres_drop, postgres_pool};
use crate::database::Database;

pub(super) async fn validate(
    database: &Database,
    id: &Identity,
    migrations: &[MigrationFile],
) -> Result<(), MigrationValidationError> {
    let admin = postgres_pool::admin(database).await?;
    if let Err(error) = postgres_create::resources(&admin, id).await {
        admin.close().await;
        return Err(error);
    }
    let validation = async {
        let target = postgres_pool::scoped(database, id).await?;
        let result = execute_postgres::run(&target, migrations).await;
        target.close().await;
        result
    }
    .await;
    let remove_database = postgres_drop::database(&admin, id).await;
    let remove_role = postgres_drop::role(&admin, id).await;
    let resource_cleanup = cleanup::combine(remove_database, remove_role);
    admin.close().await;
    cleanup::after_validation(validation, resource_cleanup)
}
