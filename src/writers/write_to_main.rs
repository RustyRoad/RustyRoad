use crate::writers::write_to_file;
use crate::Project;
use std::fs::OpenOptions;
use std::io::{Error, Write};

// Write to main.rs
pub fn write_to_main_rs(project: &Project) -> Result<(), Error> {
    let contents = r#"
// mod models;
mod routes;
#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use routes::{
    index::index
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(\"/\", routes![index])
        .mount(\"/\", FileServer::from(relative!(\"static\")))
        .attach(Template::fairing())
}"#;
    write_to_file(&project.main_rs, contents.as_bytes())
}
