// Boots the generated Zod-validated Fastify routes against the live database,
// exercises every verb, and dumps the OpenAPI document Hey API would consume.
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

// The Zod provider replaces Fastify's default AJV compilers.
app.setValidatorCompiler(validatorCompiler);
app.setSerializerCompiler(serializerCompiler);

await app.register(swagger, {
  openapi: { info: { title: "generated", version: "1.0.0" } },
  transform: jsonSchemaTransform,
});
await app.register(apiRoutes, { db, prefix: "/api" });
await app.ready();

function check(label: string, ok: boolean, detail: unknown = "") {
  console.log(`${ok ? "PASS" : "FAIL"} ${label}`, ok ? "" : detail);
  if (!ok) process.exitCode = 1;
}

// POST validated by the derived insert schema.
const created = await app.inject({
  method: "POST",
  url: "/api/users",
  payload: { emailAddress: "zod@example.com", displayName: "Zod" },
});
check("POST /users -> 201", created.statusCode === 201, created.body);
const user = created.json();

const list = await app.inject({ method: "GET", url: "/api/users" });
check("GET /users -> 200", list.statusCode === 200, list.body);
check("GET /users returns the row", list.json().length === 1, list.body);

// The path param is coerced by z.coerce.number().
const found = await app.inject({ method: "GET", url: `/api/users/${user.id}` });
check("GET /users/:id -> 200", found.statusCode === 200, found.body);
check("id coerced to number", typeof found.json().id === "number", found.body);
check("numeric stays a string", typeof found.json().balance === "string", found.json());

const patched = await app.inject({
  method: "PATCH",
  url: `/api/users/${user.id}`,
  payload: { displayName: "Renamed" },
});
check("PATCH /users/:id -> 200", patched.statusCode === 200, patched.body);
check("PATCH applied", patched.json().displayName === "Renamed", patched.body);

// Zod rejects a wrong type outright, unlike AJV coercion.
const wrongType = await app.inject({
  method: "POST",
  url: "/api/users",
  payload: { emailAddress: 12345 },
});
check("POST rejects wrong type -> 400", wrongType.statusCode === 400, wrongType.body);

// A missing required field must be rejected.
const incomplete = await app.inject({
  method: "POST",
  url: "/api/users",
  payload: { displayName: "no email" },
});
check("POST requires emailAddress -> 400", incomplete.statusCode === 400, incomplete.body);

// A non-numeric id cannot coerce, so validation fails rather than querying.
const badId = await app.inject({ method: "GET", url: "/api/users/not-a-number" });
check("GET with bad id -> 400", badId.statusCode === 400, badId.body);

const missing = await app.inject({ method: "GET", url: "/api/users/999999" });
check("GET missing -> 404", missing.statusCode === 404, missing.body);

const post = await app.inject({
  method: "POST",
  url: "/api/posts",
  payload: { authorId: user.id, title: "Hello" },
});
check("POST /posts with FK -> 201", post.statusCode === 201, post.body);

const removed = await app.inject({
  method: "DELETE",
  url: `/api/users/${user.id}`,
});
check("DELETE /users/:id -> 204", removed.statusCode === 204, removed.body);

// OpenAPI generated from the same Zod schemas is what Hey API reads.
const spec = app.swagger() as {
  paths: Record<string, Record<string, { operationId?: string }>>;
  components?: { schemas?: Record<string, unknown> };
};
const operations = Object.values(spec.paths).flatMap((path) =>
  Object.values(path).map((op) => op.operationId),
);
check("OpenAPI exposes operationIds", operations.includes("listUsers"), operations);
check("OpenAPI covers both tables", operations.includes("createPosts"), operations);

// The document must carry real property schemas, not empty objects.
const users = JSON.stringify(spec.paths["/api/users/"]);
check("OpenAPI body schema has properties", users.includes("emailAddress"), users.slice(0, 400));

console.log("openapi paths:", Object.keys(spec.paths).join(", "));

await app.close();
