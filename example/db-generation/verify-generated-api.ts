// Proves one oRPC router serves three callers: a typed RPC client, plain REST,
// and OpenAPI generation — all from the same procedure definitions.
import Fastify from "fastify";
import { createORPCClient } from "@orpc/client";
import { RPCLink } from "@orpc/client/fetch";
import { OpenAPIGenerator } from "@orpc/openapi";
import { ZodToJsonSchemaConverter } from "@orpc/zod/zod4";
import type { RouterClient } from "@orpc/server";
import { createClient } from "./db/client.js";
import { router, type AppRouter } from "./db/router.js";
import { mountRouter } from "./db/server.js";

const db = createClient(process.env.DATABASE_URL!);
const app = Fastify();
await mountRouter(app, db, "/api");
await app.listen({ port: 0 });

const address = app.server.address();
const port = typeof address === "object" && address ? address.port : 0;
const origin = `http://127.0.0.1:${port}`;

function check(label: string, ok: boolean, detail: unknown = "") {
  console.log(`${ok ? "PASS" : "FAIL"} ${label}`, ok ? "" : detail);
  if (!ok) process.exitCode = 1;
}

// ---- 1. The typed RPC client, with no HTTP shape to remember ----
const link = new RPCLink({ url: `${origin}/api/rpc` });
const rpc: RouterClient<AppRouter> = createORPCClient(link);

const created = await rpc.users.create({
  emailAddress: "orpc@example.com",
  displayName: "RPC",
});
check("rpc users.create", typeof created.id === "number", created);
check("numeric is a string", typeof created.balance === "string", created);

const listed = await rpc.users.list({});
check("rpc users.list", Array.isArray(listed) && listed.length === 1, listed);

const fetched = await rpc.users.get({ id: created.id });
check("rpc users.get", fetched.id === created.id, fetched);

const patched = await rpc.users.update({ id: created.id, displayName: "Renamed" });
check("rpc users.update", patched.displayName === "Renamed", patched);

// A missing row must arrive as a typed error, not a silent undefined.
let notFound = false;
try {
  await rpc.users.get({ id: 999999 });
} catch (error) {
  notFound = (error as { code?: string }).code === "NOT_FOUND";
}
check("rpc missing row -> NOT_FOUND", notFound);

// ---- 2. The same procedures over plain REST ----
const restList = await fetch(`${origin}/api/users`);
check("rest GET /api/users -> 200", restList.status === 200, restList.status);
check("rest list returns rows", (await restList.json()).length === 1);

const restGet = await fetch(`${origin}/api/users/${created.id}`);
check("rest GET /api/users/:id -> 200", restGet.status === 200, restGet.status);

// The path param is a string over REST; z.coerce turns it into a number.
const restRow = await restGet.json();
check("rest coerced the id", restRow.id === created.id, restRow);

const restCreate = await fetch(`${origin}/api/users`, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ emailAddress: "rest@example.com" }),
});
check("rest POST -> 200", restCreate.status === 200, await restCreate.text());

const restMissing = await fetch(`${origin}/api/users/999999`);
check("rest missing -> 404", restMissing.status === 404, restMissing.status);

// Validation must reject a wrong type rather than coercing it.
const restInvalid = await fetch(`${origin}/api/users`, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ emailAddress: 12345 }),
});
check("rest rejects wrong type", restInvalid.status >= 400, restInvalid.status);

const restDelete = await fetch(`${origin}/api/users/${created.id}`, {
  method: "DELETE",
});
check("rest DELETE -> 200", restDelete.status === 200, restDelete.status);

// ---- 3. OpenAPI generated from the same router ----
const generator = new OpenAPIGenerator({
  schemaConverters: [new ZodToJsonSchemaConverter()],
});
const document = await generator.generate(router, {
  info: { title: "Generated API", version: "1.0.0" },
});

const paths = Object.keys(document.paths ?? {});
check("openapi covers the collection", paths.includes("/api/users"), paths);
check("openapi covers the item", paths.includes("/api/users/{id}"), paths);
check("openapi covers both tables", paths.includes("/api/posts"), paths);

// The document must carry real schemas, or a generated client would be useless.
const post = JSON.stringify((document.paths as Record<string, unknown>)["/api/users"]);
check("openapi body carries properties", post.includes("emailAddress"), post.slice(0, 300));

console.log("openapi paths:", paths.join(", "));

await app.close();
