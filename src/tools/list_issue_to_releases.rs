use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListIssueToReleasesParams {
    /// Maximum number of issue-to-release links to return
    pub limit: Option<i32>,
}
