//! Explicit human-reviewed undo, bound to an exact copy of the source migration.

use super::{inventory, storage};
use sqlparser::{
    dialect::PostgreSqlDialect,
    tokenizer::{Token, Tokenizer},
};
use std::path::Path;

pub(super) fn load(root: &Path, directory: &Path, source: &[u8]) -> Result<Option<String>, String> {
    let name = directory
        .file_name()
        .ok_or("Migration directory has no file name")?;
    let review = root.join(name);
    if !inventory::exists(&review)? {
        return Ok(None);
    }
    inventory::directory(&review)?;
    if storage::read(&review.join("up.sql"))? != source {
        return Err(format!(
            "{}: reviewed up.sql does not match source bytes",
            review.display()
        ));
    }
    let undo = String::from_utf8(storage::read(&review.join("down.sql"))?)
        .map_err(|e| format!("Reviewed down.sql is not UTF-8: {e}"))?;
    if undo.trim().is_empty() || undo.contains('\0') {
        return Err("Reviewed down.sql must be nonempty SQL without NUL bytes".into());
    }
    let tokens = Tokenizer::new(&PostgreSqlDialect {}, &undo)
        .tokenize()
        .map_err(|e| format!("Reviewed down.sql could not be tokenized: {e}"))?;
    if !tokens
        .iter()
        .any(|token| !matches!(token, Token::Whitespace(_) | Token::SemiColon | Token::EOF))
    {
        return Err("Reviewed down.sql contains no SQL beyond comments or separators".into());
    }

    // Do not require parser support for explicitly reviewed procedural SQL.
    // This is a human assertion, not a proof of semantic correctness.
    Ok(Some(undo))
}
