use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCustomerParams {
    /// Customer UUID
    pub id: String,
}
