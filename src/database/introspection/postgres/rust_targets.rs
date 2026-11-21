//! Verified writable primary-key conflict targets for Rust repositories.
use super::{fetch, group, read};
use crate::database::introspection::{assemble::attach, Schema};
use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;

/// Only immediate, valid primary indexes can arbitrate an atomic conflict write.
const PRIMARY_TARGETS: &str = "\
    SELECT con.conname AS name, c.relname AS table_name, a.attname AS column_name, \
           k.ord AS position \
      FROM pg_constraint con \
      JOIN pg_class c ON c.oid = con.conrelid \
      JOIN pg_namespace n ON n.oid = c.relnamespace \
      JOIN pg_index idx ON idx.indexrelid = con.conindid \
      JOIN LATERAL unnest(con.conkey) WITH ORDINALITY AS k(attnum, ord) ON TRUE \
      JOIN pg_attribute a ON a.attrelid = con.conrelid AND a.attnum = k.attnum \
     WHERE n.nspname = $1 AND con.contype = 'p' AND NOT con.condeferrable \
       AND c.relkind IN ('r', 'p') AND idx.indisvalid AND idx.indisready \
       AND idx.indimmediate AND idx.indpred IS NULL AND idx.indexprs IS NULL \
     ORDER BY c.relname, con.conname, position";

/// Reads Rust generation metadata with verified, caller-supplied primary targets.
/// Auto-generated identities are excluded because New inputs cannot supply them.
/// Other languages retain the ordinary schema and do not gain duplicate keys.
/// Catalog errors propagate; no schema or data is mutated.
///
/// # Examples
/// ```no_run
/// # async fn inspect(connection: &rustyroad::database::DatabaseConnection) {
/// let schema = rustyroad::database::introspection::read_for_rust(connection, "public").await;
/// assert!(schema.is_ok());
/// # }
/// ```
pub async fn read_for_rust(
    connection: &DatabaseConnection,
    schema_name: &str,
) -> Result<Schema, CustomMigrationError> {
    let mut schema = read(connection, schema_name).await?;
    let DatabaseConnection::Pg(pool) = connection else {
        return Ok(schema);
    };
    let keys = group::constraints(fetch(pool.as_ref(), PRIMARY_TARGETS, schema_name).await?);
    let writable = keys
        .into_iter()
        .filter(|((name, _), columns)| {
            schema.table(name).is_some_and(|table| {
                !table.view
                    && table.primary_key.as_slice() == columns.as_slice()
                    && columns.iter().all(|name| {
                        table
                            .column(name)
                            .is_some_and(|column| !column.auto_increment && !column.nullable)
                    })
            })
        })
        .collect();
    attach::uniques(&mut schema, writable);
    Ok(schema)
}
