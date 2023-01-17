use std::fs::create_dir;
use std::path::Path;

/// # Name: create_directory
/// # Description: Creates a directory
/// # Arguments:
/// * `name` - The name of the directory
pub fn create_directory(name: &String) -> Result<(), std::io::Error> {
    create_dir(&name).unwrap_or_else(|why| {
        println!("!{:?} {:?}", &name, why.kind());
    });
    Ok(())
}
