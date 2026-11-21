# Inferred String storage during model audits

An unannotated Rust `String` states that a value uses the text codec; it does not
request removal of an existing PostgreSQL VARCHAR length limit. The audit keeps
an observed `varchar`, `character varying`, or positively bounded VARCHAR type
when the resolved field is a builtin String (optionally wrapped in Option).

The desired column retains the exact observed SQL spelling. Nullability is still
compared independently, so this rule cannot hide an Option/non-Option mismatch.
New columns continue to infer TEXT because no catalog storage exists to retain.

Explicit configured or CLI type mappings take priority. Source-defined SQLx
storage contracts also take priority. The rule does not generalize to CHAR,
domains, arrays, unknown custom codecs, numeric precision, or other type families.

To inspect a real source without connecting to PostgreSQL, pass `--offline` and
an existing generated models file or complete schema snapshot. Without
`--offline`, configured catalog discovery can connect even for `--report-only`.
Always read `rustyroad model audit --help` for installed command behavior.

This change is source-prepared and unbuilt in the current migration task. The
installed executable retains its previous behavior until the final approved
build; do not treat source edits as proof that a live proposal was corrected.
