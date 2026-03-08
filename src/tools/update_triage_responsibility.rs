use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTriageResponsibilityParams {
    /// The triage responsibility ID
    pub id: String,
    /// "assign" or "comment"
    pub action: Option<String>,
    /// Time schedule ID
    pub time_schedule_id: Option<String>,
}
