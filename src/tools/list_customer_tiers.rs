use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCustomerTiersParams {
    /// Maximum number of customer tiers to return
    pub limit: Option<i32>,
}
