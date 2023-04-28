use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/test")]
pub fn index() -> Template {
    Template::render(
        "pages/test",
        context! {
            route_name: "test",
        },
    )
}