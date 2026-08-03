//! Mutating route rendering: create, update, delete.

/// Renders the create route.
pub(super) fn create(name: &str, type_name: &str) -> String {
    format!(
        "\tapp.post(\"/\", {{\n\
         \t\tschema: {{\n\
         \t\t\ttags: [\"{type_name}\"],\n\
         \t\t\toperationId: \"create{type_name}\",\n\
         \t\t\tbody: {name}InsertSchema,\n\
         \t\t\tresponse: {{ 201: {name}SelectSchema }},\n\
         \t\t}},\n\
         \t}}, async (request, reply) => {{\n\
         \t\tconst row = await {name}Repository.create(db, request.body);\n\
         \t\treturn reply.code(201).send(row);\n\
         \t}});\n"
    )
}

/// Renders the partial-update route.
pub(super) fn update(name: &str, type_name: &str) -> String {
    format!(
        "\tapp.patch(\"/:id\", {{\n\
         \t\tschema: {{\n\
         \t\t\ttags: [\"{type_name}\"],\n\
         \t\t\toperationId: \"update{type_name}\",\n\
         \t\t\tparams,\n\
         \t\t\tbody: {name}UpdateSchema,\n\
         \t\t\tresponse: {{ 200: {name}SelectSchema, 404: errorSchema }},\n\
         \t\t}},\n\
         \t}}, async (request, reply) => {{\n\
         \t\tconst row = await {name}Repository.update(db, request.params.id, request.body);\n\
         \t\tif (!row) return reply.code(404).send({{ error: \"{type_name} not found\" }});\n\
         \t\treturn row;\n\
         \t}});\n"
    )
}

/// Renders the delete route.
///
/// A 204 carries no body, but the Zod provider still types `send`, so `null` is
/// passed explicitly rather than calling it with no argument.
pub(super) fn remove(name: &str, type_name: &str) -> String {
    format!(
        "\tapp.delete(\"/:id\", {{\n\
         \t\tschema: {{\n\
         \t\t\ttags: [\"{type_name}\"],\n\
         \t\t\toperationId: \"delete{type_name}\",\n\
         \t\t\tparams,\n\
         \t\t\tresponse: {{ 204: z.null(), 404: errorSchema }},\n\
         \t\t}},\n\
         \t}}, async (request, reply) => {{\n\
         \t\tconst removed = await {name}Repository.remove(db, request.params.id);\n\
         \t\tif (!removed) return reply.code(404).send({{ error: \"{type_name} not found\" }});\n\
         \t\treturn reply.code(204).send(null);\n\
         \t}});\n"
    )
}
