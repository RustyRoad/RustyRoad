//! Annotations for the routes that read one row by its key.

use super::{parameter, NOT_FOUND};
use crate::generators::rust::model::{Field, Model};

/// Renders the annotation for the keyed lookup.
pub(in crate::generators::rust::actix) fn find(model: &Model, key: &Field) -> String {
    format!(
        "#[utoipa::path(\n\
         \x20   get,\n\
         \x20   path = \"/{path}/{{{ident}}}\",\n\
         \x20   tag = \"{path}\",\n\
         {param}\
         \x20   responses(\n\
         \x20       (status = 200, description = \"The row\", body = {name}),\n\
         {NOT_FOUND}\
         \x20       (status = 500, description = \"Query failed\"),\n\
         \x20   ),\n\
         )]\n",
        path = model.table,
        ident = key.ident,
        param = parameter(key),
        name = model.name
    )
}
