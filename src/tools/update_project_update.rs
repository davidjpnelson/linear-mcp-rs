use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateProjectUpdateParams {
    /// Project update UUID
    pub id: String,
    /// New body content (markdown)
    pub body: Option<String>,
    /// New health status (onTrack, atRisk, offTrack)
    pub health: Option<String>,
}
