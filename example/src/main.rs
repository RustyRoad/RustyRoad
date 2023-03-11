
// mod models;
mod routes;
#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use routes::{new_route, 
    index::index
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,new_route::index])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}