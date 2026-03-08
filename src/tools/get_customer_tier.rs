use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCustomerTierParams {
    /// UUID of the customer tier
    pub id: String,
}
