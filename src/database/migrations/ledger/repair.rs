//! Explicit repair of legacy migration ledgers.

use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;

const COUNT_ROWS: &str = "SELECT COUNT(*) FROM _rustyroad_migrations";
const DELETE_STALE_ROWS: &str = "DELETE FROM _rustyroad_migrations WHERE id IN (
    SELECT id FROM (
        SELECT id, ROW_NUMBER() OVER (
            PARTITION BY name ORDER BY applied_at DESC, id DESC
        ) AS row_rank
        FROM _rustyroad_migrations
    ) ranked WHERE row_rank > 1
)";
const UNIQUE_INDEX: &str = "_rustyroad_migrations_name_direction_key";

/// Summarizes an idempotent migration-ledger repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairReport {
    /// Rows present before repair.
    pub rows_before: i64,
    /// Rows retained after repair.
    pub rows_after: i64,
    /// Superseded duplicate rows deleted by repair.
    pub rows_removed: u64,
}

/// Retains the newest row per migration identity and enforces ledger uniqueness.
///
/// The operation is transactional and idempotent. A failed delete or index build
/// rolls the entire repair back.
pub async fn repair(connection: &DatabaseConnection) -> Result<RepairReport, CustomMigrationError> {
    match connection {
        DatabaseConnection::Pg(pool) => {
            let mut transaction = pool.begin().await?;
            let before = sqlx::query_scalar::<_, i64>(COUNT_ROWS)
                .fetch_one(&mut *transaction)
                .await?;
            let removed = sqlx::query(DELETE_STALE_ROWS)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
            let index = format!(
                "CREATE UNIQUE INDEX IF NOT EXISTS {UNIQUE_INDEX} \
                 ON _rustyroad_migrations (name, direction)"
            );
            sqlx::query(&index).execute(&mut *transaction).await?;
            let after = sqlx::query_scalar::<_, i64>(COUNT_ROWS)
                .fetch_one(&mut *transaction)
                .await?;
            transaction.commit().await?;
            Ok(RepairReport {
                rows_before: before,
                rows_after: after,
                rows_removed: removed,
            })
        }
        DatabaseConnection::Sqlite(pool) => {
            let mut transaction = pool.begin().await?;
            let before = sqlx::query_scalar::<_, i64>(COUNT_ROWS)
                .fetch_one(&mut *transaction)
                .await?;
            let removed = sqlx::query(DELETE_STALE_ROWS)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
            let index = format!(
                "CREATE UNIQUE INDEX IF NOT EXISTS {UNIQUE_INDEX} \
                 ON _rustyroad_migrations (name, direction)"
            );
            sqlx::query(&index).execute(&mut *transaction).await?;
            let after = sqlx::query_scalar::<_, i64>(COUNT_ROWS)
                .fetch_one(&mut *transaction)
                .await?;
            transaction.commit().await?;
            Ok(RepairReport {
                rows_before: before,
                rows_after: after,
                rows_removed: removed,
            })
        }
        DatabaseConnection::MySql(pool) => {
            let mut transaction = pool.begin().await?;
            let before = sqlx::query_scalar::<_, i64>(COUNT_ROWS)
                .fetch_one(&mut *transaction)
                .await?;
            let removed = sqlx::query(DELETE_STALE_ROWS)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
            let index_count = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM information_schema.statistics \
                 WHERE table_schema = DATABASE() \
                   AND table_name = '_rustyroad_migrations' \
                   AND index_name = '_rustyroad_migrations_name_direction_key'",
            )
            .fetch_one(&mut *transaction)
            .await?;
            if index_count == 0 {
                let index = format!(
                    "CREATE UNIQUE INDEX {UNIQUE_INDEX} \
                     ON _rustyroad_migrations (name, direction)"
                );
                sqlx::query(&index).execute(&mut *transaction).await?;
            }
            let after = sqlx::query_scalar::<_, i64>(COUNT_ROWS)
                .fetch_one(&mut *transaction)
                .await?;
            transaction.commit().await?;
            Ok(RepairReport {
                rows_before: before,
                rows_after: after,
                rows_removed: removed,
            })
        }
    }
}
