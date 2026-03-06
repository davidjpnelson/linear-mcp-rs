use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateInitiativeUpdateParams {
    /// Initiative name or UUID
    pub initiative: String,
    /// Update body content (markdown)
    pub body: String,
    /// Health status (onTrack, atRisk, offTrack)
    pub health: Option<String>,
}
