//! Resolve ordinary module edges without inferring path overrides.
use super::{plain, unconditional, Item, Node};

/// Follow source declarations without guessing module names from filenames.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --offline --report-only
/// ```
impl Node {
    /// Reject conditional, duplicate and ambiguous declarations before traversal.
    /// Path overrides remain outside the auditor's supported ownership contract.
    /// # Examples
    /// ```sh
    /// rustyroad model audit --source src/models --offline --report-only
    /// ```
    pub(in crate::model_audit::schema::rust_types::imports::source) fn child(
        &self,
        name: &str,
    ) -> Option<Self> {
        if !super::super::super::namespace::unique(&self.items, name) {
            return None;
        }
        let declarations: Vec<_> = self
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(module) if module.ident == name => Some(module),
                _ => None,
            })
            .collect();
        if declarations.len() != 1 {
            return None;
        }
        let module = declarations[0];
        if !unconditional(&module.attrs)
            || module
                .attrs
                .iter()
                .any(|attribute| attribute.path().is_ident("path"))
        {
            return None;
        }
        let attributes: Vec<_> = module
            .attrs
            .iter()
            .filter(|attribute| !attribute.path().is_ident("path"))
            .cloned()
            .collect();
        let plain = self.plain && plain(&attributes);
        let directory = self.directory.join(name);
        if let Some((_, items)) = &module.content {
            if attributes.len() != module.attrs.len() {
                return None;
            }
            return Some(Self {
                items: items.clone(),
                file: self.file.clone(),
                path_directory: directory.clone(),
                directory,
                boundary: self.boundary.clone(),
                plain,
            });
        }
        let (file, directory) = self.module_file(module)?;
        let mut node = Self::read(&file, directory)?;
        node.boundary = self.boundary.clone();
        node.plain &= plain;
        Some(node)
    }
}
