//! Render key drop/add clauses within the same atomic ALTER as column repairs.
use super::super::super::types::PrimaryKey;
use super::super::quote;

pub(in crate::model_audit::schema::proposals) struct Change {
    pub before: Option<PrimaryKey>,
    pub before_columns: Vec<String>,
    pub name: String,
    pub after: Vec<String>,
}

impl Change {
    /// Drop only the explicitly replaced key, never its dependents via CASCADE.
    pub(in crate::model_audit::schema::proposals) fn drop_clause(
        &self,
        down: bool,
    ) -> Option<String> {
        let name = if down {
            Some(&self.name)
        } else {
            self.before.as_ref().map(|key| &key.name)
        }?;
        Some(format!("DROP CONSTRAINT {} RESTRICT", quote(name)))
    }

    /// Recreate the observed key's identity and deferral properties on rollback.
    pub(in crate::model_audit::schema::proposals) fn add_clause(
        &self,
        down: bool,
    ) -> Option<String> {
        let (name, columns, properties) = if down {
            let key = self.before.as_ref()?;
            let properties = if key.deferrable {
                if key.initially_deferred {
                    " DEFERRABLE INITIALLY DEFERRED"
                } else {
                    " DEFERRABLE INITIALLY IMMEDIATE"
                }
            } else {
                " NOT DEFERRABLE"
            };
            (&key.name, &self.before_columns, properties)
        } else {
            (&self.name, &self.after, " NOT DEFERRABLE")
        };
        let columns = columns
            .iter()
            .map(|column| quote(column))
            .collect::<Vec<_>>()
            .join(", ");
        Some(format!(
            "ADD CONSTRAINT {} PRIMARY KEY ({columns}){properties}",
            quote(name)
        ))
    }
}
