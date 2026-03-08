use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCustomerStatusParams {
    /// UUID of the customer status
    pub id: String,
}
