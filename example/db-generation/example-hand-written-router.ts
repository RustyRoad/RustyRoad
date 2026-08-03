// Stands in for hand-written procedures a project adds alongside the generated
// ones. `pull` must never overwrite this file.
import { os, ORPCError } from "@orpc/server";
import { z } from "zod";
import type { RouterContext } from "./router";

const base = os.$context<RouterContext>();

/** A procedure the generator knows nothing about. */
export const billingQuote = base
  .route({ method: "POST", path: "/api/billing/quote" })
  .input(z.object({ plan: z.enum(["basic", "pro"]) }))
  .output(z.object({ cents: z.number().int() }))
  .handler(async ({ input }) => ({
    cents: input.plan === "pro" ? 9900 : 1900,
  }));

/** Deliberately fails, to prove custom errors still work. */
export const billingRefund = base
  .route({ method: "POST", path: "/api/billing/refund" })
  .input(z.object({ id: z.string() }))
  .output(z.object({ refunded: z.boolean() }))
  .handler(async () => {
    throw new ORPCError("FORBIDDEN", { message: "Refunds are manual" });
  });

export const billingRouter = { quote: billingQuote, refund: billingRefund };
