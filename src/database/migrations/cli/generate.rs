use clap::ArgMatches;

pub(super) async fn run(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap().to_string();
    let columns = matches
        .get_many::<String>("columns")
        .map(|values| values.map(ToString::to_string).collect())
        .unwrap_or_default();
    println!("Generating migration: {name}");
    super::super::create_migration(&name, columns)
        .await
        .expect("Error creating migration");
    super::breaking_change::generated(&name);
}
