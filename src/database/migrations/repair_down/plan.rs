//! Plan missing undo files without modifying migration inputs.

use super::{inventory, reviewed, sql, storage, Entry, Status};
use std::path::{Path, PathBuf};

pub(super) fn entry(directory: PathBuf, reviews: Option<&Path>) -> (Entry, Option<Vec<u8>>) {
    let mut entry = Entry {
        directory,
        status: Status::Blocked,
        detail: String::new(),
        sql: None,
    };
    match prepare(&entry.directory, reviews) {
        Ok(None) => {
            entry.status = Status::Existing;
            entry.detail = "Existing down.sql entry left untouched (not validated)".into();
            (entry, None)
        }
        Ok(Some((sql, source, detail))) => {
            entry.status = Status::Ready;
            entry.detail = detail.into();
            entry.sql = Some(sql);
            (entry, Some(source))
        }
        Err(error) => {
            entry.detail = error;
            (entry, None)
        }
    }
}

type Prepared = Option<(String, Vec<u8>, &'static str)>;

fn prepare(directory: &Path, reviews: Option<&Path>) -> Result<Prepared, String> {
    inventory::directory(directory)?;
    if inventory::exists(&directory.join("down.sql"))? {
        return Ok(None);
    }
    let source = storage::read(&directory.join("up.sql"))?;
    if let Some(root) = reviews {
        if let Some(undo) = reviewed::load(root, directory, &source)? {
            return Ok(Some((
                undo,
                source,
                "Reviewed undo with byte-matching up.sql",
            )));
        }
    }
    let text = std::str::from_utf8(&source).map_err(|e| format!("up.sql is not UTF-8: {e}"))?;
    let undo = sql::infer(text).map_err(|e| {
        format!("Manual rollback required: {e}. Supply reviewed up.sql/down.sql copies")
    })?;
    Ok(Some((
        undo,
        source,
        "Inferred schema undo; review before use (may remove data)",
    )))
}
