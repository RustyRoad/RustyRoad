//! Help text for the versioned lifecycle commands.

pub(super) const START: &str =
    "Applies a migration's up.sql and publishes a new schema version.\n\n\
     The previous version stays in place, so clients on either version keep\n\
     working. Point a client at a version by setting its search path:\n\n  \
     SET search_path TO public_<version>;\n\n\
     The migration stays in progress until you run 'migration complete',\n\
     so it can be undone instantly with 'migration rollback-version'.\n\n\
     Only one migration may be in progress at a time.\n\n\
     POSTGRES ONLY:\n\
      Versioned schemas use Postgres schemas and views. On MySQL and\n\
      SQLite the SQL is applied without version publishing.\n\n\
     EXAMPLE:\n\
      rustyroad migration start 20251114211514-add_columns\n";

pub(super) const COMPLETE: &str =
    "Finalizes the migration started with 'migration start' and drops the\n\
     previous version's schema, so only the new version is served.\n\n\
     After completing, the migration can no longer be rolled back instantly;\n\
     use a down migration instead.\n\n\
     EXAMPLE:\n  rustyroad migration complete\n";

pub(super) const ROLLBACK: &str =
    "Rolls back a migration started with 'migration start' but not yet\n\
     completed. Drops its versioned schema and applies its down.sql.\n\n\
     This is instant because the previous version was never removed.\n\n\
     EXAMPLE:\n  rustyroad migration rollback-version\n";

pub(super) const STATUS: &str =
    "Reports the version currently being served, any migration in progress,\n\
     and the most recent baseline.\n\n\
     EXAMPLE:\n  rustyroad migration version-status\n";
