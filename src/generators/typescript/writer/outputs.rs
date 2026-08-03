//! Which artifacts `pull` emits.

/// Selected outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outputs {
    pub schema: bool,
    pub relations: bool,
    pub client: bool,
    /// Zod schemas plus Fastify routes; the API surface.
    pub api: bool,
}

impl Outputs {
    /// Everything, as `pull` emits by default.
    pub fn all() -> Self {
        Self {
            schema: true,
            relations: true,
            client: true,
            api: true,
        }
    }

    /// Only the Drizzle schema and relations, mirroring `drizzle-kit pull`.
    pub fn schema_only() -> Self {
        Self {
            schema: true,
            relations: true,
            client: false,
            api: false,
        }
    }
}
