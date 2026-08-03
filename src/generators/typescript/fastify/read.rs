//! Read-only route rendering: collection and single row.

/// Renders the collection route.
pub(super) fn list(name: &str, type_name: &str) -> String {
    format!(
        "\tapp.get(\"/\", {{\n\
         \t\tschema: {{\n\
         \t\t\ttags: [\"{type_name}\"],\n\
         \t\t\toperationId: \"list{type_name}\",\n\
         \t\t\tresponse: {{ 200: z.array({name}SelectSchema) }},\n\
         \t\t}},\n\
         \t}}, async () => {name}Repository.list(db));\n"
    )
}

/// Renders the single-row route.
pub(super) fn find(name: &str, type_name: &str) -> String {
    format!(
        "\tapp.get(\"/:id\", {{\n\
         \t\tschema: {{\n\
         \t\t\ttags: [\"{type_name}\"],\n\
         \t\t\toperationId: \"get{type_name}\",\n\
         \t\t\tparams,\n\
         \t\t\tresponse: {{ 200: {name}SelectSchema, 404: errorSchema }},\n\
         \t\t}},\n\
         \t}}, async (request, reply) => {{\n\
         \t\tconst row = await {name}Repository.find(db, request.params.id);\n\
         \t\tif (!row) return reply.code(404).send({{ error: \"{type_name} not found\" }});\n\
         \t\treturn row;\n\
         \t}});\n"
    )
}
