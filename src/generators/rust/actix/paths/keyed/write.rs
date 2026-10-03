//! Annotations for the routes that mutate one row by its key.

use super::{parameter, NOT_FOUND};
use crate::generators::rust::model::{Field, Model};

/// Renders the annotation for the update.
pub(in crate::generators::rust::actix) fn update(model: &Model, key: &Field) -> String {
    format!(
        "#[utoipa::path(\n\
         \x20   put,\n\
         \x20   path = \"/{path}/{{{ident}}}\",\n\
         \x20   tag = \"{path}\",\n\
         {param}\
         \x20   request_body = {name},\n\
         \x20   responses(\n\
         \x20       (status = 200, description = \"The row as stored\", body = {name}),\n\
         {NOT_FOUND}\
         \x20       (status = 500, description = \"Update failed\"),\n\
         \x20   ),\n\
         )]\n",
        path = model.table,
        ident = key.ident,
        param = parameter(key),
        name = model.name
    )
}

/// Renders the annotation for the delete.
///
/// No `body` on the success response: a 204 carries no content, so naming the model would
/// document a payload the handler never sends.
pub(in crate::generators::rust::actix) fn remove(model: &Model, key: &Field) -> String {
    format!(
        "#[utoipa::path(\n\
         \x20   delete,\n\
         \x20   path = \"/{path}/{{{ident}}}\",\n\
         \x20   tag = \"{path}\",\n\
         {param}\
         \x20   responses(\n\
         \x20       (status = 204, description = \"The row was removed\"),\n\
         {NOT_FOUND}\
         \x20       (status = 500, description = \"Delete failed\"),\n\
         \x20   ),\n\
         )]\n",
        path = model.table,
        ident = key.ident,
        param = parameter(key)
    )
}
