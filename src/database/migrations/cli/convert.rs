mod apply;
mod operation;
mod preview;

use clap::ArgMatches;

pub(super) fn run(matches: &ArgMatches) {
    let remove_source = matches.get_flag("remove-source");
    if matches.get_flag("dry-run") {
        preview::run();
    } else {
        apply::run(remove_source);
    }
}
