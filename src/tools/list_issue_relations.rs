use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListIssueRelationsParams {
    /// Maximum number of results to return
    pub limit: Option<i32>,
}
