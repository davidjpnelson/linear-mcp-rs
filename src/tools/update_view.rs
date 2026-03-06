use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateViewParams {
    /// Custom view UUID
    pub id: String,
    /// New view name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New color
    pub color: Option<String>,
    /// New icon
    pub icon: Option<String>,
    /// Whether the view is shared with the team
    pub shared: Option<bool>,
}
