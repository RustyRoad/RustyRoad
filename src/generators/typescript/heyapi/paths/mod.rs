//! OpenAPI `paths` rendering.

mod request;
mod responses;

use super::operations::Operation;

/// Renders the `paths` object body, grouping operations by URL.
///
/// OpenAPI keys paths by URL with verbs nested beneath, so operations sharing a
/// URL must be merged rather than emitted as duplicate keys.
pub(super) fn render(operations: &[Operation]) -> String {
    let mut urls: Vec<&str> = Vec::new();
    for operation in operations {
        if !urls.contains(&operation.url.as_str()) {
            urls.push(&operation.url);
        }
    }

    urls.iter()
        .map(|url| path(url, operations))
        .collect::<Vec<_>>()
        .join(",\n")
}

/// Renders one path with every verb defined on it.
fn path(url: &str, operations: &[Operation]) -> String {
    let verbs = operations
        .iter()
        .filter(|operation| operation.url == url)
        .map(operation)
        .collect::<Vec<_>>()
        .join(",\n");

    format!("    \"{url}\": {{\n{verbs}\n    }}")
}

/// Renders one operation under its verb.
fn operation(operation: &Operation) -> String {
    let mut sections = vec![
        format!("        \"operationId\": \"{}\"", operation.name),
        format!("        \"tags\": [\"{}\"]", operation.tag),
    ];

    if let Some(key_type) = operation.path_param {
        sections.push(request::parameters(key_type));
    }
    if operation.verb.has_body() {
        sections.push(request::request_body(operation));
    }
    sections.push(responses::responses(operation));

    format!(
        "      \"{}\": {{\n{}\n      }}",
        operation.verb.method(),
        sections.join(",\n")
    )
}
