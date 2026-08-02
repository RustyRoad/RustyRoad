//! Tests for multi-statement detection.

use super::statement::is_multi_statement;

#[test]
fn single_statement_is_not_multi() {
    assert!(!is_multi_statement("SELECT 1"));
    assert!(!is_multi_statement("SELECT 1;"));
    assert!(!is_multi_statement("  SELECT 1 ;  "));
}

#[test]
fn multiple_statements_are_detected() {
    assert!(is_multi_statement("SELECT 1; SELECT 2"));
    assert!(is_multi_statement("CREATE TABLE t (id int); INSERT INTO t VALUES (1);"));
}

#[test]
fn semicolons_in_string_literals_are_ignored() {
    // A single statement whose literal contains a semicolon must not be split.
    assert!(!is_multi_statement("SELECT 'a;b'"));
    assert!(!is_multi_statement("INSERT INTO t (c) VALUES ('x; y');"));
    // Doubled quotes are an escaped quote, not a literal boundary.
    assert!(!is_multi_statement("SELECT 'it''s; fine'"));
}

#[test]
fn semicolons_in_quoted_identifiers_are_ignored() {
    assert!(!is_multi_statement("SELECT \"odd;name\" FROM t"));
}

#[test]
fn semicolons_in_comments_are_ignored() {
    assert!(!is_multi_statement("SELECT 1 -- trailing; comment"));
    assert!(!is_multi_statement("SELECT 1 /* inline; comment */"));
    assert!(!is_multi_statement("-- leading; comment\nSELECT 1"));
}

#[test]
fn empty_and_trailing_separators_do_not_count() {
    assert!(!is_multi_statement(""));
    assert!(!is_multi_statement(";"));
    assert!(!is_multi_statement("SELECT 1;;"));
    assert!(!is_multi_statement("SELECT 1;   ;  "));
}
