use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveIssueFromReleaseParams {
    /// UUID of the issue-to-release link to remove
    pub id: String,
}
