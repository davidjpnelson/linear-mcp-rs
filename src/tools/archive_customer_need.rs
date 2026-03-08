use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArchiveCustomerNeedParams {
    /// UUID of the customer need to archive
    pub id: String,
}
