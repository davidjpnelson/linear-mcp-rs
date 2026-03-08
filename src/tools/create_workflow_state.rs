use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateWorkflowStateParams {
    /// Team key (e.g. 'ENG')
    pub team: String,
    /// State name
    pub name: String,
    /// Hex color (e.g. '#ff0000') — required
    pub color: String,
    /// State type: triage, backlog, unstarted, started, completed, or canceled
    #[serde(rename = "type")]
    pub state_type: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional sort position
    pub position: Option<f64>,
}
