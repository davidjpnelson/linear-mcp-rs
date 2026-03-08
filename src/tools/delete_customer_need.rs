use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteCustomerNeedParams {
    /// UUID of the customer need to delete
    pub id: String,
}
