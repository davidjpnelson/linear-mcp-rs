use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateLabelParams {
    /// Label name
    pub name: String,
    /// Team key (e.g. 'ENG'). If not provided, creates a workspace-level label.
    pub team: Option<String>,
    /// Hex color (e.g. '#ff0000'). Optional.
    pub color: Option<String>,
}
