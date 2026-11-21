<p align="center">
  <a href="https://github.com/RustyRoad/RustyRoad" rel="noopener">
    <img src="https://avatars.githubusercontent.com/u/138265565?s=400&u=eb116ae7b42e521b884d1288213df00032130f6a&v=4" alt="RustyRoad logo" width="200">
  </a>
</p>

<h1 align="center">RustyRoad</h1>

<p align="center">
  Rails-flavored scaffolding and migrations for Rust web apps (Actix + Tera + SQLx).
</p>

<div align="center">

[![Rust](https://img.shields.io/badge/rust-gray.svg?&logo=rust&logoColor=orange)](https://www.rust-lang.org/)
[![CI](https://img.shields.io/github/actions/workflow/status/RustyRoad/RustyRoad/ci.yml?branch=main)](https://github.com/RustyRoad/RustyRoad/actions)
[![Crates.io](https://img.shields.io/crates/v/rustyroad.svg)](https://crates.io/crates/rustyroad)
[![Docs.rs](https://img.shields.io/docsrs/rustyroad)](https://docs.rs/rustyroad)
[![Issues](https://img.shields.io/github/issues/RustyRoad/RustyRoad.svg)](https://github.com/RustyRoad/RustyRoad/issues)
[![PRs](https://img.shields.io/github/issues-pr/RustyRoad/RustyRoad.svg)](https://github.com/RustyRoad/RustyRoad/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

> RustyRoad is under active development. For day-to-day use, prefer the latest released version on crates.io.

---

<sup>In loving memory of Rusty (2014–2023), a wonderful loving pup. I am forever grateful for the time I had with him.</sup>

---

## What is RustyRoad?

RustyRoad is a Rust **CLI + generator toolkit** inspired by Ruby on Rails.

It focuses on:
- generating a consistent project structure
- generating controllers/routes/models
- generating and running database migrations
- providing a few productivity-focused database commands

Under the hood, generated projects use Actix for HTTP, Tera for templates, and SQLx for database support.

If you're curious about the motivation, there's a short write-up here:
https://rileyseaburg.com/posts/rust-needs-a-rails

## Features

- Project generator (`rustyroad new`)
- Generators (`rustyroad generate ...`)
- Database migrations (`rustyroad migration ...`)
- Database inspection / queries (`rustyroad db ...`, `rustyroad query ...`)
- **MCP Server** for AI agent integration (`rustyroad-mcp`)
- Optional GrapesJS feature (drag-and-drop editor) via `rustyroad feature add grapesjs`

## Install

### From crates.io

```bash
cargo install rustyroad
```

### From source

```bash
git clone --recurse-submodules https://github.com/RustyRoad/RustyRoad
cd RustyRoad
cargo build --release
```

## Quick start

Create a new project:

```bash
rustyroad new my_project
```

Generate a route/controller:

```bash
rustyroad generate route users
```

## Configuration

### How `rustyroad.toml` is used

RustyRoad reads your database settings from a TOML file in your **project root**.

- Default (dev): RustyRoad reads `./rustyroad.toml`
- If `ENVIRONMENT` is set and **not** `dev`: RustyRoad reads `./rustyroad.<ENVIRONMENT>.toml`

Examples:

- `ENVIRONMENT=prod` → reads `rustyroad.prod.toml`
- `ENVIRONMENT=test` → reads `rustyroad.test.toml`

There is **no** special `rustyroad.dev.toml`—dev is the plain `rustyroad.toml` file.

**Tip:** You can also use `ENV=prod` as a shorthand for `ENVIRONMENT=prod`. If both are set, `ENVIRONMENT` wins.

If you're unsure what RustyRoad is going to read on your machine, run:

```bash
rustyroad config
```

(It prints `ENVIRONMENT=...`, the config filename, and a sanitized view of the parsed database settings.)

## Audit handwritten Rust models

Like rogue-migration discovery, `rustyroad model audit` discovers handwritten
declarations and produces a review plan against existing RustyRoad-generated
models and the configured PostgreSQL catalog. It **does not change application
source or the database**; migration files are written only after terminal confirmation
or an explicit `--generate-migrations` flag.

```bash
rustyroad model audit --help
# Browse and select models for declaration and schema comparison:
rustyroad model audit --interactive
# Filter candidates; input paths are relative to --root:
rustyroad model audit --source crates/models/src --name Customer
# Save a machine-readable plan (output must not already exist):
rustyroad --format json model audit --select 'src/models.rs::Customer' --output model-audit.json
# Explicitly map a legacy name; ambiguous names require the report's full ID:
rustyroad model audit --map TikTokToken=TiktokAccessTokens --name TikTokToken
# Filesystem-only analysis, without a database connection or subprocess:
rustyroad --format json model audit --offline
```

Default discovery follows the model generator: `crates/models/src/` when present,
otherwise `src/models/` (or a single `src/models.rs`). It does not sweep unrelated
controllers and jobs. `--all-sources` opts into the previous `src/` + `crates/`
scan; repeat `--source` for exact overrides. Generated RustyRoad files, symlinks,
hidden/artifact folders, `target`, `vendor`, and `node_modules` are excluded.
Generated models are detected at `crates/db/src/models.rs` or `src/db/models.rs`;
use `--generated` or the shared config below for custom or ambiguous locations.

Use repeatable `--name`, `--status`, or exact `--select` IDs for scripts, or
`--interactive` for a terminal checkbox selector. Names are only mapping hints;
row, New and Patch names are not collapsed together. AST comparison reports
custom-only fields, additional generated fields, type spelling changes, attributes,
visibility and generics independently. Enums, aliases and tuple/unit structs need
separate review. `field_syntax_equal` is **not** safe-to-delete approval.

The auditor uses **syn AST declarations** and, when configured, **PostgreSQL
catalog facts**. It does not launch rust-analyzer, Cargo, rustc, build scripts,
or proc macros. Schema comparison and migration planning do not require caller
analysis. Types with unsupported or ambiguous persistence contracts remain
blocked rather than being guessed into tables.

Caller compatibility and usage are **not analyzed**: unknown is not zero, and
no result authorizes deletion or replacement. JSON report version **5** records
`analysis: "ast"` and per-candidate `usage: "not_run"`, replacing `symbols` and
`semantic`. Version 5 adds per-file `runtime_sql` findings to `source_files`;
consumers requiring an exact report version must support version 5. Legacy
`--symbols syntax` is accepted as a no-op; `--symbols semantic`, `--rust-analyzer`,
and `--timeout` are no longer supported.

Exit codes: **0** report produced, **1** invalid input/output, **2** partial
discovery or blocked migration generation. Review blockers and scope even
on exit 0. Reports are snapshots, not HEAD/live-schema verification or migration
patches. Automatic caller rewrites are separate work.

### Review runtime SQL

Discovery scans explicit `sqlx::query*` and `sqlx::raw_sql` call sites before
parsing Rust, including files without model declarations. Each per-file
`runtime_sql` finding records a one-based `line` and Unicode `character` column,
`query_api`, `argument_kind` (`literal`, `dynamic_or_indirect`, or `sql_file`),
and remediation guidance. Human-readable reports show these findings too.
Comments and string/character contents are not treated as call sites. Findings
survive malformed Rust; the original `parse_error` and diagnostic remain intact.
An unterminated escaped character literal is treated as opaque through its line,
so apparent SQLx calls inside it are not reported as code.

These are **advisory lexical findings**, not proof of execution or table ownership.
Aliases, bare imports, query builders, other database clients and excluded
generated files are outside this detector's coverage. It does not expand macros
or resolve Rust names. An empty findings list is not proof that a file is SQL-free.
Inspect SQL, callers and catalog evidence before replacing an operation with
canonical RustyRoad/Drizzle-generated artifacts. Preserve parameters, nullability,
ordering and transaction semantics. Report unsupported generator capabilities
rather than inventing a schema or hand-writing a substitute. Findings neither
authorize SQL execution or source rewriting nor clear generation blockers.

### Match tables and generate migrations

Generated Rust type names do not prove that a table or column exists. Database
settings are now **auto-detected**, so an ordinary `rustyroad model audit`
compares against the configured PostgreSQL catalog without repeating `--database`.
Use `--offline` to forbid database connections, or
`--schema-snapshot catalog.json` to review a previously captured inventory.
`--database` remains an explicit assertion that catalog access is required.
An unconfigured project can still perform a generated-code-only audit.

Environment/config resolution is explicit and reported in the JSON `context`:

1. `--env NAME` (alias `--environment`) overrides `ENVIRONMENT`, then `ENV`, then `dev`.
2. `--config FILE` overrides `RUSTYROAD_CONFIG_PATH`; otherwise `dev` uses
   `rustyroad.toml`, and other environments use `rustyroad.<NAME>.toml`.
3. Connection credentials follow `DATABASE_URL`, then a complete `DATABASE_*`
   set, then the selected config. Credential values are never included in reports.
4. A missing selected environment/config never falls back to another environment.
   `--offline` or a snapshot can be used without resolving database credentials.

Without `--root`, a subdirectory invocation finds the nearest RustyRoad project
above it, stopping at a Git boundary. An explicit `--root` is never changed.
Neither `--env` nor `--config` mutates the process environment. These flags are
audit options; other existing commands continue to use `ENVIRONMENT`/`ENV`.

```bash
rustyroad model audit
rustyroad model audit --env staging
ENVIRONMENT=staging rustyroad model audit
rustyroad model audit --offline
```

The existing `rustyroad generate model NAME`, Rust `pull`, and auditor share
layout settings in the **base** `rustyroad.toml` (not environment-specific DB
files). Defaults need no new configuration. For a custom layout:

```toml
[models]
source = "crates/models/src"
generated = "crates/db/src/models.rs"
```

`source` is the model source directory; `generated` names the generated
`models.rs` file. The model generator writes into `source`; Rust pull writes
into `generated`'s parent directory. Explicit `--out`, `--source`, and
`--generated` flags retain precedence. Layout settings do not automatically
register Rust modules or repair Cargo dependencies.

Table facts, provenance, column differences and SQL proposals appear separately
under `catalog` and each candidate's `table_audit` in the JSON report. No generated
Rust file is required in catalog mode. Ordinary code-only audits still work
without a database connection.

```bash
rustyroad model audit --help
# Show counts and ask whether to generate migrations for both groups:
rustyroad model audit
# No confirmation prompt and no migration writes:
rustyroad model audit --report-only
# Preview the whole batch without per-model mappings:
rustyroad model audit --generate-migrations --dry-run
# Generate both existing-table updates and missing-table creates:
rustyroad model audit --generate-migrations
# Generate only one group (also accepts "new" or "both"):
rustyroad model audit --generate-migrations --migration-scope existing
# Override a new table name and supply an optional explicit primary key:
rustyroad model audit --database --source src/models/new_record.rs \
  --table NewRecord=new_records --primary-key new_records=id \
  --generate-migrations --dry-run
# After reviewing the proposal, omit --dry-run to create canonical files:
rustyroad model audit --database --source src/models/new_record.rs \
  --table NewRecord=new_records --primary-key new_records=id \
  --generate-migrations
```

The report tells you how many declarations were found, how many selected models
match existing tables, how many have no table, and how many remain unresolved.
It separately counts unique update/create migration proposals and blocked models.
In a terminal, review model-to-table names and choose **both**, **existing only**,
**missing only**, or **report only** (the default). `--interactive` additionally
offers model checkboxes, initially all selected. JSON and piped runs never prompt
or generate files without `--generate-migrations`; `--report-only` disables prompts.

Unambiguous existing-table contracts match automatically against the catalog.
Partial projections need not repeat every database column or its primary key:
unrepresented columns are retained. Multiple matching relations still require
an explicit `--table` mapping.

Literal, typed SQLx whole-row queries in the declaration's own source scope can
also establish table ownership (`mapping: "source_row_query_contract"`). For
example, `sqlx::query_as::<_, Self>("SELECT * FROM ad_attribution_session")` in
an inherent implementation identifies that table even when name normalization
would choose something else. Supported evidence includes single-relation
`SELECT *` and `INSERT`/`UPDATE`/`DELETE ... RETURNING *`, including `query_as!`
and source-resolved constructor aliases. This is **static/local** source
analysis, not compiled SQLx or executed PostgreSQL proof.

Projections, joins, CTEs, compound statements, schema-qualified relations,
dynamic SQL, conditional/deferred scopes, and ambiguous or shadowed bindings do
not establish this contract. Conflicting whole-row targets block inference;
an explicit reviewed `--table` mapping remains authoritative. Query ownership
does not infer keys, defaults, constraints, or otherwise bypass catalog, codec,
shared-table, or migration-history safeguards.

Unmatched concrete structs receive a snake_case
candidate name, but **absence is not proof of persistence**: without an explicit
persistence contract, they remain unresolved and do not generate CREATE TABLE.
Confirm that the declaration represents a persisted row, then use
`--table 'PATH::TYPE=TABLE'` to establish the reviewed missing-table mapping.
Do not map request/response DTOs merely to clear diagnostics. Enums, aliases, generic structs and
unsupported fields remain unresolved or blocked. Incomplete catalogs never prove
table absence. Declarations sharing a table are reconciled before selection;
compatible additions generate once, and conflicting contracts block that table group.
With `--generate-migrations`, blocked table groups are skipped while unrelated
eligible existing-table updates and missing-table creations still generate files.
The report lists each skipped table and its reasons, and exits **2** if anything
was skipped—even when other migration files were generated. The text report ends
with generated/reused/skipped counts; JSON lists written or reused directories in
`migration_files` and skip reasons in `diagnostics`. `--dry-run` reports the same
table blockers but writes no migration files.

`--table MODEL=table` overrides the name/ownership mapping; use full model IDs
when names are ambiguous. Optional `--primary-key TABLE=column[,column]` adds a
reviewed non-null primary key, or explicitly replaces an existing key. Existing-key
redesign requires catalog evidence of the original constraint name, deferrability,
ordered columns, supported index properties and inbound foreign-key dependencies.
Missing restoration evidence, dependent foreign keys and unsupported indexes block
redesign. Both directions drop constraints with `RESTRICT`, never `CASCADE`, and
restore the original key in DOWN. Review uniqueness, locking and rollback dependencies;
no duplicate rows are removed or key values invented. Otherwise new tables have
**no inferred primary key**. Identity/serial, defaults, unique constraints, foreign keys,
indexes and checks are not inferred. New tables use the shared RustyRoad
`migration generate` CREATE/DROP renderer, with strict schema-qualified SQL and
the audit's retry-safe writer (no subprocess). Existing-table drafts can add
supported columns, change types with explicit `USING` casts, and set/drop
`NOT NULL`. Each direction is one atomic `ALTER TABLE` statement. These are
reviewable drafts, not a guarantee that existing data can be converted: required
additions on populated tables need a reviewed default or staged backfill;
`SET NOT NULL` requires existing NULLs to be resolved. No backfill values are
invented. Warnings cover potentially lossy casts, dependent constraints,
locking and rollback failures. Making an existing primary-key column nullable
requires explicit key redesign. Nullable replacement keys, views, SQLx rename/codec
attributes and ambiguous contracts remain blocked. Shared-table
contracts must agree even when one declaration already matches the database.
OpenAPI `#[schema(...)]`
and Serde attributes do not block table generation: they describe documentation
and wire representation, not SQL storage. They remain in the audit report;
Serde renames do not rename SQL columns.
Extra database columns are retained, never dropped. Builtin Rust storage types
are inferred conservatively; field type spelling is not compiler type identity.

Custom types (including enums, JSON wrappers, arrays, domains and spatial types)
can use reusable PostgreSQL storage mappings in the base `rustyroad.toml`:

The auditor also recognizes source-declared SQLx storage names in the audited
file and declared same-crate modules: explicit `#[derive(sqlx::Type)]` with
`#[sqlx(type_name = "...")]`, or an unconditional `Type<Postgres>` implementation
whose `type_info()` returns a literal `PgTypeInfo::with_name("...")`. Explicit
imports and aliases are resolved; basenames alone are not codec evidence.
Absolute imports such as `use ::codec::State` retain their external-crate
identity and cannot borrow a same-named local module's storage contract.
Unqualified prelude/external type names in a wildcard-import scope are not enough
to establish a storage codec: an unknown glob may shadow names such as `String`,
`Option`, or `uuid`. Absolute paths and imports (for example,
`use ::uuid::Uuid as Id`) retain their identity; explicit source-declared codecs
and configured mappings remain usable. Wildcard uncertainty is scope-local.
Source-proven value-only helpers are an exception: for example,
`mod helper { pub fn log() {} } use self::helper::*;` cannot shadow `String`.
Every target in a grouped wildcard import must meet this rule. Functions,
constants, statics, impls, and private imports qualify only with documentation
or lint-level attributes; type declarations, reexports, nested modules, macros,
conditional attributes, and unresolved targets retain uncertainty. Importing
and target module ownership must also be unambiguous and attribute-safe.
Inline-only lookup is limited to standalone files outside Cargo ownership;
an undeclared or conditional module in a Cargo package cannot bypass this
guard. Ancestor globs such as `super::*` are not value-only helper evidence.
Bare absolute names such as `::String` do not identify prelude types; use an
explicit qualified path such as `::std::string::String` instead.
Enum `rename_all` rules do not change the declared storage name. Conditional,
duplicate, transparent, generic, computed, and external-crate contracts remain
unresolved. Cross-file lookup requires a package manifest, a declared library
target (`src/lib.rs` or `[lib].path`), and an unambiguous unconditional module
chain. For example, `use crate::codec::State` follows `mod codec;` to
`src/codec.rs` or `src/codec/mod.rs`; explicit reexports are followed with a
bounded walk. Conditional modules, `#[path]` overrides and wildcard exports are
not inferred. This is source-only evidence, not compiled codec validation, and
it does not establish table ownership or create PostgreSQL types. Configured
and CLI mappings take precedence over these source contracts.

```toml
[models.type_mappings]
Payload = "jsonb"
OrderState = "public.order_state"
# Example only: confirm geometry/geography, shape and SRID against your codec.
DbPolygon = "geometry(Polygon,4326)"
```

Then `rustyroad model audit --generate-migrations` can use those storage mappings
for existing-table updates and explicitly mapped missing tables. A type mapping
does not establish table ownership or authorize primary-key redesign.
Alternatively, repeat `--type-map 'Payload=jsonb'` on the command line; CLI
mappings override configured defaults and builtin inference. Map the inner
type for `Option<T>`; nullability still comes from the Rust declaration. Matching
uses Rust type syntax, so imported and fully qualified spellings need their own
entries. Only a parsed PostgreSQL datatype is accepted, never SQL statements or
column constraints. Mappings declare storage; they do not create extensions,
domains or enum definitions, or verify a custom Rust codec. Referenced database
types/extensions must exist when migrations are eventually applied.

Generation writes `config/database/migrations/<timestamp>-audit_<table>_<hash>/`
with `up.sql`, `down.sql` and a `model-audit.json` provenance manifest. It never
applies SQL. UP uses strict, schema-qualified, quoted DDL; DOWN is explicitly
destructive to data added after the migration and needs rollback review. An
identical retry reuses the previous directory. Source discovery diagnostics do
not discard eligible proposals from successfully parsed declarations: generated
drafts retain an incomplete-discovery warning in their provenance manifests and
the audit still exits **2**. Review those diagnostics and all source contracts
before applying SQL; a draft does not establish complete model discovery.
Edited/partial artifacts, symlinks, concurrent/stale generation locks, and
unreadable history still block the eligible batch before writes. Detected
overlapping DDL skips the affected table with an explicit diagnostic, while
unrelated eligible tables continue. The writer rechecks history under its own
exclusive lock. Simple historical `ALTER COLUMN` nullability changes, default
removals, and type changes without `USING` only conflict with repairs on the
same column; literal defaults and bounded `CHECK` expressions also use their
explicit column dependencies. Disjoint columns can still receive drafts, but
every action in a historical statement must be understood before its scope is
narrowed. Whole-table proposals, computed defaults, function calls, subqueries,
procedural SQL, and unrecognized operations retain the conservative table-level
overlap check. History checking is not a simulation of
arbitrary procedural/dynamic SQL; review the pending migration chain before
execution. Historical `CREATE TABLE` statements block duplicate table creation
and additions of columns declared by that statement, but do not alone block
catalog-backed repairs to existing columns. For example, a historical
`CREATE TABLE items (id bigint)` does not itself prevent an `ALTER COLUMN id`
repair; a historical repair on `id` still triggers the overlap check.
A failed batch retains any earlier files and names them in its report. No
automatic cleanup hides partial writes.

Live access uses a read-only repeatable-read catalog transaction, never the
database-creation helper. It respects `DATABASE_URL`, complete `DATABASE_*`
settings, then the environment-specific RustyRoad config under `--root`.
`--schema` selects the live PostgreSQL schema (default `public`). Saved snapshots
must describe the full schema, not a filtered table list; they cannot prove that
the live database still matches. To reuse the inventory from a JSON audit report,
extract its `.catalog.catalog` value. The snapshot format is:

```json
{
  "version": 1,
  "schema": "public",
  "complete": true,
  "reserved_names": ["records"],
  "tables": [{
    "name": "records", "kind": "table", "primary_key": ["id"],
    "columns": [{"name": "id", "sql_type": "integer", "nullable": false}]
  }]
}
```

## Generate an API from PostgreSQL

`rustyroad pull` introspects the live PostgreSQL schema and generates a complete
typed database API. TypeScript remains the default target:

```bash
rustyroad pull
```

This writes Drizzle tables and repositories, Zod schemas, oRPC procedures, a
Fastify server adapter, OpenAPI, and a Hey API configuration to `./db`.

To also write a standalone Drizzle/Zod schema pair for application validation,
use:

```bash
rustyroad pull --zod
```

This writes `schema.ts` and `zod.ts` to `./src/schemas`; pass `--zod-out` to
choose another folder. `--zod-models` and `--zod-models-out` are accepted as
aliases.

### Typed JSON/JSONB columns

Unannotated PostgreSQL `json` and `jsonb` columns intentionally generate as
`Record<string, unknown>`. To make the database own a richer shape, put a JSON
Schema on the column comment after the `@rustyroad-json-schema` marker:

```sql
COMMENT ON COLUMN platform_trash_zone_mappings.coverage_metrics IS
  'Targeting coverage metrics.
@rustyroad-json-schema {"type":"object","properties":{"households":{"type":"integer"},"radiusMiles":{"type":"number"}},"required":["households"],"additionalProperties":false}';
```

The marker and its compact JSON value must occupy one line. Normal prose may
appear on other lines. A malformed marker stops `pull` rather than silently
weakening the generated contract.

RustyRoad uses the annotation for all TypeScript outputs:

- the Drizzle column's `$type<...>()` declaration;
- `drizzle-zod` select, insert, and update refinements;
- the OpenAPI 3.1 property schema and generated client type.

The supported JSON Schema shape keywords are `type`, `properties`, `required`,
`additionalProperties`, `items`, `enum`, `const`, `oneOf`, and `anyOf`. Supported
scalar types are `string`, `number`, `integer`, `boolean`, and `null`. Keywords
outside this subset remain in OpenAPI but do not add TypeScript or Zod behavior.
Normalize data into typed columns or related tables when it must participate in
foreign keys, indexes, uniqueness, or relational queries.

Use the Rust target for the equivalent Actix + SQLx stack:

```bash
rustyroad pull --language rust
```

The Rust target writes `./src/db` by default:

- `models.rs` — SQLx row types and separate create/patch input types
- `repositories.rs` — bound, typed CRUD queries for every table with a
  single-column primary key
- `procedures.rs` — Actix handlers for list/get/create/update/delete under
  `/api`
- `api.rs` — the developer-owned composition point for custom services
- `mod.rs` — the module facade exported to the application

Register the generated procedures with the application's pool:

```rust,ignore
mod db;

HttpServer::new(move || {
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .configure(db::configure)
})
```

Database-derived files are regenerated on each pull. Composition files are
written once and preserved, so custom code in `api.rs` or `api.ts` survives.
Pass `--force` only when those files should be reset.

The Rust output expects `actix-web`, `serde`, and SQLx's Postgres/runtime and
database-type features. The command prints the exact `cargo add` invocation
after generation. At present, `pull` introspection is PostgreSQL-only.

## Migrations

RustyRoad expects migrations in this exact location (do **not** create a plain `./migrations/` folder):

```
./config/database/migrations/<timestamp>-<name>/up.sql
./config/database/migrations/<timestamp>-<name>/down.sql
```

### Migration Commands

List migrations:

```bash
rustyroad migration list
ENV=test rustyroad migration list
```

Repair missing undo files without rewriting existing migrations:

```bash
rustyroad migration repair-down          # preview only
rustyroad migration repair-down --write  # create missing down.sql files only
```

This offline command never executes SQL or changes existing `up.sql`/`down.sql`
files. Ambiguous reversals require explicitly reviewed undo SQL, not a guessed
or partial rollback. See [the undo repair guide](MIGRATION_UNDO_REPAIR.md) for
supported SQL, exact-source review bundles, per-file publication semantics,
and safety limitations.

Repair a legacy ledger that contains duplicate rows:

```bash
rustyroad migration repair-ledger
ENVIRONMENT=prod rustyroad migration repair-ledger --yes
```

The repair runs in one transaction, keeps the newest state for each migration
identity, and restores the uniqueness guard. It does not execute `up.sql` or
`down.sql` files.

### Ledger provenance is not effect verification

`rustyroad migration list` reports whether a migration is **recorded in the
ledger**, not whether its SQL or live database effects have been proven. Each
new ledger row records provenance and the SHA-256 checksum of its migration SQL:

- `executed` — RustyRoad successfully submitted the SQL before recording it
- `baselined` — the row was adopted without executing the SQL
- `legacy` — the row predates provenance tracking

Effects remain `UNVERIFIED` unless separate verification evidence populates
`verified_at`. In particular, `rustyroad migration baseline` never executes SQL;
it records `baselined` provenance and causes subsequent migration runs to skip
the recorded identities. Use `rustyroad db schema` as live evidence for tables
and columns, while remembering that it does not verify data changes or every
database object.

`migration version-status` reads `_rustyroad_history`, which is intentionally
separate from `_rustyroad_migrations`. A ledger baseline therefore does not
become a published schema version.

Run all migrations (up) in order:

```bash
rustyroad migration all
```

Validate the complete migration chain without modifying a persistent database:

```bash
ENVIRONMENT=test rustyroad migration validate
ENVIRONMENT=prod rustyroad migration validate
```

Validation reads the active environment configuration: `rustyroad.toml` for dev or `rustyroad.<environment>.toml` for environments such as test, staging, and prod. It never connects to the configured `database_name`. Instead, it creates a randomly named disposable database on the configured PostgreSQL/MySQL server (or an OS-managed temporary file for SQLite), runs every `up.sql` in timestamp order through a generated login scoped to that disposable database, and removes the disposable database and login after success or failure.

When validating with `ENVIRONMENT=prod`, disposable resources are created on the server from `rustyroad.prod.toml`, but the configured production database is not opened or modified. The configured administrative user must be allowed to create and drop databases and temporary roles/users.

### Breaking migration warnings

RustyRoad scans generated migrations and every `up.sql` before `migration run` or `migration all` opens a database connection. It warns about operations that can destroy data or break foreign-key compatibility, including:

- PostgreSQL `ALTER COLUMN ... TYPE` and `USING` conversions
- Explicit casts such as `CAST(...)` and `::type`
- MySQL `MODIFY COLUMN` and `CHANGE COLUMN`
- Dropped constraints or foreign keys

Interactive apply commands require confirmation when findings exist. Non-interactive commands stop with exit code 2. After reviewing both sides of every foreign key, validating against a disposable database, and backing up the target database, automation can acknowledge the risk explicitly:

```bash
rustyroad migration run change_customer_id_type --allow-breaking
rustyroad migration all --allow-breaking
```

The preflight is intentionally conservative: it identifies risky SQL but cannot prove that a cast preserves every value or that application code remains compatible.

Run a single migration by name (the name is the part after the timestamp in the folder name):

```bash
rustyroad migration run create_users_table
```

Rollback a migration:

```bash
rustyroad migration rollback create_users_table
```

Generate a migration (folder + files):

```bash
rustyroad migration generate create_users_table id:serial:primary_key email:string:not_null,unique
```

### Auto-convert Rogue Migrations

If you (or an AI agent) accidentally created migrations in the wrong location (like `./migrations/`), RustyRoad can detect and convert them:

```bash
# Preview what would be converted
rustyroad migration convert --dry-run

# Convert and keep source files
rustyroad migration convert

# Convert and remove source files
rustyroad migration convert --remove-source
```

RustyRoad will also warn you when running any migration command if it detects rogue migrations.

## Database commands

Inspect schema:

```bash
rustyroad db schema
```

Inspect just one table's columns (name, type, and nullability):

```bash
rustyroad db schema users
rustyroad --format json db schema users
```

Discover tables non-interactively (including for LLMs and scripts):

```bash
rustyroad --format json db tables
rustyroad --format json db tables --prefix us
```

`db tables` returns `database_type`, `config_file`, and a `tables` array of names
in JSON mode. Prefix matching is literal and case-sensitive; no matches returns
an empty array. `db schema TABLE` retains the existing schema JSON structure,
with one table; a missing table reports an error on stderr and exits nonzero.

For humans, choose a table with live suggestions:

```bash
rustyroad db schema --interactive
```

The prompt loads table names once, suggests prefix matches as you type, and
supports **Tab** completion. Press **Enter** with the table name to inspect it;
**Ctrl-C** or **Ctrl-D** cancels. This is an in-command prompt, not shell
completion. It requires a terminal and text output; agents should use the
non-interactive commands above. PostgreSQL uses `public`, MySQL the configured
database, and SQLite the main database. No table rows are read.

Inspect enum types and their allowed values:

```bash
rustyroad db enums
```

Run ad-hoc queries:

```bash
rustyroad query "SELECT * FROM users LIMIT 10;"
rustyroad query "SELECT COUNT(*) AS total_users FROM users;"
```

## MCP Server (AI Agent Integration)

RustyRoad includes an MCP (Model Context Protocol) server that exposes database tools to AI agents like OpenCode, Claude, etc. This prevents agents from using raw `psql` commands or connecting to the wrong database.

### Available Tools

- `rustyroad_query` - Execute SQL queries
- `rustyroad_schema` - Get database schema  
- `rustyroad_enums` - Get database enum types and their allowed values
- `rustyroad_migrate` - Run migrations
- `rustyroad_migration_generate` - Create new migrations
- `rustyroad_config` - View configuration
- `rustyroad_convert_migrations` - Fix rogue migrations

### Setup

Register with OpenCode:

```bash
rustyroad-mcp --register
```

Or manually add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "rustyroad": {
      "type": "local",
      "command": ["/path/to/rustyroad-mcp"],
      "enabled": true,
      "environment": {
        "RUSTYROAD_PROJECT_DIR": "/path/to/your/project"
      }
    }
  }
}
```

## Optional: GrapesJS

RustyRoad can scaffold an optional GrapesJS editor experience:

```bash
rustyroad feature add grapesjs
```

You can learn more about GrapesJS at https://grapesjs.com/ and see the example project at `example-grapesjs/`.

## Examples

- `example/` – a basic generated app
- `example-grapesjs/` – a generated app with GrapesJS enabled

## Troubleshooting

### Building from source on Windows (PostgreSQL linkage)

If you build this repository from source on Windows and see errors about `POSTGRES_LIB_PATH` or `libpq.lib`:

1. Install PostgreSQL from the [official website](https://www.postgresql.org/download/windows/)
2. Set `POSTGRES_LIB_PATH` environment variable to the directory containing `libpq.lib` (e.g., `C:\Program Files\PostgreSQL\13\lib`)
3. For generated projects, create `.cargo/config.toml` in your project root:

```toml
[target.'cfg(windows)']
rustflags = ["-C", "link-arg=/LIBPATH:C:\\Program Files\\PostgreSQL\\13\\lib"]
```

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md`.

## License

MIT — see `LICENSE`.