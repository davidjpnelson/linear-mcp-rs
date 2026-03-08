use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTriageResponsibilityParams {
    /// The triage responsibility ID
    pub id: String,
}
