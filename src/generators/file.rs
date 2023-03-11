use std::fs::File;

/// # Name: create_file
/// # Description: Creates a file
/// # Arguments:
/// * `name` - The name of the file
pub fn create_file(name: &String) -> Result<(), std::io::Error> {
    File::create(&name)
        .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));
    Ok(())
}
