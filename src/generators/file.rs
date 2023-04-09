use std::fs::File;
use std::io::Error;

/// # Name: create_file
/// # Description: Creates a file
/// # Arguments:
/// * `name` - The name of the file
pub fn create_file(name: &str) -> Result<(), Error> {
    File::create(name)?;
    Ok(())
}