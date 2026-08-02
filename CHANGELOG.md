# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.33] - 2026-07-22

### Fixed
- `rustyroad query` now exits with status 1 when a query fails. It previously printed the error and exited 0, so callers and CI could not distinguish a failed query from a successful one.
- Supported multi-statement SQL in `rustyroad query`. Scripts were sent through the prepared-statement protocol, which accepts a single command, so Postgres rejected them with "cannot insert multiple commands into a prepared statement". Scripts now use the unprepared path, in both text and JSON output modes.

### Changed
- Statement counting ignores semicolons inside string literals, quoted identifiers, and `--` or `/* */` comments, so a single statement containing a semicolon is not treated as a script.

## [1.0.32] - 2026-07-22

### Fixed
- Restricted breaking-change scanning to migrations that are not already applied. The scanner ran before the ledger check, so `migration all` and `migration run` were blocked by warnings about migrations that would have been skipped as no-ops, forcing an unnecessary `--allow-breaking`.
- Matched named migrations by both bare name and full `<timestamp>-<name>` directory name when scanning.

### Changed
- Freshly generated migrations are still scanned directly from disk, since a new migration cannot already be applied.
- When the ledger cannot be reached, every migration is scanned as before, so an unreachable database makes the scanner more cautious rather than less.

## [1.0.31] - 2026-07-22

### Fixed
- Made `rustyroad migration all` idempotent: migrations already recorded as applied are now skipped instead of being re-executed on every run.
- Recorded migrations by their full `<timestamp>-<name>` directory name so ledger ordering is reconstructable and migrations sharing a bare name cannot collide.
- Read the ledger by row presence rather than insertion order, so a rolled-back migration is correctly reported as unapplied and can be applied again.
- Resolved exact `<timestamp>-<name>` migration directories directly, so `migration all` no longer prompts interactively (and panics without a TTY) when two migrations share a bare name.
- Replaced string-interpolated ledger inserts with bound parameters.

### Changed
- Added `UNIQUE (name, direction)` to `_rustyroad_migrations` and switched recording to an upsert, so the table is an authoritative ledger rather than an append-only audit trail. Pre-existing tables are upgraded on a best-effort basis; duplicate rows are reported and left for the operator to remove.
- Existing ledgers that stored bare migration names are still honoured, so upgrading does not replay previously applied migrations.
- Extracted ledger handling into `src/database/migrations/ledger/`, removing a duplicated `_rustyroad_migrations` table definition.

## [1.0.30] - 2026-07-22

### Added
- Added preflight detection for column type changes, explicit casts, and dropped foreign-key constraints.
- Added breaking-change warnings immediately after migration generation and before upward migration execution.
- Added interactive confirmation and a required `--allow-breaking` acknowledgement for non-interactive apply commands.
- Added fail-closed `migration run` and `migration all` coverage before any database connection.

## [1.0.29] - 2026-07-17

### Fixed
- Allowed `rustyroad migration validate` to use dev, test, staging, prod, or any other configured environment.
- Dev validation reads `rustyroad.toml`; named environments read `rustyroad.<environment>.toml`.
- Validation still never connects to the configured `database_name`; all migration SQL runs against disposable resources.
- Added real CLI coverage proving default/dev and prod validation leave configured SQLite database files untouched.

## [1.0.28] - 2026-07-16

### Added
- Added `rustyroad migration validate` to execute the complete `up.sql` chain against disposable database resources.
- Added strict `ENVIRONMENT=test` / `ENV=test` enforcement and generated, scoped PostgreSQL/MySQL validation credentials.
- Added fail-closed migration discovery, cleanup reporting, and SQLite isolation tests.

### Changed
- Centralized environment and config-file resolution, with `ENVIRONMENT` taking precedence over `ENV`.
- Extracted migration CLI construction and dispatch into focused modules with bounded file sizes.
- Validation never connects to or modifies the configured shared test database.

## [1.0.24] - 2025-12-29

### Added
- **LLM/AI Agent Enhancements**: Comprehensive CLI improvements for AI agent usability
  - Added `long_about()` documentation to 10 commands (query, db schema, migration subcommands, generate controller/model) with ENVIRONMENT variable guidance, prerequisites, examples, and warnings
  - Enhanced error messages to include config file path, current working directory, database connection details, and actionable how-to-fix suggestions
  - Added `--format` flag supporting JSON output for query, db schema, migration list, and config commands
  - Added config file echo to all database commands for transparency about which rustyroad.toml variant is being used
  - All JSON outputs include `config_file` field for agent awareness

## [1.0.23] - 2025-12-14

### Added
- **Migration CLI guidance**: Expanded `rustyroad migration --help` to explain where migrations live and the expected folder/file structure.
- **Beginner/LLM-friendly aliases**: `rustyroad migration generate` now supports `new`, `create`, and `make` aliases; `migration status` is an alias of `migration list`.

### Fixed
- `rustyroad migration all` now correctly runs all migrations (previously attempted to run directory names as migrations).
- Implemented `migration redo <name>` (down then up) and `migration reset` (rollback all) behaviors to match CLI help.

### Changed
- Improved migration/runtime error messages to be more actionable (missing `rustyroad.toml`, missing migrations directory, and common misplacement in `./migrations`).
- Updated README migration examples to match the current CLI.

## [1.0.22] - 2025-11-30

### Added
- **Migration Generator Constraint Support**: Added comprehensive constraint parsing for both `CREATE TABLE` and `ALTER TABLE` migrations
  - `nullable` / `null` - Explicitly mark columns as nullable (no warning)
  - `not_null` - Add NOT NULL constraint
  - `primary_key` - Add PRIMARY KEY constraint
  - `unique` - Add UNIQUE constraint
  - `default=<value>` - Set default value (auto-quotes strings, supports booleans and NULL)
  - `references=table(column)` - Add foreign key constraint with explicit table/column reference

### Fixed
- Fixed "Unsupported constraint 'nullable'" warning when using nullable columns in migrations
- `ALTER TABLE` migrations now properly parse and apply column constraints (previously ignored)

### Example Usage
```bash
# Create table with constraints
rustyroad migration generate users \
  id:serial:primary_key \
  email:string:not_null,unique \
  name:string:nullable \
  role:string:default=user \
  created_at:timestamp:not_null

# Add columns with constraints  
rustyroad migration generate add_quickbooks_fields_to_employees \
  quickbooks_employee_id:string:nullable \
  quickbooks_employee_name:string:nullable \
  quickbooks_mapped_at:timestamp:nullable

# Foreign key with explicit reference
rustyroad migration generate add_author_to_posts \
  author_id:integer:not_null,references=users(id)
```

## [1.0.21] - Previous Release

- Initial constraint support for CREATE TABLE migrations
- Basic migration generation with type mapping