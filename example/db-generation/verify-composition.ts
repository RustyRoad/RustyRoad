// Proves generated and hand-written procedures are served together, and that a
// re-pull did not disturb either.
import Fastify from "fastify";
import { createORPCClient } from "@orpc/client";
import { RPCLink } from "@orpc/client/fetch";
import { OpenAPIGenerator } from "@orpc/openapi";
import { ZodToJsonSchemaConverter } from "@orpc/zod/zod4";
import type { RouterClient } from "@orpc/server";
import { createClient } from "./db/client.js";
import { router, type AppRouter } from "./db/api.js";
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

const link = new RPCLink({ url: `${origin}/api/rpc` });
const api: RouterClient<AppRouter> = createORPCClient(link);

// Generated procedures still work, including the column added before re-pulling.
const user = await api.users.create({
  emailAddress: "own@example.com",
  phone: "555-0111",
});
check("generated procedure works", user.phone === "555-0111", user);

// The hand-written procedure is reachable through the same client.
const quote = await api.billing.quote({ plan: "pro" });
check("hand-written procedure works", quote.cents === 9900, quote);

// Its custom error is preserved rather than flattened.
let forbidden = false;
try {
  await api.billing.refund({ id: "abc" });
} catch (error) {
  forbidden = (error as { code?: string }).code === "FORBIDDEN";
}
check("hand-written error code survives", forbidden);

// Both are reachable over REST at their declared paths.
const restGenerated = await fetch(`${origin}/api/users`);
check("rest reaches generated", restGenerated.status === 200, restGenerated.status);

const restCustom = await fetch(`${origin}/api/billing/quote`, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ plan: "basic" }),
});
check("rest reaches hand-written", restCustom.status === 200, restCustom.status);
check("hand-written body correct", (await restCustom.json()).cents === 1900);

// The OpenAPI document covers both, so a generated client sees everything.
const generator = new OpenAPIGenerator({
  schemaConverters: [new ZodToJsonSchemaConverter()],
});
const document = await generator.generate(router, {
  info: { title: "Composed API", version: "1.0.0" },
});
const paths = Object.keys(document.paths ?? {});
check("openapi covers generated", paths.includes("/api/users"), paths);
check("openapi covers hand-written", paths.includes("/api/billing/quote"), paths);

console.log("openapi paths:", paths.join(", "));

await app.close();
