//! Registration of generated CRUD endpoints.
use super::ROUTE_PREFIX;
use crate::database::introspection::Table;
use crate::generators::rust::{allocation::Names, naming::literal};

/// Emit existing route registrations without changing endpoint identities.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(tables: &[&Table], names: &Names) -> String {
    let registrations = tables
        .iter()
        .map(|table| format!("        .configure({})", names.function(table, "configure")))
        .collect::<Vec<_>>()
        .join("\n");
    let table_configurations = tables
        .iter()
        .map(|table| {
            let configure = names.function(table, "configure");
            let list = names.function(table, "list");
            let get = names.function(table, "get");
            let create = names.function(table, "create");
            let update = names.function(table, "update");
            let delete = names.function(table, "delete");
            format!(
                "fn {configure}(cfg: &mut web::ServiceConfig) {{\n\
                     cfg.service(\n\
                         web::resource({collection})\n\
                             .route(web::get().to({list}))\n\
                             .route(web::post().to({create})),\n\
                     )\n\
                     .service(\n\
                         web::resource({member})\n\
                             .route(web::get().to({get}))\n\
                             .route(web::patch().to({update}))\n\
                             .route(web::delete().to({delete})),\n\
                     );\n\
                 }}\n",
                collection = literal(&format!("/{}", table.name)),
                member = literal(&format!("/{}/{{id}}", table.name)),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "/// Registers every generated CRUD procedure below `{prefix}`.\n\
         pub fn configure_generated(cfg: &mut web::ServiceConfig) {{\n\
             cfg.service(web::scope({prefix_literal})\n{registrations});\n\
         }}\n\n\
         {table_configurations}",
        prefix = ROUTE_PREFIX,
        prefix_literal = literal(ROUTE_PREFIX),
    )
}
