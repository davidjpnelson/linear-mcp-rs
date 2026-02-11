use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateProjectUpdateParams {
    /// Project name or UUID
    pub project: String,
    /// Update body (markdown)
    pub body: String,
    /// Health status: 'onTrack', 'atRisk', or 'offTrack'
    pub health: Option<String>,
}
