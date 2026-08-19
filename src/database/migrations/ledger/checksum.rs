//! SHA-256 checksums for migration source files.

use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::Path;

/// Returns the lowercase SHA-256 digest of a migration file's exact bytes.
pub fn file_checksum(path: &Path) -> Result<String, io::Error> {
    let bytes = fs::read(path)?;
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest.iter() {
        let _ = write!(hex, "{byte:02x}");
    }
    Ok(hex)
}

#[cfg(test)]
mod tests {
    use super::file_checksum;

    #[test]
    fn checksum_uses_exact_file_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("up.sql");
        std::fs::write(&path, b"SELECT 1;\n").unwrap();

        assert_eq!(
            file_checksum(&path).unwrap(),
            "b4e0497804e46e0a0b0b8c31975b062152d551bac49c3c2e80932567b4085dcd"
        );
    }
}
