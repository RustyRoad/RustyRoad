//! The catch-all route delegating to the right handler.

/// Renders the route body.
///
/// One route serves both transports: the path decides which handler runs, so a
/// procedure is reachable by RPC and by its declared REST path without duplication.
pub(super) fn route() -> String {
    format!("{}{}{}", open(), forward(), reply())
}

/// Renders the route opening and handler selection.
fn open() -> String {
    String::from(
        "\tapp.all(`${prefix}/*`, async (request, reply) => {\n\
         \t\tconst url = new URL(request.url, `http://${request.headers.host ?? \"localhost\"}`);\n\
         \t\tconst rpcPrefix = `${prefix}/rpc` as `/${string}`;\n\
         \t\tconst isRpc = url.pathname.startsWith(rpcPrefix);\n\
         \t\tconst handler = isRpc ? rpcHandler : openApiHandler;\n\n",
    )
}

/// Renders the request forwarded to the handler.
///
/// A GET or HEAD carries no body, and passing the raw stream for the rest avoids
/// buffering the payload twice.
fn forward() -> String {
    String::from(
        "\t\tconst { matched, response } = await handler.handle(\n\
         \t\t\tnew Request(url, {\n\
         \t\t\t\tmethod: request.method,\n\
         \t\t\t\theaders: request.headers as RequestInit[\"headers\"],\n\
         \t\t\t\tbody: request.method === \"GET\" || request.method === \"HEAD\"\n\
         \t\t\t\t\t? undefined\n\
         \t\t\t\t\t: (request.raw as unknown as RequestInit[\"body\"]),\n\
         \t\t\t\tduplex: \"half\",\n\
         \t\t\t} as RequestInit),\n\
         \t\t\t{ context: { db }, prefix: isRpc ? rpcPrefix : undefined },\n\
         \t\t);\n\n",
    )
}

/// Renders the reply, translating the handler's response back to Fastify.
fn reply() -> String {
    String::from(
        "\t\tif (!matched) return reply.code(404).send({ error: \"Not found\" });\n\n\
         \t\treply.code(response.status);\n\
         \t\tresponse.headers.forEach((value, key) => reply.header(key, value));\n\
         \t\treturn reply.send(response.body ? await response.text() : null);\n\
         \t});\n\
         }\n",
    )
}
