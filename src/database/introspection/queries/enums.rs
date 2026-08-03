//! Enum type catalog query.

/// User-defined enum types and their values, in declaration order.
///
/// Values are ordered by `enumsortorder` rather than name, because Postgres
/// comparison follows declaration order and a client should see the same order.
pub const ENUMS: &str = "\
    SELECT t.typname AS name, e.enumlabel AS value \
      FROM pg_type t \
      JOIN pg_enum e ON e.enumtypid = t.oid \
      JOIN pg_namespace n ON n.oid = t.typnamespace \
     WHERE n.nspname = $1 \
     ORDER BY t.typname, e.enumsortorder";
