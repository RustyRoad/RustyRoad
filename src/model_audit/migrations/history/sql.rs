//! Inspect statement boundaries without confusing migration data with executable DDL.

mod statements;

use crate::database::migrations::SqlOperation;
use sqlparser::{
    dialect::PostgreSqlDialect,
    tokenizer::{Token, Tokenizer},
};
use std::{cell::RefCell, collections::BTreeMap};

thread_local! {
    /// Cache exact SQL content, never paths or metadata; callers still reread files.
    static CACHE: RefCell<Cache> = RefCell::new(Cache::default());
}

#[derive(Default)]
struct Cache {
    bytes: usize,
    operations: BTreeMap<String, Vec<SqlOperation>>,
}

/// Tokenize first so semicolons and DDL words inside strings remain inert.
/// Lexical failures block inspection rather than silently skipping history.
pub(super) fn operations(sql: &str) -> Result<Vec<SqlOperation>, String> {
    if let Some(operations) = CACHE.with(|cache| cache.borrow().operations.get(sql).cloned()) {
        return Ok(operations);
    }
    let operations = statements::parse(sql)?;
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.bytes + sql.len() > 8 * 1024 * 1024 {
            *cache = Cache::default();
        }
        if sql.len() <= 8 * 1024 * 1024 {
            cache.bytes += sql.len();
            cache.operations.insert(sql.to_owned(), operations.clone());
        }
    });
    Ok(operations)
}

/// Match parsed SQL identifiers to catalog names without folding quoted case.
pub(super) fn same_column(sql: &str, catalog: &str) -> bool {
    let sql = sql.trim();
    if let Some(quoted) = sql
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    {
        quoted.replace("\"\"", "\"") == catalog
    } else {
        sql.to_lowercase() == catalog
    }
}

/// Unqualified history can use another search path, so flag its table name.
pub(super) fn same_table(name: &str, schema: &str, table: &str) -> bool {
    let Ok(tokens) = Tokenizer::new(&PostgreSqlDialect {}, name).tokenize() else {
        return false;
    };
    let tokens: Vec<_> = tokens
        .into_iter()
        .filter(|token| !matches!(token, Token::Whitespace(_)))
        .collect();
    match tokens.as_slice() {
        [Token::Word(name)] => same_column(&name.to_string(), table),
        [Token::Word(namespace), Token::Period, Token::Word(name)] => {
            same_column(&namespace.to_string(), schema) && same_column(&name.to_string(), table)
        }
        _ => false,
    }
}
