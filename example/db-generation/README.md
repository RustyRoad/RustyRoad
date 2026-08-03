# `rustyroad pull` — one schema, one API, one client

Three features that used to be separate concerns now compose into a single
workflow:

1. **Migrations** change the database.
2. **`pull`** reads the changed database and regenerates the API layer.
3. **oRPC** serves that layer over RPC and REST, and describes itself in OpenAPI,
   which generates the browser client.

The point is that step 1 is the *only* place you describe a change. Everything
downstream is derived, so there is no second definition to keep in sync and no
moment where the client believes something the database no longer does.

## The loop

```sh
# 1. change the database
rustyroad migration generate add_phone_to_users phone:string:nullable
rustyroad migration all

# 2. regenerate everything from what the database now is
rustyroad pull

# 3. regenerate the browser client from the API's own description
npx tsx db/openapi.ts db/openapi/openapi.json
cd db/openapi && npx @hey-api/openapi-ts
```

After step 2 the server validates `phone`, the OpenAPI document mentions it, and
`tsc` fails anywhere the new column was handled incorrectly. After step 3 the
frontend has it too. Nothing was hand-edited.

## How a column becomes a client method

```
rustyroad migration          ──► Postgres
                                    │  introspected by pull
                                    ▼
                                schema.ts          Drizzle tables
                                    │  drizzle-zod derives
                                    ▼
                                zod.ts             validation schemas
                                    │  procedures reference
                                    ▼
                                router.ts          oRPC procedures
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        ▼                           ▼                           ▼
   RPC (typed client)       REST (declared path)        openapi.json
                                                              │
                                                              ▼
                                                        Hey API client
```

Each arrow is generated. The only handwritten artifact in that chain is the
migration.

Note that `zod.ts` and `client.ts` *derive* their shapes rather than listing
columns — `createSelectSchema(users)` and `InferSelectModel<typeof users>` both
read the table. So a new column will not appear in those files as text, but it is
present in the types, the validation, and the wire format. Grepping for a column
name is the wrong check; `tsc` is the right one.

## Why this matters in practice

**A breaking change is caught by the compiler, not by users.** Rename a column,
re-run `pull`, and `tsc` fails at every call site that used the old name —
including the frontend, once the client is regenerated. Previously that mismatch
surfaced as a runtime 500.

**Validation cannot drift.** The Zod schemas are derived from the Drizzle tables,
which come from the live database. There is no hand-written validator to forget
to update.

**The client cannot describe an endpoint the server does not serve.** oRPC
generates the OpenAPI document from the procedures' own schemas, so the document
and the server are the same source.

**A migration is reversible before it is visible.** Combined with the versioned
lifecycle, `migration start` publishes a new schema version while the old one is
still served, so `pull` can regenerate against the new shape and be verified
before `migration complete` retires the old version.

## Generated files

| File | Contents |
|---|---|
| `schema.ts` | Drizzle tables, columns, defaults, constraints, indexes |
| `relations.ts` | `one`/`many` relations derived from foreign keys |
| `zod.ts` | Zod schemas derived from the tables via `drizzle-zod` |
| `client.ts` | A typed repository per table |
| `router.ts` | oRPC procedures over those repositories |
| `server.ts` | Fastify adapter serving the router over RPC and REST |
| `openapi.ts` | Script writing the OpenAPI document from the router |
| `openapi/openapi.json` | A static document, so a client can be built offline |
| `openapi/openapi-ts.config.ts` | Hey API config, preset to the flat SDK style |

```sh
rustyroad pull                              # -> ./db
rustyroad pull --out ./src/db --casing preserve
rustyroad pull --schema-only                # schema.ts + relations.ts only
```

## Dependencies

```sh
npm install drizzle-orm pg zod drizzle-zod fastify \
            @orpc/server @orpc/openapi @orpc/zod @orpc/client
npm install -D drizzle-kit typescript@5 tsx
```

## Serving it

`mountRouter` attaches both handlers. Requests under `<prefix>/rpc` are handled
as RPC; everything else is routed by the paths the procedures declare.

```ts
import Fastify from "fastify";
import { createClient } from "./db/client.js";
import { mountRouter } from "./db/server.js";

const db = createClient(process.env.DATABASE_URL!);
const app = Fastify();

await mountRouter(app, db, "/api");
await app.listen({ port: 3000 });
```

## Calling it over RPC

The typed client needs no knowledge of URLs or status codes:

```ts
import { createORPCClient } from "@orpc/client";
import { RPCLink } from "@orpc/client/fetch";
import type { RouterClient } from "@orpc/server";
import type { AppRouter } from "./db/router.js";

const link = new RPCLink({ url: "http://localhost:3000/api/rpc" });
const api: RouterClient<AppRouter> = createORPCClient(link);

const user = await api.users.create({ emailAddress: "a@b.com" });
const found = await api.users.get({ id: user.id });
```

A missing row raises an `ORPCError` with code `NOT_FOUND`, so failures stay typed
rather than surfacing as `undefined`.

## Calling it over REST

The same procedures answer plain HTTP:

```sh
curl localhost:3000/api/users
curl localhost:3000/api/users/1
curl -X POST localhost:3000/api/users \
  -H 'Content-Type: application/json' -d '{"emailAddress":"a@b.com"}'
```

Path parameters arrive as strings and are coerced by `z.coerce`, so a
non-numeric `:id` fails validation instead of reaching the database.

## Generating the browser client

Two paths, same result. Generate from the static document, which needs no
TypeScript and no running server:

```sh
cd db/openapi && npx @hey-api/openapi-ts
```

Or regenerate the document from the router first. This is the authoritative form,
since oRPC derives it from the procedures themselves:

```sh
npx tsx db/openapi.ts db/openapi/openapi.json
cd db/openapi && npx @hey-api/openapi-ts
```

Hey API turns an `operationId` of `users.list` into `usersList`:

```ts
import { client } from "./db/openapi/generated/client.gen";
import { usersList, usersCreate } from "./db/openapi/generated/sdk.gen";

client.setConfig({ baseUrl: "https://api.example.com" });
const users = await usersList();
```

Once deployed you can point Hey API at the live document instead, and no call
site changes.

### TypeScript version

`@hey-api/openapi-ts` 0.87.x fails under TypeScript 7 with
`Cannot read properties of undefined (reading 'LineFeed')`. Pin TypeScript 5:

```sh
npm install -D typescript@5.8.2
```

## Behaviour worth knowing

- **Composite primary keys are skipped.** There is no single `{id}` form, so no
  procedures are generated for them. The table, relations, and Zod schemas are.
- **Tables without a primary key** get a schema and types but no repository.
- **`numeric` is a string**, matching Drizzle, so decimal precision is not lost
  through a float.
- **`jsonb` columns are annotated** `.$type<Record<string, unknown>>()`, because
  Drizzle would otherwise infer `unknown` while `drizzle-zod` infers a recursive
  `Json` union, and the two are not assignable.
- **`server.ts` disables Fastify's body parsers** for its route, because the oRPC
  handlers need the raw request.
- **Re-running overwrites these files.** Keep hand-written code elsewhere.

## Verifying a generated folder

`verify-generated-api.ts` exercises all three consumers against a real database:
the RPC client, the REST surface, and the OpenAPI output.

```sh
DATABASE_URL=postgres://... npx tsx verify-generated-api.ts
```

`verify-heyapi-client.ts` does the same through a Hey API–generated client, which
is what proves the last hop of the chain.
