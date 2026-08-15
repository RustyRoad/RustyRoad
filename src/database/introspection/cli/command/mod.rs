//! Clap definition for `rustyroad pull`.

mod args;

use super::help;
use clap::Command;

/// Returns the `pull` command.
pub(crate) fn pull() -> Command {
    let command = Command::new("pull")
        .alias("introspect")
        .about("Introspect the database into a TypeScript folder")
        .long_about(help::long_about());

    args::models(args::typescript(command))
}

#[cfg(test)]
mod tests {
    use super::pull;

    #[test]
    fn zod_target_is_accepted() {
        let matches = pull()
            .try_get_matches_from(["pull", "--zod"])
            .expect("--zod should parse");

        assert!(matches.get_flag("zod"));
    }

    #[test]
    fn zod_models_alias_is_accepted() {
        let matches = pull()
            .try_get_matches_from(["pull", "--zod-models"])
            .expect("--zod-models should parse");

        assert!(matches.get_flag("zod"));
    }

    #[test]
    fn zod_output_folder_is_captured() {
        let matches = pull()
            .try_get_matches_from(["pull", "--zod", "--zod-out", "generated"])
            .expect("--zod-out should parse");

        assert_eq!(
            matches.get_one::<String>("zod-out").map(String::as_str),
            Some("generated")
        );
    }
}
