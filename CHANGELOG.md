# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.7.2] - 2026-08-14

### Added
- `rustyroad pull` can now filter introspected schemas and emit companion Rust and TetherScript model folders from the same TypeScript pull flow.
- Rust model generation now uses a shared layout writer, naming resolver, ownership tracking, and split Actix CRUD renderers for keyed and unkeyed routes.

### Changed
- Pull reporting and generator file ownership are shared across TypeScript, Rust, and TetherScript outputs so preserved files and missing parent-module declarations are reported consistently.

## [1.7.1] - 2026-08-10

### Fixed
- Rust generation now disambiguates PostgreSQL enum labels that normalize to
  the same Rust variant, such as `recurring_service` and `recurring-service`.
- Generated `inet` and `cidr` fields now use SQLx's `IpNetwork` re-export,
  preventing type-trait failures when an application has a different direct
  `ipnetwork` dependency version. Rust pull guidance no longer adds that
  redundant direct dependency.

## [1.7.0] - 2026-08-09

### Added
- `rustyroad pull --language rust` now generates the Rust counterpart to the
  TypeScript Drizzle/oRPC/Fastify stack: SQLx models and repositories, Actix
  CRUD procedures, and the same OpenAPI/Hey API client contract.
- Rust composition files (`api.rs` and `mod.rs`) are written once and preserved
  across pulls; generated procedures use a stable registration entry point so
  newly introspected tables become reachable without rewriting custom code.
- Rust create and patch inputs follow database defaults and nullability, with
  nullable patches distinguishing an omitted field from an explicit JSON null.

### Changed
- Migration list output now reports `RECORDED IN LEDGER` and `UNVERIFIED`
  instead of treating ledger presence as proof that SQL ran or its effects exist.
- `_rustyroad_migrations` now records provenance (`executed`, `baselined`, or
  the `legacy` default), a SHA-256 migration checksum, and optional
  `verified_at`; existing ledgers are upgraded in place.
- `migration baseline` records explicit `baselined` provenance and no longer
  describes those rows as applied. `version-status` now explains that lifecycle
  history and migration-ledger bookkeeping are separate evidence sources.

### Fixed
- SQLite data-type category discovery now returns its supported Boolean,
  numeric, date/time, text, and fallback types.
- Generated grouped imports are deterministic, and filesystem-dependent tests
  no longer race through the process-wide current directory or require live
  MySQL/PostgreSQL services during the default test suite.

## [1.6.4] - 2026-08-08

### Changed
- Published the ledger status and transactional repair work prepared under the tagged but unpublished `v1.6.3` revision.
- Normalized the repository with stable Rustfmt so the GitHub release gate and local formatting checks agree.

## [1.6.3] - 2026-08-08

### Fixed
- `migration list` now uses the execution gate's exact full-identity lookup and legacy bare-name fallback, so its status cannot disagree with `migration run`.
- Ledger status selection is deterministic when legacy tables contain duplicate timestamps.

### Added
- `migration repair-ledger` transactionally retains the newest row per migration identity, removes superseded duplicates, and restores the unique index that prevents new duplicate growth.
- The repair requires interactive confirmation or `--yes`, is idempotent, and never executes application migration SQL.

## [1.6.2] - 2026-08-08

### Fixed
- `migration list` now matches full timestamped ledger identities to their migration directories while displaying the human-readable suffix.
- Legacy bare-name rows remain compatible, and distinct migrations sharing a suffix are not collapsed.

## [1.6.1] - 2026-08-03

### Fixed
- `query` no longer renders most Postgres types as `<unprintable>`. The decoder attempted only `String`, `i32`, `i64`, and `bool`, so `smallint`, `jsonb`, `numeric`, `uuid`, timestamps, dates, times, enums, and arrays all fell through, forcing callers to cast in SQL just to read their own data. A 16-column table went from 3 readable columns to 14.
- Integer widths are now tried narrowest first, so a `smallint` is not silently widened.
- `numeric` decodes through `BigDecimal` and renders as a string, preserving the precision the type exists for; routing it through `f64` would lose it.
- `json` and `jsonb` render as real JSON rather than a quoted string.
- Both the table renderer and the JSON output now share one decoder, so the two formats cannot disagree about a type. The decoding logic existed in five near-identical copies.
- A value sqlx cannot decode now reports its SQL type and size — `<INTERVAL, 16 bytes>` — instead of a bare `<unprintable>`, so the caller knows what it is and can cast to text.

