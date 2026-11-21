//! Catalog evidence for the subset of primary keys we can restore faithfully.

/// Preserve key order; exclude index customization the renderer cannot reproduce.
pub(super) const PRIMARY_KEYS: &str = "
SELECT c.relname AS table_name, k.conname AS name,
       k.condeferrable AS deferrable, k.condeferred AS initially_deferred,
       ARRAY(SELECT a.attname::text FROM unnest(k.conkey) WITH ORDINALITY AS x(num, ord)
             JOIN pg_attribute a ON a.attrelid=k.conrelid AND a.attnum=x.num
             ORDER BY x.ord) AS columns,
       ARRAY(SELECT format('%I.%I.%I', fn.nspname, fc.relname, fk.conname)
             FROM pg_constraint fk JOIN pg_class fc ON fc.oid=fk.conrelid
             JOIN pg_namespace fn ON fn.oid=fc.relnamespace
             WHERE fk.contype='f' AND fk.confrelid=k.conrelid
             ORDER BY fn.nspname, fc.relname, fk.conname) AS referenced_by,
       (c.relkind='r' AND NOT c.relispartition AND k.conparentid=0
        AND i.indisvalid AND i.indisready AND i.indislive AND am.amname='btree'
        AND i.indnkeyatts=i.indnatts AND i.indexprs IS NULL AND i.indpred IS NULL
        AND ic.reltablespace=0 AND ic.reloptions IS NULL
        AND NOT i.indisclustered AND NOT i.indisreplident
        AND obj_description(k.oid, 'pg_constraint') IS NULL
        AND obj_description(ic.oid, 'pg_class') IS NULL
        AND NOT EXISTS (
            SELECT 1 FROM unnest(i.indclass::oid[], i.indcollation::oid[],
                                i.indoption::smallint[], i.indkey::smallint[])
                AS x(opclass, collation_oid, index_options, num)
            JOIN pg_opclass op ON op.oid=x.opclass
            JOIN pg_attribute a ON a.attrelid=c.oid AND a.attnum=x.num
            WHERE NOT op.opcdefault OR x.index_options<>0 OR x.collation_oid<>a.attcollation
               OR a.attoptions IS NOT NULL)) AS standard_index
FROM pg_constraint k JOIN pg_class c ON c.oid=k.conrelid
JOIN pg_namespace n ON n.oid=c.relnamespace
JOIN pg_index i ON i.indexrelid=k.conindid
JOIN pg_class ic ON ic.oid=i.indexrelid JOIN pg_am am ON am.oid=ic.relam
WHERE n.nspname=$1 AND k.contype='p'
ORDER BY c.relname";
