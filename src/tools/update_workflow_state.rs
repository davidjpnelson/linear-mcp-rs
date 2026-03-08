use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateWorkflowStateParams {
    /// Workflow state UUID
    pub id: String,
    /// New name
    pub name: Option<String>,
    /// Hex color (e.g. '#ff0000')
    pub color: Option<String>,
    /// New description
    pub description: Option<String>,
    /// Sort position
    pub position: Option<f64>,
}
