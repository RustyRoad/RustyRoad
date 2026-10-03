//! Annotations for the routes that address the collection rather than one row.

use crate::generators::rust::model::Model;

/// Renders the annotation for the listing.
pub(in crate::generators::rust::actix) fn list(model: &Model) -> String {
    format!(
        "#[utoipa::path(\n\
         \x20   get,\n\
         \x20   path = \"/{path}\",\n\
         \x20   tag = \"{path}\",\n\
         \x20   responses(\n\
         \x20       (status = 200, description = \"Every row\", body = [{name}]),\n\
         \x20       (status = 500, description = \"Query failed\"),\n\
         \x20   ),\n\
         )]\n",
        path = model.table,
        name = model.name
    )
}

/// Renders the annotation for the insert.
pub(in crate::generators::rust::actix) fn create(model: &Model) -> String {
    format!(
        "#[utoipa::path(\n\
         \x20   post,\n\
         \x20   path = \"/{path}\",\n\
         \x20   tag = \"{path}\",\n\
         \x20   request_body = {name},\n\
         \x20   responses(\n\
         \x20       (status = 201, description = \"The row as stored\", body = {name}),\n\
         \x20       (status = 500, description = \"Insert failed\"),\n\
         \x20   ),\n\
         )]\n",
        path = model.table,
        name = model.name
    )
}

/// Renders the annotation for an insert taking no body.
///
/// Every column is database-assigned, so there is nothing for a caller to send.
pub(in crate::generators::rust::actix) fn create_empty(model: &Model) -> String {
    format!(
        "#[utoipa::path(\n\
         \x20   post,\n\
         \x20   path = \"/{path}\",\n\
         \x20   tag = \"{path}\",\n\
         \x20   responses(\n\
         \x20       (status = 201, description = \"The row as stored\", body = {name}),\n\
         \x20       (status = 500, description = \"Insert failed\"),\n\
         \x20   ),\n\
         )]\n",
        path = model.table,
        name = model.name
    )
}
