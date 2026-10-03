//! Representable indexes only; unsupported predicates/expressions must not become keys.

/// Reads plain indexes; e.g. `fetch(pool, INDEXES, "public")` excludes partial keys.
/// INCLUDE columns are payload, not conflict-target columns. Deferred/invalid indexes
/// cannot arbitrate ON CONFLICT. The public Index model deliberately has no such flags.
pub const INDEXES: &str = "\
    SELECT c.relname AS table_name, i.relname AS name, a.attname AS column_name, \
           ix.indisunique AS unique, k.ord AS position \
      FROM pg_index ix \
      JOIN pg_class c ON c.oid = ix.indrelid \
      JOIN pg_class i ON i.oid = ix.indexrelid \
      JOIN pg_namespace n ON n.oid = c.relnamespace \
      JOIN LATERAL unnest(ix.indkey) WITH ORDINALITY AS k(attnum, ord) ON TRUE \
      JOIN pg_attribute a ON a.attrelid = ix.indrelid AND a.attnum = k.attnum \
     WHERE n.nspname = $1 AND NOT ix.indisprimary \
       AND c.relkind IN ('r', 'p') AND k.ord <= ix.indnkeyatts \
       AND ix.indpred IS NULL AND ix.indexprs IS NULL \
       AND ix.indisvalid AND ix.indisready AND ix.indimmediate \
       AND NOT EXISTS (SELECT 1 FROM pg_constraint con \
                        WHERE con.conindid = ix.indexrelid AND con.contype = 'u') \
     ORDER BY c.relname, i.relname, position";
