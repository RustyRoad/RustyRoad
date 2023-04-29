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
    index::{index},
    dashboard::{index as dashboard_route},
    login::{index as login_route},
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, dashboard_route, login_route])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}"#;
    write_to_file(&project.main_rs, contents.as_bytes())
}
// need to create route module

pub fn add_new_route_to_main_rs(route_name: &String) -> Result<(), Error> {
    // Determine the OS and run the appropriate shell command to add a new route to main.rs

    if cfg!(target_os = "windows") {
        let command = format!(
            "(Get-Content src/main.rs) -replace 'routes::{{', 'routes::{{{}, ' | Set-Content src/main.rs",
            route_name
        );
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(&command)
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("error: {}", stderr);
        }

        let command = format!(
            r#"(Get-Content src/main.rs) -replace '(?<=\.mount\("/"\, routes!\[index)', ' ,{0}::index' | Set-Content src/main.rs"#,
            route_name
        );

        let output = Command::new("powershell")
            .arg("-Command")
            .arg(&command)
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("error: {}", stderr);
        }
    } else {
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
    }

    Ok(())
}
