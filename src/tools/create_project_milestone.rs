use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateProjectMilestoneParams {
    /// Project name or UUID
    pub project: String,
    /// Milestone name
    pub name: String,
    /// Milestone description
    pub description: Option<String>,
    /// Target date (ISO format)
    #[serde(rename = "targetDate")]
    pub target_date: Option<String>,
}
