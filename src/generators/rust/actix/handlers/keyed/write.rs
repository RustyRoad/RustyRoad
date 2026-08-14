//! Handlers that mutate one row by its key.

use super::super::{borrow, paths};
use crate::generators::rust::model::{Field, Model};

/// Renders the update handler.
pub(in crate::generators::rust::actix) fn update(model: &Model, key: &Field) -> String {
    format!(
        "{}\
         pub async fn update(\n\
         \x20   path: web::Path<{ty}>,\n\
         \x20   row: web::Json<{name}>,\n\
         ) -> impl Responder {{\n\
         \x20   match {name}::update({deref}path.into_inner(), row.into_inner()).await {{\n\
         \x20       Ok(row) => HttpResponse::Ok().json(row),\n\
         \x20       Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),\n\
         \x20       Err(error) => super::super::api::server_error(error),\n\
         \x20   }}\n\
         }}\n\n",
        paths::update(model, key),
        ty = paths::owned_key(key.key_type()),
        deref = borrow(key.key_type()),
        name = model.name
    )
}

/// Renders the delete handler.
///
/// Deleting an absent row is a 404, matching the `bool` the model returns: the caller asked for
/// a specific row to be gone and it was never there.
pub(in crate::generators::rust::actix) fn remove(model: &Model, key: &Field) -> String {
    format!(
        "{}\
         pub async fn remove(path: web::Path<{ty}>) -> impl Responder {{\n\
         \x20   match {name}::delete({deref}path.into_inner()).await {{\n\
         \x20       Ok(true) => HttpResponse::NoContent().finish(),\n\
         \x20       Ok(false) => HttpResponse::NotFound().finish(),\n\
         \x20       Err(error) => super::super::api::server_error(error),\n\
         \x20   }}\n\
         }}\n\n",
        paths::remove(model, key),
        ty = paths::owned_key(key.key_type()),
        deref = borrow(key.key_type()),
        name = model.name
    )
}
