use crate::writers::write_to_file;
use crate::Project;
use std::io::Error;
use std::process::Command;
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
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}"#;
    write_to_file(&project.main_rs, contents.as_bytes())
}
// need to create route module

pub fn add_new_route_to_main_rs(route_name: &String) -> Result<(), Error> {
    // run shell command to add new route to main.rs

    let command = format!(
        "sed -i 's/routes::{{/routes::{{{}, /' src/main.rs",
        route_name
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("error: {}", stderr);
    }

    let command = format!(
        r#"sed -i 's#\.mount("/", routes!\[index#& ,{}::index#' src/main.rs"#,
        route_name
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("error: {}", stderr);
    }

    Ok(())
}
