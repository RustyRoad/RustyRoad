/// An eligible source row linked to an input ID and an optional lowest-ID row.
/// The duplicate check uses input/source identities; uniqueness errors propagate.
/// # Examples
/// ```toml
/// [models.rust_relation_inserts.add_active_crm_list_member]
/// table = "customer_contact_lists"
/// input_column = "list_id"
/// source_table = "crm_leads"
/// source_key = "id"
/// source_target = "crm_lead_id"
/// eligible_column = "archived"
/// eligible_value = false
/// optional_table = "leads"
/// optional_source = "crm_lead_id"
/// optional_key = "id"
/// optional_target = "lead_id"
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationInsert {
    pub table: String,
    pub input_column: String,
    pub source_table: String,
    pub source_key: String,
    pub source_target: String,
    pub eligible_column: String,
    pub eligible_value: bool,
    pub optional_table: String,
    pub optional_source: String,
    pub optional_key: String,
    pub optional_target: String,
}
