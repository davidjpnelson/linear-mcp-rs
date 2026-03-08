use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueToReleaseParams {
    /// UUID of the issue-to-release link
    pub id: String,
}
