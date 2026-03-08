use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateProjectLabelParams {
    /// Name of the project label
    pub name: String,
    /// Hex color code for the project label (e.g. "#ff0000")
    pub color: Option<String>,
    /// Description of the project label
    pub description: Option<String>,
    /// UUID of the parent label (for nested labels)
    pub parent_id: Option<String>,
    /// Whether this label is a group label
    pub is_group: Option<bool>,
}
