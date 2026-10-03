//! The generated loop bodies, kept verbatim.

/// The loop checking that every required column was supplied.
pub(super) const REQUIRED_LOOP: &str = "    for column in required {\n\
     \x20       let value = row[column]\n\
     \x20       if value == nil {\n\
     \x20           return Err(\"{MODEL}.{column} is required\")\n\
     \x20       }\n\
     \x20   }\n";

/// The loop checking that every supplied value has the kind its column decodes to.
///
/// A nil is skipped because absence is the required check's business; checking it here too
/// would reject every legitimately empty nullable column.
pub(super) const KIND_LOOP: &str = "    for column in kinds.keys() {\n\
     \x20       let value = row[column]\n\
     \x20       // A nil is absence, which the required check above already ruled on.\n\
     \x20       if value != nil && type_of(value) != kinds[column] {\n\
     \x20           return Err(\"{MODEL}.{column} must be \" + kinds[column])\n\
     \x20       }\n\
     \x20   }\n\n";
