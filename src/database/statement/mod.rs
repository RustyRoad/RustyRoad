//! Detects whether a query holds more than one SQL statement.
//!
//! `sqlx::query` uses the prepared-statement protocol, which accepts exactly one
//! statement. Postgres rejects anything else with "cannot insert multiple commands
//! into a prepared statement". Scripts with several statements must therefore be
//! run through the unprepared path instead.

mod skip;

/// Returns `true` when `sql` contains more than one statement.
///
/// Semicolons inside string literals, quoted identifiers, and comments are ignored,
/// so a single statement containing `';'` is not mistaken for a script.
pub fn is_multi_statement(sql: &str) -> bool {
    statement_count(sql) > 1
}

/// Counts the non-empty statements in `sql`.
fn statement_count(sql: &str) -> usize {
    let mut count = 0;
    let mut is_empty = true;
    let mut chars = sql.chars().peekable();

    while let Some(character) = chars.next() {
        match character {
            '\'' | '"' => {
                skip::quoted(&mut chars, character);
                is_empty = false;
            }
            '-' if chars.peek() == Some(&'-') => skip::line_comment(&mut chars),
            '/' if chars.peek() == Some(&'*') => skip::block_comment(&mut chars),
            ';' => {
                count += usize::from(!is_empty);
                is_empty = true;
            }
            c if c.is_whitespace() => {}
            _ => is_empty = false,
        }
    }

    // A trailing statement without a closing semicolon still counts.
    count + usize::from(!is_empty)
}
