use std::fs::File;

/// # Name: create_navbar_component
/// # Description: Creates a navbar component
/// # Arguments:
/// * `name` - The name of the navbar component

pub fn create_navbar_component(name: &String) -> Result<(), std::io::Error> {
    File::create(&name)
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));
    Ok(())
}
