//! Server wiring for the generated oRPC router.
//!
//! Emits the Fastify adapter and the script that writes the OpenAPI document, so
//! the same router serves RPC, REST, and client generation without hand-written
//! glue.

mod server;

pub use server::server;

/// Renders `openapi.ts`, the script that writes the document.
///
/// oRPC derives the document from the procedures' own Zod schemas, so it cannot
/// describe an endpoint the server does not serve, or a shape it does not validate.
pub fn openapi_script() -> String {
    String::from(
        "import { writeFile } from \"node:fs/promises\";\n\
         import { OpenAPIGenerator } from \"@orpc/openapi\";\n\
         import { ZodToJsonSchemaConverter } from \"@orpc/zod/zod4\";\n\
         import { router } from \"./router.js\";\n\n\
         /**\n\
          * Writes the OpenAPI document for the generated router.\n\
          *\n\
          * Run this in CI before generating a client, so no server needs to boot:\n\
          *   npx tsx db/openapi.ts db/openapi/openapi.json\n\
          */\n\
         const generator = new OpenAPIGenerator({\n\
         \tschemaConverters: [new ZodToJsonSchemaConverter()],\n\
         });\n\n\
         const document = await generator.generate(router, {\n\
         \tinfo: { title: \"Generated API\", version: \"1.0.0\" },\n\
         });\n\n\
         const target = process.argv[2] ?? \"db/openapi/openapi.json\";\n\
         await writeFile(target, `${JSON.stringify(document, null, 2)}\\n`);\n\
         console.log(`wrote ${target}`);\n",
    )
}
