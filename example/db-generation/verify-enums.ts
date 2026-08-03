// Proves a Postgres enum survives the whole chain: pgEnum in the schema, Zod
// validation on the procedure, and the allowed values in OpenAPI.
import Fastify from "fastify";
import { createORPCClient } from "@orpc/client";
import { RPCLink } from "@orpc/client/fetch";
import type { RouterClient } from "@orpc/server";
import { createClient } from "./db/client.js";
import { type AppRouter } from "./db/api.js";
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

// A valid enum value is accepted and returned.
const order = await api.orders.create({ status: "shipped" });
check("enum value accepted", order.status === "shipped", order);

// The value is typed: passing "cancelled" would fail tsc, and at runtime Zod
// rejects it rather than letting Postgres raise a 22P02.
let rejected = false;
try {
  // @ts-expect-error deliberately invalid, to prove validation rejects it
  await api.orders.create({ status: "cancelled" });
} catch (error) {
  rejected = true;
  const code = (error as { code?: string }).code;
  check("invalid enum rejected as validation", code === "BAD_REQUEST", code);
}
check("invalid enum value rejected", rejected);

// The same rejection happens over REST.
const rest = await fetch(`${origin}/api/orders`, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ status: "cancelled" }),
});
check("rest rejects invalid enum", rest.status >= 400, rest.status);

// A valid value over REST still works.
const ok = await fetch(`${origin}/api/orders`, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ status: "delivered" }),
});
check("rest accepts valid enum", ok.status === 200, ok.status);

await app.close();
