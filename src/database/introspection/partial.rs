//! Partial uniqueness is separate from unconditional repository keys.
use super::postgres::fetch;
use crate::database::{migrations::CustomMigrationError, DatabaseConnection};
use sqlx::Row;

/// A verified plain-column partial unique index and its server-rendered predicate.
/// # Examples
/// ```no_run
/// # async fn example(connection: &rustyroad::database::DatabaseConnection) {
/// let indexes = rustyroad::database::introspection::read_partial_unique_indexes(connection, "public").await;
/// assert!(indexes.is_ok());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PartialUniqueIndex {
    pub table: String,
    pub name: String,
    pub columns: Vec<String>,
    pub predicate: String,
}

const QUERY: &str = "SELECT t.relname AS table_name, c.relname AS name,
    array_agg(a.attname::text ORDER BY k.ord) AS columns,
    pg_get_expr(i.indpred, i.indrelid) AS predicate
    FROM pg_index i JOIN pg_class t ON t.oid = i.indrelid
    JOIN pg_namespace n ON n.oid = t.relnamespace
    JOIN pg_class c ON c.oid = i.indexrelid
    JOIN LATERAL unnest(i.indkey) WITH ORDINALITY AS k(attnum, ord) ON TRUE
    JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = k.attnum
    WHERE n.nspname = $1 AND t.relkind IN ('r','p') AND i.indisunique
      AND i.indisvalid AND i.indisready AND i.indimmediate
      AND i.indpred IS NOT NULL AND i.indexprs IS NULL
      AND k.ord <= i.indnkeyatts AND NOT a.attisdropped
    GROUP BY t.relname, c.relname, pg_get_expr(i.indpred, i.indrelid), i.indnkeyatts
    HAVING count(*) = i.indnkeyatts
    ORDER BY t.relname, c.relname";

/// Reads valid immediate partial keys without changing the ordinary schema model.
/// The predicate comes from PostgreSQL, never from a caller-provided SQL fragment.
/// # Examples
/// ```no_run
/// # async fn example(connection: &rustyroad::database::DatabaseConnection) {
/// let result = rustyroad::database::introspection::read_partial_unique_indexes(connection, "public").await;
/// assert!(result.is_ok());
/// # }
/// ```
pub async fn read_partial_unique_indexes(
    connection: &DatabaseConnection,
    schema: &str,
) -> Result<Vec<PartialUniqueIndex>, CustomMigrationError> {
    let DatabaseConnection::Pg(pool) = connection else {
        return Ok(Vec::new());
    };
    fetch(pool.as_ref(), QUERY, schema)
        .await?
        .iter()
        .map(|row| {
            Ok(PartialUniqueIndex {
                table: row.try_get("table_name")?,
                name: row.try_get("name")?,
                columns: row.try_get("columns")?,
                predicate: row.try_get("predicate")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()
        .map_err(CustomMigrationError::SqlxError)
}
