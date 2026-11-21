//! Bound inverses to ordinary PostgreSQL identifiers, retaining their quoting.

use sqlparser::ast::{Ident, ObjectName};

pub(super) fn check(name: &ObjectName, max_parts: usize) -> Result<(), String> {
    if name.0.is_empty() || name.0.len() > max_parts {
        return Err("Object qualification requires reviewed undo".into());
    }
    for part in &name.0 {
        let ident = part
            .as_ident()
            .ok_or("Computed object names require reviewed undo")?;
        identifier(ident)?;
    }
    Ok(())
}

pub(super) fn identifier(ident: &Ident) -> Result<(), String> {
    if ident.value.is_empty() || !matches!(ident.quote_style, None | Some('"')) {
        return Err("Non-PostgreSQL object identifiers require reviewed undo".into());
    }
    Ok(())
}
