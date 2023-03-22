use std::{fs::File, io::Write};

pub fn create_migration(name: &str) -> Result<(), std::io::Error> {
    // check for database folder
    // if it doesn't exist, create it

    match std::fs::create_dir("src/database") {
        Ok(_) => {}
        Err(_) => {}
    }

    // check for migrations folder
    // if it doesn't exist, create it
    match std::fs::create_dir("src/database/migrations") {
        Ok(_) => {}
        Err(_) => {}
    }
   
    let mut file = File::create(format!("src/database/migrations/{}.rs", name)).unwrap();
    file.write_all(format!("src/database/migrations/{}.rs", name).as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    Ok(())
}
