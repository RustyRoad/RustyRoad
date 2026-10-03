//! Constraint and index catalog queries.

/// Foreign keys with their local and referenced columns, in key order.
pub const FOREIGN_KEYS: &str = "\
    SELECT con.conname AS name, \
           c.relname AS table_name, \
           f.relname AS foreign_table, \
           a.attname AS column_name, \
           fa.attname AS foreign_column, \
           con.confupdtype AS on_update, \
           con.confdeltype AS on_delete, \
           k.ord AS position \
      FROM pg_constraint con \
      JOIN pg_class c ON c.oid = con.conrelid \
      JOIN pg_class f ON f.oid = con.confrelid \
      JOIN pg_namespace n ON n.oid = c.relnamespace \
      JOIN LATERAL unnest(con.conkey, con.confkey) \
             WITH ORDINALITY AS k(local, remote, ord) ON TRUE \
      JOIN pg_attribute a ON a.attrelid = con.conrelid AND a.attnum = k.local \
      JOIN pg_attribute fa ON fa.attrelid = con.confrelid AND fa.attnum = k.remote \
     WHERE n.nspname = $1 AND con.contype = 'f' \
     ORDER BY con.conname, position";

/// Immediate unique constraints; e.g. `fetch(pool, UNIQUES, "public")` omits deferred keys.
/// Deferred constraints cannot arbitrate ON CONFLICT and have no model flag.
pub const UNIQUES: &str = "\
    SELECT con.conname AS name, c.relname AS table_name, a.attname AS column_name, \
           k.ord AS position \
      FROM pg_constraint con \
      JOIN pg_class c ON c.oid = con.conrelid \
      JOIN pg_namespace n ON n.oid = c.relnamespace \
      JOIN LATERAL unnest(con.conkey) WITH ORDINALITY AS k(attnum, ord) ON TRUE \
      JOIN pg_attribute a ON a.attrelid = con.conrelid AND a.attnum = k.attnum \
     WHERE n.nspname = $1 AND con.contype = 'u' AND NOT con.condeferrable \
      ORDER BY con.conname, position";