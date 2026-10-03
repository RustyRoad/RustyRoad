//! The driver script exercising the generated model without a database.

/// The driver exercising `new` and `validate`.
///
/// The CRUD functions need a `db` capability, but the shape and validation logic is pure
/// and is where the generator's decisions actually live.
pub(super) const DRIVER: &str = "import \"./product/mod.tether\" as product\n\
     \n\
     fn main() {\n\
     \x20   let seeded = product.new()?\n\
     \x20   println(\"columns: \" + str(product.columns()?.len()))\n\
     \x20   println(\"table: \" + product.TABLE)\n\
     \n\
     \x20   // Seeded rows leave required columns at their zero, which validates.\n\
     \x20   println(\"seeded ok: \" + str(product.validate(seeded)?))\n\
     \n\
     \x20   let mut missing = product.new()?\n\
     \x20   missing[\"name\"] = nil\n\
     \x20   println(\"missing rejected: \" + str(product.validate(missing).is_err()))\n\
     \n\
     \x20   let mut wrong = product.new()?\n\
     \x20   wrong[\"active\"] = \"yes\"\n\
     \x20   println(\"wrong kind rejected: \" + str(product.validate(wrong).is_err()))\n\
     \x20   return Ok(\"done\")\n\
     }\n";

/// What the driver must print for the generated logic to be correct.
pub(super) const EXPECTED: [&str; 5] = [
    "columns: 9",
    "table: products",
    "seeded ok: true",
    "missing rejected: true",
    "wrong kind rejected: true",
];

/// Every generated file, with the module root last.
///
/// The root imports the others, so checking it last also exercises the import graph the
/// generator wrote.
pub(super) const FILES: [&str; 5] = [
    "create.tether",
    "read.tether",
    "update.tether",
    "delete.tether",
    "mod.tether",
];
