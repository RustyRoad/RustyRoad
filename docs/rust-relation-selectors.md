# Foreign-key collection operations

Rust repository generation can emit `list_<table>_by_<columns>` and
`delete_<table>_by_<columns>` for declared foreign keys that are not already
primary-key or generated unique-key targets. Both operations use bound equality
predicates derived from the catalog's local column list.

The list returns all matching generated rows and makes no ordering guarantee.
The delete issues one DELETE statement and returns its affected-row count; it
does not perform a preliminary read or run a per-row deletion loop. Composite
foreign-key argument order follows the catalog, even when identity comparison
uses sorted column sets to avoid duplicates.

Views, empty/invalid keys and duplicate column identities produce no operations.
A foreign key never authorizes an upsert or a uniqueness assumption. Global
function-name allocation handles collisions with other tables and selectors.

Read `rustyroad pull --help` before generation. The migration task currently has
source changes only: no native build or regeneration has been performed, so
applications must not call these functions until the generated output contains
them for the intended table and key.
