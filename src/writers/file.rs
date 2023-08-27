use std::io::Write;

/// # Name: write_to_file
/// ## Description
/// This function writes to a file
/// ## Arguments
/// * `file` - The path to the file to write to
/// * `message` - The message to write to the file
/// ## Returns
/// * `Ok(())` if the message was successfully written to the file, or an Error if something went wrong
/// ## Example
/// ```
/// use crate::writers::write_to_file;
/// use std::io::Error;
///
/// fn main() -> Result<(), Error> {
///    write_to_file("test.txt", "Hello, world!".as_bytes())?;
///   Ok(())
/// }
/// ```
pub fn write_to_file(file: &str, message: &[u8]) -> Result<(), std::io::Error> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)?;
    file.write_all(message).unwrap();
    Ok(())
}
