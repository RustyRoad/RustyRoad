//! Execute selection, migration-file generation and final evidence emission.
use clap::ArgMatches;

/// Execute the command, preserving JSON on stdout and nonzero incomplete-audit status.
pub(crate) fn run(matches: &ArgMatches, format: &str) {
    let args = matches
        .subcommand_matches("audit")
        .expect("clap requires audit");
    match super::execute::audit(args).and_then(|mut report| {
        let scope = super::confirmation::request(&report, args, format)?;
        super::migrations::generate(&mut report, args, scope);
        super::coverage::reconcile(&mut report);
        super::output::emit(&report, format, args.get_one::<String>("output"))?;
        Ok(report.diagnostics.is_empty())
    }) {
        Ok(true) => (),
        Ok(false) => std::process::exit(2),
        Err(error) => {
            if format == "json" {
                println!(
                    "{}",
                    serde_json::json!({"error": {"kind": "model_audit", "message": error}})
                );
            } else {
                eprintln!("Model audit: {error}");
            }
            std::process::exit(1);
        }
    }
}
