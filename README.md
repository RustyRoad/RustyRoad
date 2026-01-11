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

**Tip:** You can also use `ENV=prod` as a shorthand for `ENVIRONMENT=prod`.

If you're unsure what RustyRoad is going to read on your machine, run:

```bash
rustyroad config
```

(It prints `ENVIRONMENT=...`, the config filename, and a sanitized view of the parsed database settings.)

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
```

Run all migrations (up) in order:

```bash
rustyroad migration all
```

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
