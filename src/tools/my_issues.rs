use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MyIssuesParams {
    /// Include completed/canceled issues (default false)
    #[serde(rename = "includeCompleted")]
    pub include_completed: Option<bool>,
    /// Max results (default 50)
    pub limit: Option<u32>,
}
