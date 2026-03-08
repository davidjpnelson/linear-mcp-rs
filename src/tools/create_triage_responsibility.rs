use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTriageResponsibilityParams {
    /// Team key
    pub team: String,
    /// "assign" or "comment"
    pub action: String,
    /// Time schedule ID
    pub time_schedule_id: Option<String>,
}