### Changed
- Enabled sqlx's `bigdecimal` feature, required to decode `numeric` exactly.

### Notes
- Verified live against PostgreSQL 16 across 16 column types. Evidence in `.codetether-agent/evidence/live-values.md`.
- Only the Postgres paths were rewritten. The MySQL and SQLite renderers still carry the original four-type block and will show `<unprintable>` for the same types.

## [1.6.0] - 2026-08-03

### Added
- `rustyroad pull` now respects hand-written code. Files derived from the database are still rewritten on every run, but `db/api.ts` and `db/openapi/openapi-ts.config.ts` are written once and then left alone, so hand-written procedures and config edits survive regeneration. `pull` reports each file as `wrote` or `kept`, and `--force` overwrites the preserved ones.
- Split the router: `router.ts` now exports a `generated` namespace and is regenerated every run, while `api.ts` composes it with hand-written routers and is the file served. This is what makes composition survive a re-pull.
- `pull` warns when a table's procedures are generated but not referenced by the preserved `api.ts`, printing the line to add. Automatic wiring is deferred to a later release; silently generating unreachable procedures would be worse than saying so.
- Added `pgEnum` support. A Postgres enum previously degraded to `text`, losing its values. Enum types are now introspected and emitted as `pgEnum` declarations ahead of the tables that reference them, so the allowed values reach the Drizzle types, the Zod validation, and the OpenAPI document. An invalid value is rejected as a validation error rather than reaching Postgres and raising `22P02`.
- A nullable enum admits `null` in both its JSON Schema `type` and `enum` list, because a validator checks membership before nullability.
- `pull` now reports the number of enum types found alongside tables, columns, and foreign keys.

### Notes
- Verified live against PostgreSQL 16. Ownership: a second `pull` after adding a hand-written `billing` router reported `kept ./db/api.ts`, the composition survived, and 8/8 runtime checks passed with generated and hand-written procedures served together over RPC, REST, and OpenAPI, including a preserved custom `FORBIDDEN` error code. Enums: 5/5 runtime checks passed, with an invalid value rejected as `BAD_REQUEST` on both transports. `tsc --noEmit` under strict passed in both cases.
- Because `api.ts` is preserved, `os.router()` is no longer used in it: that helper widens the context type and would reject procedures built on a typed context. A plain nested record is used instead, which oRPC accepts.

## [1.5.0] - 2026-08-03

Unifies the generated API surface on oRPC, so migrations, `pull`, and client
generation compose into one workflow rather than three.

### Added
- `rustyroad pull` now generates `db/router.ts`: oRPC procedures over the typed repositories, each declaring `.route({ method, path })`, `.input()`, and `.output()` with the Zod schemas derived from the Drizzle tables. One definition serves an RPC call, a REST request, and the OpenAPI document oRPC generates from it.
- Added `db/server.ts`, a Fastify adapter mounting both the RPC and OpenAPI handlers, so requests under `<prefix>/rpc` are handled as RPC and everything else is routed by the declared paths.
- Added `db/openapi.ts`, a script writing the OpenAPI document from the router. Because oRPC derives it from the procedures' own schemas, the document cannot describe an endpoint the server does not serve.
- A missing row raises an `ORPCError` with code `NOT_FOUND`, which stays typed for RPC callers and maps to a 404 over REST.
- Rewrote `example/db-generation/README.md` around the migration -> pull -> client loop, documenting how the three features compose and what each guarantees.

### Changed
- Replaced the hand-written Fastify route plugins with the oRPC router. A column was previously described in the Drizzle table, the Zod schemas, and the route definitions; it is now described once.

### Notes
- Verified live against PostgreSQL 16, 18/18 runtime checks: the typed RPC client, the same procedures over REST with `z.coerce` on path parameters, and OpenAPI generation, all from one router. `@hey-api/openapi-ts` then generated a client from that document, converting an `operationId` of `users.list` into `usersList`.
- The documented loop was itself verified: a nullable column added by `migration generate` and applied by `migration all` became reachable over RPC and REST after a single `pull`, with no hand-editing. Renaming that column in Postgres and re-running `pull` made `tsc` fail at every stale call site rather than failing at runtime.
- `server.ts` disables Fastify's body parsers on its route, because the oRPC handlers need the raw request.

