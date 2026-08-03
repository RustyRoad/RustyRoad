//! Request body and path parameter rendering.

use super::super::operations::{Operation, Verb};

/// Renders the request body declaration.
pub(super) fn request_body(operation: &Operation) -> String {
    let row = operation.row_type.as_deref().unwrap_or("ApiError");
    let schema = match operation.verb {
        Verb::Patch => format!("Patch{row}"),
        _ => format!("New{row}"),
    };

    format!(
        "        \"requestBody\": {{\n\
         \x20         \"required\": true,\n\
         \x20         \"content\": {{\n\
         \x20           \"application/json\": {{\n\
         \x20             \"schema\": {{ \"$ref\": \"#/components/schemas/{schema}\" }}\n\
         \x20           }}\n\
         \x20         }}\n\
         \x20       }}"
    )
}

/// Renders the path parameter declaration.
///
/// Declaring the type lets the client serialize it and lets Fastify coerce it.
pub(super) fn parameters(key_type: &str) -> String {
    let schema = if key_type == "number" {
        "{ \"type\": \"integer\" }"
    } else {
        "{ \"type\": \"string\" }"
    };

    format!(
        "        \"parameters\": [\n\
         \x20         {{\n\
         \x20           \"name\": \"id\",\n\
         \x20           \"in\": \"path\",\n\
         \x20           \"required\": true,\n\
         \x20           \"schema\": {schema}\n\
         \x20         }}\n\
         \x20       ]"
    )
}
