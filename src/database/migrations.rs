use crate::{
    generators::create_file,
    writers::{write_to_file, write_to_module},
};

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

    // create directory with timestamp and name of migration
    // then create up and down files

    let folder_name = format!(
        "src/database/migrations/{}_{}",
        chrono::Local::now().format("%Y%m%d%H%M%S"),
        name
    );

    match std::fs::create_dir(&folder_name) {
        Ok(_) => {}
        Err(_) => {
            println!("Migration already exists");
            return Ok(());
        }
    }

    create_file(&format!("{}/up.sql", folder_name).to_string())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    let file = format!("{}/up.sql", folder_name).to_string();

    let contents = r#"
      CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
  );
  "#
    .to_string();

    write_to_file(&file, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    // create the down file
    create_file(&format!("{}/down.sql", folder_name).to_string())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    let file = format!("{}/down.sql", folder_name).to_string();

    let contents = r#"
  DROP TABLE IF EXISTS users;
  "#
    .to_string();

    write_to_file(&file, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

    Ok(())
}
