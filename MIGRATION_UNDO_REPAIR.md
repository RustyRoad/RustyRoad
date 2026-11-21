# Repair missing migration undo files

`rustyroad migration repair-down` is an offline, missing-file-only operation.
It does not use migration conversion, connect to a database, execute SQL, or
modify migration history. Existing `up.sql` and `down.sql` files are not rewritten.

## Preview and write

```bash
# Preview the canonical migration directory; no files are written.
rustyroad migration repair-down

# An explicit preview is equivalent; --dry-run conflicts with --write.
rustyroad migration repair-down --dry-run

# Preview one migration rather than the entire directory.
rustyroad migration repair-down --path config/database/migrations/20260101000000-example

# After reviewing the displayed SQL, create missing undo files only.
rustyroad migration repair-down --write
```

The default path is `./config/database/migrations`. Discovery is nonrecursive
and sorted. A directory containing `up.sql` is treated as a single migration;
otherwise its immediate subdirectories are considered migrations.

The inferred subset is deliberately small: unconditional ordinary `CREATE
TABLE`, explicitly named nonconcurrent `CREATE INDEX`, and unconditional
`ALTER TABLE ... ADD COLUMN`. Statements and column additions are reversed in
reverse order. A plain outer `BEGIN`/`COMMIT` pair is retained. Parsing must
succeed for the entire source and the inferred undo; no partial undo is emitted.

Conditional creation, destructive changes, data updates, procedural SQL,
inherited/temporary tables, concurrent indexes, and other unsupported operations
require reviewed undo.
`IF NOT EXISTS` is not proof that a migration created an object; automatically
dropping that object could remove something that predates the migration.

## Supply a reviewed undo

For an unsupported migration, prepare an independent review directory:

```text
reviewed-undo/
  20260101000000-example/
    up.sql    # exact byte-for-byte copy of the existing migration
    down.sql  # explicitly reviewed, nonempty undo SQL
```

```bash
rustyroad migration repair-down --reviewed-down-dir reviewed-undo
rustyroad migration repair-down --reviewed-down-dir reviewed-undo --write
```

The copied `up.sql` must match the original exactly, including whitespace.
A review directory takes precedence over inference for that migration; stale
or incomplete review files block repair. Reviewed SQL is a human assertion,
not a proof of correctness. It is checked for UTF-8, NUL bytes, tokenizable SQL,
and content beyond comments or separators. It need not parse as statements
(to permit PostgreSQL procedural SQL).

## Safety and results

- `Existing`: a `down.sql` entry already exists and is untouched, not validated.
- `Ready`: undo is planned but has not been written.
- `Written`: a missing `down.sql` was created without replacing another entry.
- `Blocked`: a source, review, inference, or write error needs attention.

Any planning blocker prevents **all** writes; errors return exit status 2.
Writes stage and sync a temporary file in the migration directory, recheck
the source bytes, and publish with no-clobber semantics. Existing entries,
including dangling `down.sql` symlinks, are never replaced. Symlink migration
directories and nonregular source/review files are rejected.

Publication is per file, not atomic across directories. A later write failure
leaves earlier `Written` files in place and stops further writes. Rerunning
skips those files. Keep migration/review directories stable during repair;
path checks are not protection against hostile concurrent directory changes.

Review inferred SQL before use: undoing schema creation can destroy data.
This command cannot restore historical data or prove catalog, inheritance,
trigger, or application compatibility. Inspect the generated undo before applying it.

## Validation scope

Implementation review is static/local only. Builds, tests, CLI execution,
and repair against a project migration directory are not-run under the current
execution restrictions. The commands above are usage examples, not execution
evidence. Runtime spatial assignment and transactional cursor reservation are
separate application concerns and are not implemented by this command.