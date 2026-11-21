# Rust primary-key conflict operations

Rust pulls include atomic conflict operations for primary keys when PostgreSQL
confirms that the key is immediate, valid and ready, and every key column can be
supplied by the caller. Identity/serial keys are excluded because their values
are not present in the generated create input. Deferrable keys and views are
excluded. Composite key order remains the catalog order.

This augments only Rust generation. TypeScript introspection keeps its existing
primary-key and index representation.

```sh
rustyroad pull --language rust --out ./src/db
```

The functions use the same naming scheme as other verified conflict targets:

- `upsert_<table>_patch_by_<key>` inserts from the generated create input and
  updates only the fields supplied in the independent generated patch.
- `upsert_<table>_patch_touch_by_<key>` additionally sets the transaction-time
  value of `updated_at` on conflict when the patch omits that field. It is
  emitted only for an eligible timestamp column outside the conflict key.
- `upsert_<table>_snapshot_by_<key>` retains the existing snapshot policy.

An explicit patch timestamp takes precedence, including an explicit null when
the schema permits it. An empty patch remains a conflict no-op and returns
`sqlx::Error::RowNotFound`; selecting the timestamp-touch variant does not create
an update by itself. Insert-side defaults remain database-owned.

These operations do not authorize schema changes or infer application-level
uniqueness. They are generated only from the observed catalog constraints.
