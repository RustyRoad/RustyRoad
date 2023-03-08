use crate::writers::write_to_file;
use crate::Project;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
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
    let file = File::open("./src/main.rs").unwrap_or_else(|why| {
        println!("Couldn't open main.rs: {}", why.to_string());
        panic!();
    });

    let mut perms = file.metadata().unwrap().permissions();
    perms.set_readonly(false);
  
    let mut reader = BufReader::new(&file);
      let temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("temp.rs")?;
    let mut writer = BufWriter::new(temp_file);
    // let search_text_1 = r#"use routes::{"#;
    // let insert_text_1 = format!(
    //     r#"
    //     {}::index,"#,
    //     route_name
    // );

    // // Search for the search text in the file and write the insert text underneath it
    // let mut found1 = false;
    // let mut line = String::new();
    // while reader.read_line(&mut line).unwrap_or_else(|why| {
    //     println!("Couldn't read line: {}", why.to_string());
    //     panic!();
    // }) > 0
    // {
    //     if line.contains(search_text_1) {
    //         writer.write(line.as_bytes()).unwrap_or_else(|why| {
    //             println!("Couldn't write to temp file: {}", why.to_string());
    //             panic!();
    //         });
    //         writer
    //             .write(insert_text_1.as_bytes())
    //             .unwrap_or_else(|why| {
    //                 println!("Couldn't write to temp file: {}", why.to_string());
    //                 panic!();
    //             });
    //         writer.write(b"\n").unwrap_or_else(|why| {
    //             println!("Couldn't write to temp file: {}", why.to_string());
    //             panic!();
    //         });
    //         found1 = true;
    //     } else {
    //         writer.write(line.as_bytes()).unwrap_or_else(|why| {
    //             println!("Couldn't write to temp file: {}", why.to_string());
    //             panic!();
    //         });
    //     }
    //     line.clear();
    // }

    // // If the search text was not found, return an error
    // if !found1 {
    //     return Err(std::io::Error::new(
    //         std::io::ErrorKind::NotFound,
    //         "1 Search text not found",
    //     ));
    // }

    // // get the contents of the file from temp.txt
    // let temp_file_content = format!("{}", std::fs::read_to_string("temp.txt").unwrap_or_else(|why| {
    //     println!("Couldn't read temp file: {}", why.to_string());
    //     panic!();
    // }));
  
    // // Write the contents of temp.txt to main.rs
    // write_to_file(&file_name, &temp_file_content.as_bytes()).unwrap_or_else(|why| {
    //     println!("Couldn't write to main.rs: {}", why.to_string());
    //     panic!();
    // });

    // // Rename the temporary file to the original filename
    // rename("temp.txt", "./src/main.rs").unwrap_or_else(|why| {
    //     println!("Couldn't rename temp file: {}", why.to_string());
    //     panic!();
    // });

    let search_text = "use routes::{";
    let insert_text = format!(
        r#".mount("/{}", routes![{}::index])"#,
        route_name, route_name
    );

    // Search for the search text in the file and write the insert text underneath it
    let mut found = false;
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap_or_else(|why| {
        println!("Couldn't read line: {}", why.to_string());
        panic!();
    }) > 0
    {
        if line.contains(search_text) {
            writer.write(line.as_bytes()).unwrap_or_else(|why| {
                println!("Couldn't write to temp file: {}", why.to_string());
                panic!();
            });
            writer.write(insert_text.as_bytes()).unwrap_or_else(|why| {
                println!("Couldn't write to temp file: {}", why.to_string());
                panic!();
            });
            writer.write(b"\n").unwrap_or_else(|why| {
                println!("Couldn't write to temp file: {}", why.to_string());
                panic!();
            });
            found = true;
        } else {
            writer.write(line.as_bytes()).unwrap_or_else(|why| {
                println!("Couldn't write to temp file: {}", why.to_string());
                panic!();
            });
        }
        line.clear();
    }

    // If the search text was not found, return an error
    if !found {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "2 Search text not found",
        ));
    }


    

   
    // Rename the temporary file to the original file name
    // check if unix or windows
 
        std::fs::copy("temp.txt", "./src/main.rs")?;

    Ok(())
}
