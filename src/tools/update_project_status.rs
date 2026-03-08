use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateProjectStatusParams {
    /// UUID of the project status to update
    pub id: String,
    /// New name for the project status
    pub name: Option<String>,
    /// New hex color code (e.g. "#ff0000")
    pub color: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New position in the list
    pub position: Option<f64>,
    /// New type: backlog, planned, started, paused, completed, or canceled
    #[serde(rename = "type")]
    pub status_type: Option<String>,
    /// Whether this status represents an indefinite state
    pub indefinite: Option<bool>,
}
