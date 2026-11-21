//! Conservative PostgreSQL schema inverses; unsupported statements block the file.

mod alter;
mod create;
mod names;

use sqlparser::{ast::Statement, dialect::PostgreSqlDialect, parser::Parser};

pub(super) fn infer(source: &str) -> Result<String, String> {
    if source.contains('\0') {
        return Err("up.sql contains a NUL byte".into());
    }
    let statements = Parser::parse_sql(&PostgreSqlDialect {}, source)
        .map_err(|e| format!("Cannot parse complete PostgreSQL migration: {e}"))?;
    let wrapped = plain_transaction(&statements);
    let body = if wrapped {
        &statements[1..statements.len() - 1]
    } else {
        statements.as_slice()
    };
    if body.is_empty() {
        return Err("Migration contains no reversible schema statements".into());
    }
    let mut undo = Vec::new();
    for (index, statement) in body.iter().enumerate().rev() {
        let reversed = match statement {
            Statement::CreateTable(table) => create::table(table),
            Statement::CreateIndex(index) => create::index(index),
            Statement::AlterTable(table) => alter::table(table),
            _ => Err("Unsupported statement; prior state or side effects need review".into()),
        };
        undo.extend(
            reversed.map_err(|e: String| {
                format!("Statement {}: {e}", index + 1 + usize::from(wrapped))
            })?,
        );
    }
    if undo.is_empty() || undo.iter().any(|statement| statement.trim().is_empty()) {
        return Err("Migration did not produce a complete, nonempty undo".into());
    }
    let mut sql = String::from(
        "-- Inferred schema undo. Review before use; dropping objects can remove data.\n",
    );
    if wrapped {
        sql.push_str("BEGIN;\n");
    }
    sql.push_str(&undo.join("\n"));
    sql.push('\n');
    if wrapped {
        sql.push_str("COMMIT;\n");
    }
    Parser::parse_sql(&PostgreSqlDialect {}, &sql)
        .map_err(|e| format!("Generated undo could not be parsed: {e}"))?;
    Ok(sql)
}

fn plain_transaction(statements: &[Statement]) -> bool {
    matches!(statements.first(), Some(Statement::StartTransaction {
        modes, modifier: None, statements, exception: None,
        has_end_keyword: false, ..
    }) if modes.is_empty() && statements.is_empty())
        && matches!(
            statements.last(),
            Some(Statement::Commit {
                chain: false,
                end: false,
                modifier: None,
            })
        )
        && statements.len() >= 2
}

pub(super) fn conditional() -> String {
    "Conditional DDL does not establish object ownership; supply reviewed undo".into()
}
