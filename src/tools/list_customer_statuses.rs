use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCustomerStatusesParams {
    /// Maximum number of customer statuses to return
    pub limit: Option<i32>,
}
