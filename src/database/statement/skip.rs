//! Skipping over SQL regions where a semicolon is not a statement separator.

use std::iter::Peekable;
use std::str::Chars;

/// Consumes a quoted literal or identifier, honouring doubled closing quotes.
pub(super) fn quoted(chars: &mut Peekable<Chars>, quote: char) {
    while let Some(character) = chars.next() {
        if character == quote {
            if chars.peek() == Some(&quote) {
                chars.next();
                continue;
            }
            return;
        }
    }
}

/// Consumes a `--` comment through end of line.
pub(super) fn line_comment(chars: &mut Peekable<Chars>) {
    for character in chars.by_ref() {
        if character == '\n' {
            return;
        }
    }
}

/// Consumes a `/* ... */` comment.
pub(super) fn block_comment(chars: &mut Peekable<Chars>) {
    chars.next();
    while let Some(character) = chars.next() {
        if character == '*' && chars.peek() == Some(&'/') {
            chars.next();
            return;
        }
    }
}
