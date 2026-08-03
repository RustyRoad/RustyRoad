//! The Fastify adapter serving the generated router.

mod mount;
mod route;

/// Renders `server.ts`.
///
/// Mounting both handlers is what makes the router one surface: RPC for the typed
/// client, REST for everything else, from the same procedures.
pub fn server() -> String {
    format!("{}{}", imports(), mount::mount())
}

/// Renders the imports and the two handlers.
///
/// The router comes from `api.ts` rather than `router.ts`, so hand-written
/// procedures composed there are served too.
fn imports() -> String {
    String::from(
        "import type { FastifyInstance } from \"fastify\";\n\
         import { OpenAPIHandler } from \"@orpc/openapi/fetch\";\n\
         import { RPCHandler } from \"@orpc/server/fetch\";\n\
         import type { Database } from \"./client\";\n\
         import { router } from \"./api\";\n\n\
         /** Serves the router over REST, using the declared method and path. */\n\
         const openApiHandler = new OpenAPIHandler(router);\n\n\
         /** Serves the router over RPC, for the typed client. */\n\
         const rpcHandler = new RPCHandler(router);\n\n",
    )
}
