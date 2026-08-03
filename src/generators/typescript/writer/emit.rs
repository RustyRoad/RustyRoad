//! Writing individual files, honouring ownership.

use super::ownership::{Outcome, Ownership};
use std::path::Path;
use std::{fs, io};

/// Writes one file unless the developer owns it and it already exists.
pub(super) fn write_owned(
    directory: &Path,
    name: &str,
    contents: &str,
    ownership: Ownership,
    force: bool,
) -> io::Result<Outcome> {
    let path = directory.join(name);

    if !ownership.should_write(&path, force) {
        return Ok(Outcome::Preserved(path));
    }

    fs::write(&path, contents)?;
    Ok(Outcome::Written(path))
}
