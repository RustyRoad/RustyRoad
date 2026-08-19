//! Printing query results as an aligned text table.
//!
//! Shared by every backend arm of the `query` command so the three of them agree
//! on layout instead of each repeating the same header and cell loops.

use sqlx::{Column, Row};

const COLUMN_WIDTH: usize = 15;
const RULE_WIDTH: usize = 50;

/// Prints `rows` as a table, rendering each cell with `display`.
///
/// Returns `false` when there was nothing to print, so the caller can report an
/// empty result without inspecting the rows itself.
pub fn table<R>(rows: &[R], display: impl Fn(&R, &str) -> String) -> bool
where
    R: Row,
{
    let Some(first_row) = rows.first() else {
        return false;
    };

    header(first_row);
    for row in rows {
        cells(row, &display);
    }

    true
}

/// Prints the column names of `row`, followed by a rule.
fn header<R: Row>(row: &R) {
    for (i, column) in row.columns().iter().enumerate() {
        if i > 0 {
            print!(" | ");
        }
        print!("{:<COLUMN_WIDTH$}", column.name());
    }
    println!();
    println!("{:-<RULE_WIDTH$}", "");
}

/// Prints one row's values.
fn cells<R: Row>(row: &R, display: impl Fn(&R, &str) -> String) {
    for (i, column) in row.columns().iter().enumerate() {
        if i > 0 {
            print!(" | ");
        }
        print!("{:<COLUMN_WIDTH$}", display(row, column.name()));
    }
    println!();
}
