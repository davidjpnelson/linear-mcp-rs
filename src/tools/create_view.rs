use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateViewParams {
    /// View name — required
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Color
    pub color: Option<String>,
    /// Icon
    pub icon: Option<String>,
    /// Team key to scope the view to (e.g. 'ENG')
    pub team: Option<String>,
    /// Whether the view is shared with the team
    pub shared: Option<bool>,
}
