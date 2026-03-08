use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateProjectStatusParams {
    /// Name of the project status
    pub name: String,
    /// Hex color code for the project status (e.g. "#ff0000")
    pub color: String,
    /// Type of the project status: backlog, planned, started, paused, completed, or canceled
    #[serde(rename = "type")]
    pub status_type: String,
    /// Position of the project status in the list
    pub position: f64,
    /// Description of the project status
    pub description: Option<String>,
    /// Whether this status represents an indefinite state
    pub indefinite: Option<bool>,
}
