// Proves the generated Hey API client talks to the generated Fastify server.
//
// The server is booted from db/routes.ts and the client is generated from
// db/openapi/openapi.json, so a mismatch in URL, method, body shape, or status
// code shows up here rather than in production.
import Fastify from "fastify";
import {
  serializerCompiler,
  validatorCompiler,
} from "fastify-type-provider-zod";
import { createClient } from "./db/client.js";
import { apiRoutes } from "./db/routes.js";
import { client } from "./db/openapi/generated/client.gen.js";
import {
  createUsers,
  deleteUsers,
  getUsers,
  listUsers,
  updateUsers,
} from "./db/openapi/generated/sdk.gen.js";
import type { Users } from "./db/openapi/generated/types.gen.js";

const db = createClient(process.env.DATABASE_URL!);
const app = Fastify();

app.setValidatorCompiler(validatorCompiler);
app.setSerializerCompiler(serializerCompiler);
await app.register(apiRoutes, { db, prefix: "/api" });
await app.listen({ port: 0 });

const address = app.server.address();
const port = typeof address === "object" && address ? address.port : 0;

// Point the generated client at the running server.
client.setConfig({ baseUrl: `http://127.0.0.1:${port}` });

function check(label: string, ok: boolean, detail: unknown = "") {
  console.log(`${ok ? "PASS" : "FAIL"} ${label}`, ok ? "" : detail);
  if (!ok) process.exitCode = 1;
}

// The SDK's function names come from the operationIds we generate.
const created = await createUsers({
  body: { emailAddress: "hey@example.com", displayName: "Hey" },
});
check("createUsers -> 201", created.response.status === 201, created.error);

const user = created.data as Users;
check("created row is typed", typeof user.id === "number", user);
check("numeric is a string", typeof user.balance === "string", user);

const listed = await listUsers();
check("listUsers -> 200", listed.response.status === 200, listed.error);
check("listUsers returns an array", Array.isArray(listed.data), listed.data);

// The path parameter is serialized by the client from the OpenAPI declaration.
const fetched = await getUsers({ path: { id: user.id } });
check("getUsers -> 200", fetched.response.status === 200, fetched.error);
check("getUsers returns the row", fetched.data?.id === user.id, fetched.data);

const patched = await updateUsers({
  path: { id: user.id },
  body: { displayName: "Renamed" },
});
check("updateUsers -> 200", patched.response.status === 200, patched.error);
check("patch applied", patched.data?.displayName === "Renamed", patched.data);

// A missing row must surface as a typed 404 rather than a thrown exception.
const missing = await getUsers({ path: { id: 999999 } });
check("getUsers missing -> 404", missing.response.status === 404, missing.data);

const removed = await deleteUsers({ path: { id: user.id } });
check("deleteUsers -> 204", removed.response.status === 204, removed.error);

const afterDelete = await getUsers({ path: { id: user.id } });
check("row is gone -> 404", afterDelete.response.status === 404, afterDelete.data);

await app.close();
