use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateProjectParams {
    /// Project name
    pub name: String,
    /// Project description (markdown)
    pub description: Option<String>,
    /// Comma-separated team keys (e.g. 'ENG, DESIGN')
    pub teams: Option<String>,
    /// Lead email address
    pub lead: Option<String>,
    /// Target date (ISO format, e.g. '2025-06-01')
    #[serde(rename = "targetDate")]
    pub target_date: Option<String>,
    /// Start date (ISO format)
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
}
