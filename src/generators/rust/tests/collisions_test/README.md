# Rust identifier collision regression

`collisions_test.rs` exercises the public Rust API writer and parses its generated
models, repositories, and procedures. The fixture includes:

- row/create/patch type collisions (`proposal`, `new_proposal`, `patch_proposal`);
- keyword, punctuation and suffix collisions, including `type` / `type_`;
- filtered create/patch fields that must reuse the row's allocation;
- enums colliding with rows, enum variants, and enum-valued primary/unique keys;
- composite-key suffixes colliding with single columns and another table's CRUD;
- generated helper/import names such as `Interval`, `From`, and `ApiError`.

Allocation is deterministic for a given schema regardless of table/column order.
Rows take priority over enums and input types. A collision receives a numeric
suffix, skipping both already allocated names and other objects' preferred names.
SQL strings, SQLx rename attributes, enum labels and serde keys retain their
original database spelling. No application-generated files should be hand-edited.

## Reproduce the generated-crate compile and serde checks

From the repository root:

```sh
RUSTYROAD_COLLISION_FIXTURE_OUT="$PWD/artifacts/rust-identifier-collisions/generated" \
  cargo test --offline --lib generators::rust::tests
CARGO_TARGET_DIR="$PWD/target" cargo test --offline \
  --manifest-path artifacts/rust-identifier-collisions/generated/Cargo.toml
```

The first command writes the actual generator output under `generated/src/db`,
plus the separate compiler harness (`compiler.toml` and `compiler_lib.rs.txt`).
The second compiles all generated modules against real dependencies and checks
serde round trips. `--offline` requires those dependencies in the local Cargo
cache; remove that flag when fetching dependencies is appropriate.

These are fixture-backed, mocked-local checks: no live database is contacted and
they do not establish that an application's schema was regenerated or migrated.
