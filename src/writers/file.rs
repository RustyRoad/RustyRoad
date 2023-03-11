use std::io::Write;

pub fn write_to_file(file: &String, message: &[u8]) -> Result<(), std::io::Error> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)?;
    file.write_all(message).unwrap();
    Ok(())
}
