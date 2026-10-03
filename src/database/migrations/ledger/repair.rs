//! Explicit repair of legacy migration ledgers.

use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;
use sqlx::AssertSqlSafe;

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
/// MySQL rejects `IF NOT EXISTS` on `CREATE UNIQUE INDEX`, so its existence is
/// probed explicitly instead.
const MYSQL_INDEX_EXISTS: &str = "SELECT COUNT(*) FROM information_schema.statistics \
     WHERE table_schema = DATABASE() \
       AND table_name = '_rustyroad_migrations' \
       AND index_name = '_rustyroad_migrations_name_direction_key'";

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

/// Deduplicates the ledger inside one transaction, then runs `$ensure_index`
/// against that same transaction before committing.
///
/// Each backend owns a distinct transaction type, so this is a macro rather than
/// a generic function; `$ensure_index` is a block that may use `transaction`.
macro_rules! repair_in_transaction {
    ($pool:expr, |$transaction:ident| $ensure_index:block) => {{
        let mut $transaction = $pool.begin().await?;
        let rows_before = sqlx::query_scalar::<_, i64>(COUNT_ROWS)
            .fetch_one(&mut *$transaction)
            .await?;
        let rows_removed = sqlx::query(DELETE_STALE_ROWS)
            .execute(&mut *$transaction)
            .await?
            .rows_affected();

        $ensure_index

        let rows_after = sqlx::query_scalar::<_, i64>(COUNT_ROWS)
            .fetch_one(&mut *$transaction)
            .await?;
        $transaction.commit().await?;
        Ok(RepairReport {
            rows_before,
            rows_after,
            rows_removed,
        })
    }};
}

/// Builds the unique-index statement, optionally guarded by `IF NOT EXISTS`.
fn create_index(if_not_exists: bool) -> String {
    let guard = if if_not_exists { "IF NOT EXISTS " } else { "" };
    format!(
        "CREATE UNIQUE INDEX {guard}{UNIQUE_INDEX} \
         ON _rustyroad_migrations (name, direction)"
    )
}

/// Retains the newest row per migration identity and enforces ledger uniqueness.
///
/// The operation is transactional and idempotent. A failed delete or index build
/// rolls the entire repair back.
pub async fn repair(connection: &DatabaseConnection) -> Result<RepairReport, CustomMigrationError> {
    match connection {
        DatabaseConnection::Pg(pool) => repair_in_transaction!(pool, |transaction| {
            sqlx::query(AssertSqlSafe(create_index(true)))
                .execute(&mut *transaction)
                .await?;
        }),
        DatabaseConnection::Sqlite(pool) => repair_in_transaction!(pool, |transaction| {
            sqlx::query(AssertSqlSafe(create_index(true)))
                .execute(&mut *transaction)
                .await?;
        }),
        DatabaseConnection::MySql(pool) => repair_in_transaction!(pool, |transaction| {
            let existing = sqlx::query_scalar::<_, i64>(MYSQL_INDEX_EXISTS)
                .fetch_one(&mut *transaction)
                .await?;
            if existing == 0 {
                sqlx::query(AssertSqlSafe(create_index(false)))
                    .execute(&mut *transaction)
                    .await?;
            }
        }),
    }
}
