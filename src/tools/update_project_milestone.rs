use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateProjectMilestoneParams {
    /// Project milestone UUID
    pub id: String,
    /// New milestone name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New target date (ISO format, e.g. '2025-06-01')
    #[serde(rename = "targetDate")]
    pub target_date: Option<String>,
}
