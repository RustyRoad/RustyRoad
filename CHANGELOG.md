# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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