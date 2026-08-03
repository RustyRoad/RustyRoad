# `rustyroad pull` — generated database layer

Introspects a live Postgres database and writes a folder of TypeScript, in the
spirit of `drizzle-kit pull` but carrying the API layer as well.

```sh
rustyroad pull                              # -> ./db
rustyroad pull --out ./src/db --casing preserve
rustyroad pull --schema-only                # schema.ts + relations.ts only
```

## What is generated

| File | Contents |
|---|---|
| `schema.ts` | Drizzle tables, columns, defaults, constraints, indexes |
| `relations.ts` | `one`/`many` relations derived from foreign keys |
| `zod.ts` | Zod schemas derived from the tables via `drizzle-zod` |
| `client.ts` | A typed repository per table |
| `routes.ts` | Fastify plugins using `fastify-type-provider-zod` |

## Why Zod is the centre

The Zod schemas are not a second description of the database. `drizzle-zod`
derives them from the Drizzle tables, and those tables are generated from the
live schema. So one definition drives four things:

```
Postgres ──► schema.ts ──► zod.ts ──► request validation
                              ├──────► response serialization
                              └──────► OpenAPI ──► Hey API client
```

Change a column, re-run `pull`, and validation, serialization, the OpenAPI
document, and the generated client all move together. There is no hand-written
validator to fall out of step.

## Dependencies

```sh
npm install drizzle-orm pg zod drizzle-zod fastify \
            fastify-type-provider-zod @fastify/swagger
npm install -D drizzle-kit
```

## Wiring it up

The Zod provider replaces Fastify's default AJV compilers, and
`jsonSchemaTransform` is what turns the Zod schemas into OpenAPI:

```ts
import Fastify from "fastify";
import swagger from "@fastify/swagger";
import {
  serializerCompiler,
  validatorCompiler,
  jsonSchemaTransform,
} from "fastify-type-provider-zod";
import { createClient } from "./db/client.js";
import { apiRoutes } from "./db/routes.js";

const db = createClient(process.env.DATABASE_URL!);
const app = Fastify();

app.setValidatorCompiler(validatorCompiler);
app.setSerializerCompiler(serializerCompiler);

await app.register(swagger, {
  openapi: { info: { title: "api", version: "1.0.0" } },
  transform: jsonSchemaTransform,
});
await app.register(apiRoutes, { db, prefix: "/api" });

await app.listen({ port: 3000 });
```

## Hey API

Point Hey API at the OpenAPI document the server now serves. Route
`operationId`s are generated as `listUsers`, `getUsers`, `createUsers`,
`updateUsers`, `deleteUsers`, so the generated client method names are stable
across re-runs.

```sh
npx @hey-api/openapi-ts -i http://localhost:3000/documentation/json -o src/api
```

## Generated routes

Per table with a single-column primary key:

| Method | Path | Body | Success |
|---|---|---|---|
| GET | `/` | — | 200 array |
| GET | `/:id` | — | 200 or 404 |
| POST | `/` | insert schema | 201 |
| PATCH | `/:id` | update schema | 200 or 404 |
| DELETE | `/:id` | — | 204 or 404 |

## Behaviour worth knowing

- **Composite primary keys are skipped.** There is no single `/:id` form, so no
  routes are invented for them. The table, relations, and Zod schemas are still
  generated.
- **Tables without a primary key** get a schema and types but no repository.
- **`numeric` is a string**, matching Drizzle, so decimal precision is not lost
  through a float.
- **Integer path params are coerced** with `z.coerce.number().int()`. A
  non-numeric `:id` fails validation with 400 rather than reaching the database.
- **`jsonb` columns are annotated** `.$type<Record<string, unknown>>()`, because
  Drizzle would otherwise infer `unknown` while `drizzle-zod` infers a recursive
  `Json` union, and the two are not assignable.
- **Re-running overwrites these files.** Keep hand-written code elsewhere.

## Verifying a generated folder

`verify-generated-api.ts` boots the generated routes against a real database and
exercises every verb, validation failure, and the OpenAPI output.

```sh
DATABASE_URL=postgres://... npx tsx verify-generated-api.ts
```
