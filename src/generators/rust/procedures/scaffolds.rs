//! Developer-owned API and module scaffolds, written once.

/// Render the developer-owned API composition point.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn composition() -> String {
    String::from(
        "// Written once by `rustyroad pull --language rust`, then yours.\n\
         // Generated procedures are refreshed in procedures.rs; custom routes belong here.\n\n\
         use actix_web::web;\n\n\
         use super::procedures;\n\n\
         /// Registers generated procedures plus your hand-written services.\n\
         pub fn configure(cfg: &mut web::ServiceConfig) {\n\
             procedures::configure_generated(cfg);\n\
             // Add hand-written services here: cfg.service(my_service);\n\
         }\n",
    )
}

/// Render the developer-owned module facade.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn module() -> String {
    String::from(
        "// Written once by `rustyroad pull --language rust`, then yours.\n\n\
         pub mod api;\n\
         pub mod models;\n\
         pub mod procedures;\n\
         pub mod repositories;\n\n\
         pub use api::configure;\n",
    )
}
