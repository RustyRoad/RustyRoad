use crate::database::migrations::BreakingChangeFinding;
use dialoguer::Confirm;
use std::io::{self, IsTerminal};

pub(super) fn changes(findings: &[BreakingChangeFinding], allowed: bool) -> io::Result<()> {
    if findings.is_empty() || allowed {
        return Ok(());
    }
    if !io::stdin().is_terminal() {
        return Err(blocked_error());
    }
    let confirmed = Confirm::new()
        .with_prompt("Apply these potentially breaking migration changes?")
        .default(false)
        .interact()
        .map_err(io::Error::other)?;
    if confirmed {
        Ok(())
    } else {
        Err(blocked_error())
    }
}

fn blocked_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "Migration blocked because breaking changes were detected. Review the warning and rerun with --allow-breaking only after validating the migration and backing up the target database.",
    )
}
