use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteCustomerTierParams {
    /// UUID of the customer tier to delete
    pub id: String,
}
