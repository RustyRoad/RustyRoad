//! Long-form help for `rustyroad pull`.

/// Returns the command's detailed description.
pub(super) fn long_about() -> &'static str {
    "Reads the live database schema and writes generated TypeScript.\n\n\
     OUTPUT (default ./db):\n\
      schema.ts     Drizzle table definitions, enums, constraints, and indexes\n\
      relations.ts  one/many relations derived from foreign keys\n\
      client.ts     a typed repository per table\n\
      zod.ts        Zod schemas derived from the tables\n\
      router.ts     oRPC procedures over those repositories\n\
      api.ts        composition point: generated plus your own procedures\n\
      server.ts     Fastify adapter serving RPC and REST\n\
      openapi.ts    script writing the OpenAPI document from the router\n\
      openapi/      a static OpenAPI document plus a Hey API config\n\n\
     FILE OWNERSHIP:\n\
      Files derived from the database are rewritten on every run. `api.ts` and\n\
      `openapi/openapi-ts.config.ts` are written once and then left alone, so\n\
      hand-written procedures and config edits survive. Pass --force to\n\
      overwrite those too.\n\n\
      Because api.ts is preserved, a table added later is not wired up\n\
      automatically; pull warns when that happens and prints the line to add.\n\n\
     POSTGRES ONLY:\n\
      Introspection reads the Postgres catalog. Other backends are not\n\
      supported by this command yet.\n\n\
     CONFIG:\n\
      Database connection from ./rustyroad.toml (or ./rustyroad.<ENVIRONMENT>.toml).\n\n\
     EXAMPLES:\n\
      rustyroad pull\n\
      rustyroad pull --out ./src/db --casing preserve\n\
      rustyroad pull --schema-only\n"
}
