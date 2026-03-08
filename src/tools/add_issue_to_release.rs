use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddIssueToReleaseParams {
    /// Issue identifier (e.g. "ENG-123") or UUID
    pub issue: String,
    /// UUID of the release to add the issue to
    pub release: String,
}
