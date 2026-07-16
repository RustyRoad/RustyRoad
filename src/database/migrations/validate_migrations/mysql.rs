use super::cleanup;
use super::error::MigrationValidationError;
use super::execute_mysql;
use super::identity::Identity;
use super::model::MigrationFile;
use super::{mysql_create, mysql_drop, mysql_pool};
use crate::database::Database;

pub(super) async fn validate(
    database: &Database,
    id: &Identity,
    migrations: &[MigrationFile],
) -> Result<(), MigrationValidationError> {
    let admin = mysql_pool::admin(database).await?;
    if let Err(error) = mysql_create::resources(&admin, id).await {
        admin.close().await;
        return Err(error);
    }
    let validation = async {
        let target = mysql_pool::scoped(database, id).await?;
        let result = execute_mysql::run(&target, migrations).await;
        target.close().await;
        result
    }
    .await;
    let remove_database = mysql_drop::database(&admin, id).await;
    let remove_user = mysql_drop::user(&admin, id).await;
    let resource_cleanup = cleanup::combine(remove_database, remove_user);
    admin.close().await;
    cleanup::after_validation(validation, resource_cleanup)
}
