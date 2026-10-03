//! The handler functions, one per verb.

mod keyed;

use super::paths;
use crate::generators::rust::model::Model;

pub(super) use keyed::{find, remove, update};

/// Renders the listing handler.
pub(super) fn list(model: &Model) -> String {
    format!(
        "{}\
         pub async fn list() -> impl Responder {{\n\
         \x20   match {name}::all().await {{\n\
         \x20       Ok(rows) => HttpResponse::Ok().json(rows),\n\
         \x20       Err(error) => super::super::api::server_error(error),\n\
         \x20   }}\n\
         }}\n\n",
        paths::list(model),
        name = model.name
    )
}

/// Renders the insert handler.
///
/// A table whose every column is database-assigned has a `create` taking no argument, so the
/// handler must not extract a body it cannot pass on.
pub(super) fn create(model: &Model) -> String {
    if model.insertable().is_empty() {
        return format!(
            "{}\
             pub async fn create() -> impl Responder {{\n\
             \x20   match {name}::create().await {{\n\
             \x20       Ok(row) => HttpResponse::Created().json(row),\n\
             \x20       Err(error) => super::super::api::server_error(error),\n\
             \x20   }}\n\
             }}\n\n",
            paths::create_empty(model),
            name = model.name
        );
    }

    format!(
        "{}\
         pub async fn create(row: web::Json<{name}>) -> impl Responder {{\n\
         \x20   match {name}::create(row.into_inner()).await {{\n\
         \x20       Ok(row) => HttpResponse::Created().json(row),\n\
         \x20       Err(error) => super::super::api::server_error(error),\n\
         \x20   }}\n\
         }}\n\n",
        paths::create(model),
        name = model.name
    )
}

/// Returns the borrow a keyed model method needs.
///
/// `find(&str)` takes a borrow while `find(i32)` takes the value, so an owned `String` from
/// the path has to be re-borrowed for the former.
fn borrow(rust: &str) -> &'static str {
    match rust {
        "String" => "&",
        _ => "",
    }
}
