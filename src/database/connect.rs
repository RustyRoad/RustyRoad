// use super::Database;

// // ## Name: connect
// // ## Description: Connects to the database
// // ## Parameters:
// // - database: [`&Database`] - the database struct
// // ## Returns:
// // - if the connection was successful: [`Ok(Pool<ConnectionManager<PgConnection>>)`]
// // - if there was an error connecting to the database: [`Err(Error)`]
// // ## Example:
// // ```rust
// // use rustyroad::database::connect;
// // use rustyroad::database::Database;
// //  // read the config file
// //  let config = Config::from_file("rustyroad.toml").unwrap();
// //  // get the database struct from the config file
// //  let database = config.database.unwrap();
// //  // connect to the database
// //  let pool = connect(&database).unwrap();
// // // ```
// // pub fn connect(database: &Database) -> Result<Pool<ConnectionManager<PgConnection>>, Error> {
// //    // read the config file
// //     let config = Config::from_file("rustyroad.toml").unwrap();

// //     // get the database struct from the config file
// //     let database: Database = config.database.unwrap();

// //     // connect to the database
// //     let pool = connect(&database).unwrap();

// //     // return the pool
// //     Ok(pool)
// // }