## [1.4.0] - 2026-08-03

### Added
- `rustyroad pull` now also writes `db/openapi/openapi.json`, an OpenAPI 3.1 document describing the generated Fastify routes, and `db/openapi/openapi-ts.config.ts`, a `@hey-api/openapi-ts` config preset to the flat SDK style (`@hey-api/sdk` with `asClass: false`).
- Generating the document rather than scraping it from a running server removes the bootstrap problem: a browser client can be generated in CI without booting the API. Once deployed, Hey API can be pointed at the live document instead, and call sites do not change.
- The document declares the same `operationId`s the routes carry, so generated SDK function names (`listUsers`, `createUsers`, `getUsers`, `updateUsers`, `deleteUsers`) are stable across re-runs.
- Component schemas carry a select, insert, and patch variant per table plus a shared error shape, with generated keys absent from input schemas, defaulted columns optional, nullable columns as type unions, and `format` declared for uuid, date, time, and timestamp columns.

### Notes
- `@hey-api/openapi-ts` 0.87.x requires TypeScript 5; under TypeScript 7 it fails with `Cannot read properties of undefined (reading 'LineFeed')`. The generated config states this, and `example/db-generation/README.md` documents the pin.
- Verified live against PostgreSQL 16: `pull` wrote the document, `@hey-api/openapi-ts` 0.87.5 generated a client from it (exit 0), `tsc --noEmit` passed under strict, and the generated client drove the generated Fastify server through 12/12 checks including a typed 404 for a missing row.

## [1.3.0] - 2026-08-03

### Added
- Added `rustyroad pull`, which introspects a live Postgres database and writes a folder of TypeScript, in the spirit of `drizzle-kit pull` but carrying the API layer as well.
- Added rich Postgres introspection reading tables, columns with types and defaults, primary keys, foreign keys with referential actions, unique constraints, and indexes. Types come from `format_type`, so modifiers such as `varchar(255)` and `numeric(12,2)` survive.
- Added `schema.ts` generation: Drizzle table definitions with inline single-column primary keys, table-level composite keys, foreign keys, uniques, and indexes. Tables are emitted in dependency order, because a `const` is not hoisted and a foreign key referencing a later binding would fail at module evaluation.
- Added `relations.ts` generation deriving `one`/`many` relations from foreign keys.
- Added `zod.ts` generation deriving Zod schemas from the Drizzle tables via `drizzle-zod`, so validation cannot drift from the database.
- Added `client.ts` generation with a typed CRUD repository per table.
- Added `routes.ts` generation: one Fastify plugin per table using `fastify-type-provider-zod`, so the same Zod schemas validate requests, serialize responses, and produce the OpenAPI document that client generators such as Hey API consume. Routes declare stable `operationId`s.
- Added `example/db-generation/` documenting the wiring and carrying a script that boots the generated routes against a real database.

### Fixed
- `migration generate` no longer fails with "Migration already exists" when two migrations are generated within the same second. The folder prefix has second resolution, so a retry collided; a numeric suffix is now appended. The check also reported every folder-creation failure as a collision, including a missing parent directory, and now creates the parent chain and reports the real error.

### Notes
- Introspection and generation are Postgres-only.
- Tables with a composite primary key get a schema, relations, and Zod schemas, but no routes, since there is no single `/:id` form.
- `numeric` is carried as a string, matching Drizzle, so decimal precision is not lost through a float.
- `jsonb` columns are annotated `.$type<Record<string, unknown>>()`, because Drizzle would otherwise infer `unknown` while `drizzle-zod` infers a recursive `Json` union, and the two are not assignable.

## [1.2.1] - 2026-08-03

Five bugs found by running the versioned lifecycle against a real PostgreSQL 16
server. Each one passed the SQL-shape unit tests and still failed live.

