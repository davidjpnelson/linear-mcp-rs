use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTriageResponsibilityParams {
    /// The triage responsibility ID
    pub id: String,
}
