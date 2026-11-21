//! Check every pending source before the first write, then publish without replacement.

use super::{storage, Entry, Pending, Status};

pub(super) fn write(entries: &mut [Entry], pending: &[Pending]) {
    for item in pending {
        let entry = &mut entries[item.entry];
        if let Err(error) = storage::preflight(&entry.directory, &item.source) {
            entry.status = Status::Blocked;
            entry.detail = error;
            return;
        }
    }
    for item in pending {
        let entry = &mut entries[item.entry];
        let result = entry
            .sql
            .as_deref()
            .ok_or_else(|| "Planned undo SQL is missing; nothing published".to_string())
            .and_then(|sql| storage::publish(&entry.directory, &item.source, sql));
        match result {
            Ok(()) => {
                entry.status = Status::Written;
                entry.detail = "Created missing down.sql; existing files were not replaced".into();
            }
            Err(error) => {
                entry.status = Status::Blocked;
                entry.detail = error;
                break;
            }
        }
    }
}
