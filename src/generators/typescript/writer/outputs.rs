//! Which artifacts `pull` emits.

/// Selected outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outputs {
    pub schema: bool,
    pub relations: bool,
    pub client: bool,
    /// Zod schemas derived from the Drizzle tables.
    pub zod: bool,
    /// oRPC procedures plus the Fastify adapter; the API surface.
    pub api: bool,
    /// OpenAPI document plus Hey API config, written to an `openapi/` subfolder.
    pub sdk: bool,
}

impl Outputs {
    /// Everything, as `pull` emits by default.
    pub fn all() -> Self {
        Self {
            schema: true,
            relations: true,
            client: true,
            zod: true,
            api: true,
            sdk: true,
        }
    }

    /// Only the Drizzle schema and relations, mirroring `drizzle-kit pull`.
    pub fn schema_only() -> Self {
        Self {
            schema: true,
            relations: true,
            client: false,
            zod: false,
            api: false,
            sdk: false,
        }
    }

    /// The Drizzle definitions and Zod schemas they derive.
    ///
    /// `zod.ts` imports the table definitions, so a standalone Zod target needs
    /// `schema.ts` as well. Keeping that dependency generated avoids maintaining a
    /// second PostgreSQL-to-Zod type mapper that can drift from the main pull output.
    pub fn zod_only() -> Self {
        Self {
            schema: true,
            relations: false,
            client: false,
            zod: true,
            api: false,
            sdk: false,
        }
    }
}
