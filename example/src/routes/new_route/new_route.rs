use rocket_dyn_templates::{context, Template};

#[get("/new_route")]
pub fn index() -> Template {
    Template::render(
        "pages/new_route",
        context! {
            route_name: "new_route",
        },
    )
}