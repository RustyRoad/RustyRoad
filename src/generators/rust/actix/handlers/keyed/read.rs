//! The handler that reads one row by its key.

use super::super::{borrow, paths};
use crate::generators::rust::model::{Field, Model};

/// Renders the keyed lookup handler.
///
/// A missing row is a 404 rather than a 500: the request was well-formed and the answer is
/// simply that nothing matches, which is what `find` returning `None` already says.
pub(in crate::generators::rust::actix) fn find(model: &Model, key: &Field) -> String {
    format!(
        "{}\
         pub async fn find(path: web::Path<{ty}>) -> impl Responder {{\n\
         \x20   match {name}::find({deref}path.into_inner()).await {{\n\
         \x20       Ok(Some(row)) => HttpResponse::Ok().json(row),\n\
         \x20       Ok(None) => HttpResponse::NotFound().finish(),\n\
         \x20       Err(error) => super::super::api::server_error(error),\n\
         \x20   }}\n\
         }}\n\n",
        paths::find(model, key),
        ty = paths::owned_key(key.key_type()),
        deref = borrow(key.key_type()),
        name = model.name
    )
}
