use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteCustomerParams {
    /// Customer UUID
    pub id: String,
}
