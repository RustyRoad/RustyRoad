//! Long-form help for `rustyroad pull`.

/// Returns the command's detailed description.
pub(super) fn long_about() -> &'static str {
    "Reads the live database schema and writes a generated TypeScript or Rust API.\n\n\
     TYPESCRIPT OUTPUT (default ./db):\n\
      schema.ts     Drizzle table definitions, enums, constraints, and indexes\n\
      relations.ts  one/many relations derived from foreign keys\n\
      client.ts     a typed repository per table\n\
      zod.ts        Zod schemas derived from the tables\n\
      router.ts     oRPC procedures over those repositories\n\
      api.ts        composition point: generated plus your own procedures\n\
      server.ts     Fastify adapter serving RPC and REST\n\
      openapi.ts    script writing the OpenAPI document from the router\n\
      openapi/      a static OpenAPI document plus a Hey API config\n\n\
     RUST OUTPUT (--language rust, default ./src/db):\n\
      models.rs       SQLx row models plus typed create and patch inputs\n\
      repositories.rs typed SQLx CRUD repositories with bound parameters\n\
      procedures.rs   Actix CRUD handlers and /api route registration\n\
      api.rs          composition point: generated plus your own services\n\
      mod.rs          module facade exporting the API configuration\n\
      openapi/        the same static OpenAPI document and Hey API config\n\n\
     FILE OWNERSHIP:\n\
      Files derived from the database are rewritten on every run. Composition\n\
      files (`api.ts`, or `api.rs` and `mod.rs`) plus the Hey API config are\n\
      written once and then left alone, so hand-written code survives. Pass\n\
      --force to overwrite those too.\n\n\
      Because api.ts is preserved, a table added later is not wired up\n\
      automatically; pull warns when that happens and prints the line to add.\n\n\
      Rust's stable `configure_generated` entry point automatically includes new\n\
      tables without rewriting api.rs.\n\n\
     POSTGRES ONLY:\n\
      Introspection reads the Postgres catalog. Other backends are not\n\
      supported by this command yet.\n\n\
     RUST MODELS (--models, default ./src/models):\n\
      One folder per table, following the layout hand-written models use:\n\
        mod.rs      the struct, its derives, Default, and new\n\
        create.rs   the insert\n\
        read.rs     the listing and the keyed lookup\n\
        update.rs   the update\n\
        delete.rs   the delete\n\
      Each CRUD file adds an impl block to the struct in mod.rs. The parent\n\
      mod.rs is written once and then left alone, so hand-written models\n\
      declared beside the generated ones survive.\n\n\
     TETHERSCRIPT MODELS (--tether-models, default ./models):\n\
      The same folder layout in .tether files, reaching SQL through the `db`\n\
      capability. TetherScript has no structs, so a row is a map keyed by\n\
      column name and mod.tether carries what a struct would: the column\n\
      list, seeded defaults, and a validate() checking required columns and\n\
      value kinds before any write.\n\n\
     ZOD SCHEMAS (--zod, default ./src/schemas):\n\
      A standalone schema.ts and zod.ts pair. The latter derives select, insert,\n\
      and update validators from the generated Drizzle definitions, keeping the\n\
      validation types in lockstep with the database. Use --zod-out to choose a\n\
      different folder.\n\n\
     CONFIG:\n\
      Database connection from ./rustyroad.toml (or ./rustyroad.<ENVIRONMENT>.toml).\n\n\
     EXAMPLES:\n\
      rustyroad pull\n\
      rustyroad pull --language rust\n\
      rustyroad pull --out ./src/db --casing preserve\n\
      rustyroad pull --schema-only\n\
      rustyroad pull --models\n\
      rustyroad pull --models --models-out ./src/models\n\
      rustyroad pull --tether-models\n\
      rustyroad pull --zod\n\
      rustyroad pull --zod --zod-out ./src/schemas\n"
}
