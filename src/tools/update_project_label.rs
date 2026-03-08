use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateProjectLabelParams {
    /// UUID of the project label to update
    pub id: String,
    /// New name for the project label
    pub name: Option<String>,
    /// New hex color code (e.g. "#ff0000")
    pub color: Option<String>,
    /// New description
    pub description: Option<String>,
    /// UUID of the new parent label
    pub parent_id: Option<String>,
    /// Whether this label is a group label
    pub is_group: Option<bool>,
}
