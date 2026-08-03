//! Error and payload helpers shared by the generated procedures.

/// Renders the helper functions the procedure bodies call.
///
/// Returning an `ORPCError` rather than a bare 404 keeps the failure typed for RPC
/// callers while still mapping to the right HTTP status over REST.
pub(super) fn helpers() -> String {
    String::from(
        "/** Throws a typed NOT_FOUND when a row is missing. */\n\
         function found<T>(row: T | undefined, entity: string): T {\n\
         \tif (!row) throw new ORPCError(\"NOT_FOUND\", { message: `${entity} not found` });\n\
         \treturn row;\n\
         }\n\n\
         /** Throws a typed NOT_FOUND when a delete matched nothing. */\n\
         async function removed(result: Promise<boolean>, entity: string): Promise<boolean> {\n\
         \tif (!(await result)) {\n\
         \t\tthrow new ORPCError(\"NOT_FOUND\", { message: `${entity} not found` });\n\
         \t}\n\
         \treturn true;\n\
         }\n\n\
         /** Drops `id` from an update payload, leaving only the columns to set. */\n\
         function rest<T extends { id: unknown }>(input: T): Omit<T, \"id\"> {\n\
         \tconst { id: _id, ...values } = input;\n\
         \treturn values;\n\
         }\n\n",
    )
}

/// Renders the request context type.
pub(super) fn context() -> String {
    String::from(
        "/** Context every procedure receives. */\n\
         export interface RouterContext {\n\tdb: Database;\n}\n\n",
    )
}

/// Renders the base builder every procedure extends.
pub(super) fn base() -> String {
    String::from(
        "/** Base builder carrying the context type. */\n\
         const base = os.$context<RouterContext>();\n\n",
    )
}
