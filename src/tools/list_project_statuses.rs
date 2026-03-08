use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListProjectStatusesParams {
    /// Maximum number of project statuses to return
    pub limit: Option<i32>,
}
