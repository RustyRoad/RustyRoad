//! The parent `api` module: one `configure`, one `ApiDoc`, one error helper.

use crate::generators::layout::Module;

/// The header, including how an application is expected to mount the routes.
const HEADER: &str = "//! The generated HTTP surface.\n\
     //!\n\
     //! Written once by `rustyroad pull --actix`, then yours. Call `configure` inside\n\
     //! whatever scope and middleware your application already uses:\n\
     //!\n\
     //! ```ignore\n\
     //! App::new().service(\n\
     //!     web::scope(\"/api/db\")\n\
     //!         .wrap(YourAuthGuard)\n\
     //!         .configure(models_db::api::configure),\n\
     //! )\n\
     //! ```\n\n\
     use actix_web::{web, HttpResponse};\n\n";

/// The shared error responder.
///
/// One place to decide what a failed query looks like on the wire, and the message is logged
/// rather than returned: a `sqlx` error can name columns, constraints, and SQL fragments.
const SERVER_ERROR: &str = "/// Logs a query failure and returns an opaque 500.\n\
     ///\n\
     /// The error is deliberately not sent to the client: a `sqlx` message can name columns,\n\
     /// constraints, and SQL text, which is more than a caller needs to know.\n\
     pub fn server_error(error: sqlx::Error) -> HttpResponse {\n\
     \x20   log::error!(\"query failed: {error}\");\n\
     \x20   HttpResponse::InternalServerError().finish()\n\
     }\n\n";

/// Renders the parent `api` module.
pub fn parent(modules: &[Module]) -> String {
    format!(
        "{HEADER}{SERVER_ERROR}{}{}",
        aggregate(modules),
        doc(modules)
    )
}

/// Renders the aggregate `configure`.
fn aggregate(modules: &[Module]) -> String {
    let calls = modules
        .iter()
        .map(|module| format!("\x20   super::{}::routes::configure(cfg);\n", module.name))
        .collect::<String>();

    format!(
        "/// Registers every generated model's routes.\n\
         pub fn configure(cfg: &mut web::ServiceConfig) {{\n{calls}}}\n\n"
    )
}

/// Renders the `OpenApi` document collecting every annotated handler.
fn doc(modules: &[Module]) -> String {
    let paths = modules
        .iter()
        .map(|module| format!("\x20       super::{}::routes::list,\n", module.name))
        .collect::<String>();

    format!(
        "/// The OpenAPI document for the generated routes.\n\
         ///\n\
         /// Only the listings are collected: utoipa's `paths(...)` needs each handler named,\n\
         /// and a keyed route's presence depends on the table having a single-column key.\n\
         /// Add the others per model as you expose them.\n\
         #[derive(utoipa::OpenApi)]\n\
         #[openapi(\n\
         \x20   paths(\n{paths}\x20   ),\n\
         )]\n\
         pub struct ApiDoc;\n"
    )
}
