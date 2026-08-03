//! The mount function attaching both handlers to Fastify.

use super::route;

/// Renders `mountRouter`.
pub(super) fn mount() -> String {
    format!("{}{}{}", signature(), parsers(), route::route())
}

/// Renders the doc comment and signature.
fn signature() -> String {
    String::from(
        "/**\n\
         \x20* Mounts both handlers on a Fastify instance.\n\
         \x20*\n\
         \x20* `prefix` must match the paths declared on the procedures. Requests under\n\
         \x20* `<prefix>/rpc` are handled as RPC; everything else is routed as REST.\n\
         \x20*/\n\
         export async function mountRouter(\n\
         \tapp: FastifyInstance,\n\
         \tdb: Database,\n\
         \tprefix = \"/api\",\n\
         ): Promise<void> {\n",
    )
}

/// Renders the content-type parser reset.
///
/// The oRPC handlers read the raw request, so Fastify must not consume the body.
fn parsers() -> String {
    String::from(
        "\t// The pattern is anchored so it matches the essence MIME type rather than\n\
         \t// any substring, which Fastify warns about as a CORS risk.\n\
         \tapp.removeAllContentTypeParsers();\n\
         \tapp.addContentTypeParser(/^.*$/, (_request, payload, done) => done(null, payload));\n\n",
    )
}
