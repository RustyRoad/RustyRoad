use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/dashboard")]
pub fn index() -> Template {
    Template::render(
        "pages/dashboard",
        context! {
            route_name: "dashboard",
        },
    )
}