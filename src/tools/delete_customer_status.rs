use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteCustomerStatusParams {
    /// UUID of the customer status to delete
    pub id: String,
}
