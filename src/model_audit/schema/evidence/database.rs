//! Isolate the async catalog reader from callers already using a Tokio runtime.
use super::super::{live, types::Catalog};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

/// Read PostgreSQL catalog facts with a sixty-second deadline and join the worker.
pub(super) fn read(
    root: &Path,
    schema: String,
    config: Option<PathBuf>,
) -> Result<Catalog, String> {
    let root = root.to_owned();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| error.to_string())?;
        runtime.block_on(async {
            tokio::time::timeout(
                Duration::from_secs(60),
                live::read(&root, &schema, config.as_deref()),
            )
            .await
            .map_err(|_| {
                "Read-only PostgreSQL catalog audit timed out after 60 seconds".to_string()
            })?
        })
    })
    .join()
    .map_err(|_| "PostgreSQL catalog reader thread failed")?
}
