//! Resolve wildcard targets through declared modules, never dependency basenames.
use super::Context;
use syn::{Item, Path};

impl Context {
    /// Read `crate::helpers` from the verified source tree owning this row.
    pub(in crate::model_audit::schema::rust_types::imports) fn glob_items(
        &self,
        path: &Path,
        scope: usize,
    ) -> Option<Vec<Item>> {
        let mut modules = self.modules.clone();
        modules.extend_from_slice(self.inline.get(..scope)?);
        // Attributes on the importing module can inject otherwise unseen types.
        if !self.node(&modules)?.plain {
            return None;
        }
        let current = modules.clone();
        let mut parts = path.segments.iter().peekable();
        match parts.peek()?.ident.to_string().as_str() {
            "crate" => {
                modules.clear();
                parts.next();
            }
            "self" => {
                parts.next();
            }
            _ => {}
        }
        while parts.peek().is_some_and(|part| part.ident == "super") {
            modules.pop()?;
            parts.next();
        }
        for part in parts {
            modules.push(part.ident.to_string());
        }
        if current.starts_with(&modules) {
            return None;
        }
        let node = self.node(&modules)?;
        node.plain.then_some(node.items)
    }
}
