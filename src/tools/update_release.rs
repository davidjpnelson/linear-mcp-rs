use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateReleaseParams {
    /// Release UUID
    pub id: String,
    /// New release name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New version string
    pub version: Option<String>,
    /// New commit SHA
    #[serde(rename = "commitSha")]
    pub commit_sha: Option<String>,
    /// Stage UUID to move the release to
    pub stage: Option<String>,
}
