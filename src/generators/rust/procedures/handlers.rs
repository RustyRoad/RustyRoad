//! Actix handlers using the same scoped key codec as repositories.
use super::ROUTE_PREFIX;
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{allocation::Names, types};

/// Emit CRUD handlers without losing the primary column's owning table.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, schema: &Schema, names: &Names) -> String {
    let list = names.function(table, "list");
    let get = names.function(table, "get");
    let find = names.function(table, "find");
    let create = names.function(table, "create");
    let update = names.function(table, "update");
    let delete = names.function(table, "delete");
    let model = names.row(table);
    let new_model = names.create(table);
    let patch_model = names.patch(table);
    let key = table
        .column(&table.primary_key[0])
        .expect("introspected primary key should reference a column");
    let key_type = types::map_column(table, key, schema, names).rust;

    format!(
        "/// `GET {prefix}/{table_name}`\n\
         async fn {list}(pool: web::Data<PgPool>) -> Result<web::Json<Vec<{model}>>, ApiError> {{\n\
             Ok(web::Json(repositories::{list}(pool.get_ref()).await?))\n\
         }}\n\n\
         /// `GET {prefix}/{table_name}/{{id}}`\n\
         async fn {get}(pool: web::Data<PgPool>, id: web::Path<{key_type}>) -> Result<web::Json<{model}>, ApiError> {{\n\
             repositories::{find}(pool.get_ref(), id.into_inner())\n\
                 .await?\n\
                 .map(web::Json)\n\
                 .ok_or(ApiError::NotFound({entity:?}))\n\
         }}\n\n\
         /// `POST {prefix}/{table_name}`\n\
         async fn {create}(pool: web::Data<PgPool>, input: web::Json<{new_model}>) -> Result<(web::Json<{model}>, StatusCode), ApiError> {{\n\
             let row = repositories::{create}(pool.get_ref(), input.into_inner()).await?;\n\
             Ok((web::Json(row), StatusCode::CREATED))\n\
         }}\n\n\
         /// `PATCH {prefix}/{table_name}/{{id}}`\n\
         async fn {update}(pool: web::Data<PgPool>, id: web::Path<{key_type}>, input: web::Json<{patch_model}>) -> Result<web::Json<{model}>, ApiError> {{\n\
             repositories::{update}(pool.get_ref(), id.into_inner(), input.into_inner())\n\
                 .await?\n\
                 .map(web::Json)\n\
                 .ok_or(ApiError::NotFound({entity:?}))\n\
         }}\n\n\
         /// `DELETE {prefix}/{table_name}/{{id}}`\n\
         async fn {delete}(pool: web::Data<PgPool>, id: web::Path<{key_type}>) -> Result<HttpResponse, ApiError> {{\n\
             if repositories::{delete}(pool.get_ref(), id.into_inner()).await? {{\n\
                 Ok(HttpResponse::NoContent().finish())\n\
             }} else {{\n\
                 Err(ApiError::NotFound({entity:?}))\n\
             }}\n\
         }}\n\n",
        prefix = ROUTE_PREFIX,
        table_name = table.name,
        entity = model,
    )
}
