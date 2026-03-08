use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveCustomerNeedParams {
    /// UUID of the customer need to unarchive
    pub id: String,
}
