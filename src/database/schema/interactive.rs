//! Optional human-facing completion over one live catalog snapshot.
use crate::database::migrations::CustomMigrationError;
use rustyline::{
    completion::{Completer, Pair},
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    history::DefaultHistory,
    validate::Validator,
    CompletionType, Config, Context, Editor, Helper,
};
use std::{
    borrow::Cow,
    io::{self, IsTerminal},
};

pub(super) fn check_terminal(format: &str) -> Result<(), CustomMigrationError> {
    if format == "json" || !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,
            "--interactive requires a terminal and text output. For scripts use 'db tables --prefix PREFIX' and 'db schema TABLE' with --format json.").into());
    }
    Ok(())
}

struct Tables(Vec<String>);

impl Completer for Tables {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        // Replace the whole identifier, including spaces or punctuation.
        if pos != line.len() {
            return Ok((0, Vec::new()));
        }
        Ok((
            0,
            self.0
                .iter()
                .filter(|name| name.starts_with(line))
                .map(|name| Pair {
                    display: name.clone(),
                    replacement: name.clone(),
                })
                .collect(),
        ))
    }
}

impl Hinter for Tables {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, _: &Context<'_>) -> Option<String> {
        if line.is_empty() || pos != line.len() || self.0.iter().any(|name| name == line) {
            return None;
        }
        self.0
            .iter()
            .find_map(|name| name.strip_prefix(line).map(str::to_owned))
    }
}

impl Highlighter for Tables {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(format!("\x1b[2m{hint}\x1b[0m"))
    }
}
impl Validator for Tables {}
impl Helper for Tables {}

pub(super) fn select(names: Vec<String>) -> Result<Option<String>, CustomMigrationError> {
    if names.is_empty() {
        eprintln!("No tables found.");
        return Ok(None);
    }
    // Control characters in identifiers must not be rendered as terminal commands.
    let names: Vec<_> = names
        .into_iter()
        .filter(|name| !name.chars().any(char::is_control))
        .collect();
    if names.is_empty() {
        eprintln!("No terminal-safe table names found. Use 'db tables' with --format json.");
        return Ok(None);
    }
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();
    let mut editor = Editor::<Tables, DefaultHistory>::with_config(config)
        .map_err(|error| io::Error::other(error.to_string()))?;
    editor.set_helper(Some(Tables(names)));
    eprintln!(
        "Type a table prefix; Tab completes/lists matches, Enter inspects, Ctrl-C/Ctrl-D cancels."
    );
    loop {
        match editor.readline("Table> ") {
            Ok(name) => {
                if editor
                    .helper()
                    .is_some_and(|helper| helper.0.contains(&name))
                {
                    return Ok(Some(name));
                }
                eprintln!("Unknown table. Use Tab to complete a table name.");
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => return Ok(None),
            Err(error) => return Err(io::Error::other(error.to_string()).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_hints_and_completions_agree() {
        let helper = Tables(vec!["user events".into(), "users".into(), "zones".into()]);
        let history = DefaultHistory::new();
        let context = Context::new(&history);
        assert_eq!(helper.hint("us", 2, &context).as_deref(), Some("er events"));
        let (start, matches) = helper.complete("us", 2, &context).unwrap();
        assert_eq!(start, 0);
        assert_eq!(
            matches
                .iter()
                .map(|p| p.replacement.as_str())
                .collect::<Vec<_>>(),
            vec!["user events", "users"]
        );
        assert!(helper.hint("users", 5, &context).is_none());
        assert!(helper
            .complete("missing", 7, &context)
            .unwrap()
            .1
            .is_empty());
        assert!(helper.hint("users", 2, &context).is_none());
        assert!(helper.complete("users", 2, &context).unwrap().1.is_empty());
    }

    #[test]
    fn unicode_and_literal_prefixes_are_safe() {
        let helper = Tables(vec!["équipe".into(), "users_%".into()]);
        let history = DefaultHistory::new();
        let context = Context::new(&history);
        assert_eq!(helper.hint("é", 2, &context).as_deref(), Some("quipe"));
        assert_eq!(
            helper.complete("users_", 6, &context).unwrap().1[0].replacement,
            "users_%"
        );
        assert_eq!(helper.complete("", 0, &context).unwrap().1.len(), 2);
    }
}
