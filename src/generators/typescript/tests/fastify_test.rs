//! Fastify route structure and the Zod type provider.

use super::support::schema;
use crate::generators::typescript::fastify::render;
use crate::generators::typescript::Casing;

#[test]
fn every_verb_is_generated_as_a_plugin_route() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("app.get(\"/\""));
    assert!(ts.contains("app.get(\"/:id\""));
    assert!(ts.contains("app.post(\"/\""));
    assert!(ts.contains("app.patch(\"/:id\""));
    assert!(ts.contains("app.delete(\"/:id\""));
}

#[test]
fn plugins_use_the_zod_type_provider() {
    let ts = render(&schema(), Casing::Camel);

    // The provider infers request and reply types from the schemas, so no explicit
    // route generics are needed and handlers stay type-checked.
    assert!(ts.contains("FastifyPluginAsyncZod"));
    assert!(ts.contains("from \"fastify-type-provider-zod\""));
    assert!(ts.contains("export const usersRoutes: FastifyPluginAsyncZod<RouteOptions>"));
}

#[test]
fn plugins_are_registered_under_table_prefixes() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("export const apiRoutes: FastifyPluginAsyncZod<RouteOptions>"));
    assert!(ts.contains("app.register(usersRoutes, { ...options, prefix: \"/users\" })"));
    assert!(ts.contains("app.register(postsRoutes, { ...options, prefix: \"/posts\" })"));
}

#[test]
fn deletes_send_an_explicit_null_body() {
    let ts = render(&schema(), Casing::Camel);

    // A 204 carries no body, but the Zod provider still types `send`.
    assert!(ts.contains("reply.code(204).send(null)"));
}
