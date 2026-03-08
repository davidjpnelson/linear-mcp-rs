use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCustomerNeedParams {
    /// UUID of the customer need
    pub id: String,
}
