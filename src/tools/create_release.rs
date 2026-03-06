use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateReleaseParams {
    /// Release name — required
    pub name: String,
    /// Release pipeline UUID — required
    pub pipeline: String,
    /// Description
    pub description: Option<String>,
    /// Version string
    pub version: Option<String>,
    /// Commit SHA
    #[serde(rename = "commitSha")]
    pub commit_sha: Option<String>,
    /// Start date (ISO format)
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    /// Target date (ISO format)
    #[serde(rename = "targetDate")]
    pub target_date: Option<String>,
}
