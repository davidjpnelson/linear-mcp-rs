use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateProjectParams {
    /// Project name or UUID
    pub id: String,
    /// New project name
    pub name: Option<String>,
    /// New description (markdown)
    pub description: Option<String>,
    /// New state (e.g. 'planned', 'started', 'paused', 'completed', 'canceled')
    pub state: Option<String>,
    /// New lead email (use 'none' to remove lead)
    pub lead: Option<String>,
    /// New target date (ISO format, or 'none' to clear)
    #[serde(rename = "targetDate")]
    pub target_date: Option<String>,
    /// New start date (ISO format, or 'none' to clear)
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
}
