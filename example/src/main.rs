
// mod models;
mod routes;
#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use routes::{
    index::{index},
    dashboard::{index as dashboard_route},
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, dashboard_route])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}