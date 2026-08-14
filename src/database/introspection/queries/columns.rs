//! Column and primary key catalog queries.

/// Columns of every base table, view, and materialized view in a schema.
///
/// `format_type` is used rather than `data_type` so modifiers survive:
/// `character varying(255)` instead of a bare `character varying`.
pub const COLUMNS: &str = "\
    SELECT c.relname AS table_name, \
           a.attname AS column_name, \
           format_type(a.atttypid, a.atttypmod) AS sql_type, \
           NOT a.attnotnull AS nullable, \
           pg_get_expr(d.adbin, d.adrelid) AS default_value, \
           (a.attidentity <> '' OR pg_get_expr(d.adbin, d.adrelid) LIKE 'nextval%') AS auto_increment \
      FROM pg_attribute a \
      JOIN pg_class c ON c.oid = a.attrelid \
      JOIN pg_namespace n ON n.oid = c.relnamespace \
      LEFT JOIN pg_attrdef d ON d.adrelid = a.attrelid AND d.adnum = a.attnum \
     WHERE n.nspname = $1 AND c.relkind IN ('r', 'v', 'm') AND a.attnum > 0 AND NOT a.attisdropped \
     ORDER BY c.relname, a.attnum";

/// Names of the views and materialized views in a schema.
///
/// Kept as a separate query rather than a column on `COLUMNS`, so the column reader
/// stays a plain (table, column) pair and view-ness attaches once per relation.
pub const VIEWS: &str = "\
    SELECT c.relname AS table_name \
      FROM pg_class c \
      JOIN pg_namespace n ON n.oid = c.relnamespace \
     WHERE n.nspname = $1 AND c.relkind IN ('v', 'm') \
     ORDER BY c.relname";

/// Primary key columns per table, in key order.
pub const PRIMARY_KEYS: &str = "\
    SELECT c.relname AS table_name, a.attname AS column_name, \
           array_position(i.indkey, a.attnum) AS position \
      FROM pg_index i \
      JOIN pg_class c ON c.oid = i.indrelid \
      JOIN pg_namespace n ON n.oid = c.relnamespace \
      JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey) \
     WHERE n.nspname = $1 AND i.indisprimary \
     ORDER BY c.relname, position";