### Fixed
- Backfill cursor keys are now cast to text in SQL. Reading an `int4` primary key as text client-side failed with "invalid input syntax for type integer", so every backfill over an integer key aborted.
- Trigger functions now normalize `current_setting('search_path')` before comparing it. PostgreSQL returns the value quoted when the schema name requires it, so the literal comparison never matched and the reverse trigger silently never fired.
- A `down` expression is now rewritten from the new logical column name to the physical shadow column. `NEW` exposes physical columns only, so an expression referencing the renamed column failed with `record "new" has no field ...`.
- A `nullable: false` declaration on `alter_column` now survives promotion. The constraint was dropped during shadow promotion and targeted the pre-rename name, so the completed column came back nullable.
- Version views no longer expose RustyRoad's own history table or the internal backfill marker column to clients.

### Notes
- Verified live: 2500-row table, `zone_id int4` renamed to `trash_zone_id uuid` with `nullable: false`. Backfill converged; a write through the old schema was readable through the new and vice versa; after `complete` the column was `uuid NOT NULL` with all 2502 rows populated and no triggers, functions, shadow, or marker left behind; `rollback-version` restored the original table exactly.

## [1.2.0] - 2026-07-22

### Added
- Added declarative migrations. A migration may be authored as `migration.json` declaring `operations` instead of raw `up.sql`, which lets the runner apply a breaking column change without breaking clients still running the old code.
- Added the `alter_column` operation: retype, rename, or change nullability by creating a shadow physical column, installing triggers so a write through either schema version populates both, backfilling existing rows, then promoting the shadow at completion.
- Added `add_column`, `drop_column`, and `sql` operations. `add_column` backfills existing rows from its `up` expression and defers `NOT NULL` until the backfill has run; `drop_column` keeps the column populated for old readers via `down` until completion.
- Added backfill triggers that compare `search_path` against the version being served, so a write arriving through the old schema is rewritten into the new column and vice versa, without the application knowing a migration is in progress.
- Added a batched backfill using keyset pagination over the primary key with `FOR NO KEY UPDATE`, so cost stays flat as it progresses and concurrent writers to other rows are not blocked.
- Added completion plan persistence, so `complete` finishes the physical changes `start` deliberately deferred even though it runs in a separate invocation.
- Added `example/migrations/declarative/` documenting the operations and the start/complete rollout.

### Changed
- `migration start` now accepts either a declarative `migration.json` or raw `up.sql`; raw SQL is wrapped as a single operation so both authoring styles share one execution path.
- `migration rollback-version` now also drops the shadow columns, triggers, and backfill marker that `start` created.

### Notes
- Backfilled tables require a primary key, since the backfill paginates over it rather than locking the whole table.
- Declarative operations are Postgres-only, as they depend on versioned schemas.

## [1.1.0] - 2026-07-22

### Added
- Added versioned schema support, modelled on pgroll. Each schema version is published as a Postgres schema `<schema>_<version>` containing one view per table, so old and new schemas are served simultaneously from the same physical tables. Clients select a version with `SET search_path`.
- Added a two-phase migration lifecycle. `rustyroad migration start <version>` applies a migration's SQL and publishes a new version while leaving the previous one in place; `rustyroad migration complete` finalizes it and retires the previous version.
- Added `rustyroad migration rollback-version` to undo a started-but-incomplete migration instantly, because the previous version was never removed.
- Added `rustyroad migration version-status` reporting the version being served, any migration in progress, and the most recent baseline.
- Added `rustyroad migration baseline` to record every migration on disk as applied without executing it, for databases whose schema is already current but whose ledger does not say so.
- Added a migration history table with a linear parent chain, a `done` flag separating started from completed migrations, and a `migration_type` marking baselines. Unique indexes enforce at most one migration in progress, a single root, and no forks.
- Added Postgres schema introspection that maps in-flight physical columns back to their logical names and hides columns pending deletion, so a version's views expose a stable client-facing shape.

### Changed
- Views restate column defaults explicitly, since a view does not inherit them from its underlying table, and use `security_invoker` on Postgres 15 and later so row-level security on the underlying table is respected.
- A failed `start` withdraws its history row, so a failed migration does not block the next one.
- Every schema, view, table, and column identifier in generated DDL is quoted.

### Notes
- Versioned schemas are Postgres-only. On MySQL and SQLite, migrations are applied without version publishing.

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