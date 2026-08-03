# Declarative migrations

A migration may be authored as `migration.json` instead of `up.sql`/`down.sql`.
Declaring *what* changes lets the runner apply a breaking column change without
breaking clients that are still running the old code.

## Why

A plain `ALTER TABLE ... RENAME COLUMN zone_id TO trash_zone_id` breaks every
reader of `zone_id` the instant it runs. There is no window in which both the old
and new application versions work, so the schema change and the deploy have to be
coordinated exactly.

A declared `alter_column` instead:

1. adds a shadow column `_rustyroad_new_zone_id` beside the original,
2. installs triggers so a write through either schema version populates both,
3. backfills existing rows in batches, and
4. at `complete`, drops the original and promotes the shadow.

Both versions read and write correctly for the whole rollout.

## Usage

Place `migration.json` in the migration directory, then:

```sh
# Publish the new version alongside the current one.
rustyroad migration start 20251114211514-rename_zone_id

# Old clients keep using the previous version's schema.
# New clients opt in:
#   SET search_path TO public_20251114211514-rename_zone_id;

# Once every client is on the new version:
rustyroad migration complete

# Or, if something looks wrong before completing:
rustyroad migration rollback-version
```

`rustyroad migration version-status` reports what is being served and what is in
flight.

## Operations

### `alter_column`

Retype, rename, or change nullability. `up` converts old values to new; `down`
converts new values back so old readers keep working.

```json
{
  "alter_column": {
    "table": "platform_trash_zone_mappings",
    "column": "zone_id",
    "name": "trash_zone_id",
    "type": "uuid",
    "nullable": false,
    "up": "NEW.zone_id::text::uuid",
    "down": "NEW.trash_zone_id::text::int4"
  }
}
```

### `add_column`

`up` populates existing rows. `nullable: false` is applied at `complete`, once
the backfill has given every row a value.

```json
{
  "add_column": {
    "table": "users",
    "column": "status",
    "type": "text",
    "nullable": false,
    "up": "'active'"
  }
}
```

### `drop_column`

`down` keeps the column populated for old readers until `complete` drops it.

```json
{
  "drop_column": { "table": "users", "column": "legacy", "down": "'unknown'" }
}
```

### `sql`

Raw SQL, applied verbatim. No shadow column, trigger, or backfill.

```json
{ "sql": { "up": "CREATE INDEX idx_users_status ON users (status)" } }
```

## Constraints

- **Postgres only.** Versioned schemas use Postgres schemas and views. On MySQL
  and SQLite the SQL is applied without version publishing.
- **One migration at a time.** History is a linear chain; a second `start` is
  rejected until the first is completed or rolled back.
- **Backfilled tables need a primary key.** The backfill paginates over it in
  batches rather than locking the whole table.
- `up` and `down` are SQL expressions over `NEW`, evaluated inside a trigger.
