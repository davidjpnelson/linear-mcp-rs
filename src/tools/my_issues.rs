use schemars::JsonSchema;
use serde::Deserialize;

use super::list_issues::PriorityLevel;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MyIssuesParams {
    /// Include completed/canceled issues (default false)
    #[serde(rename = "includeCompleted")]
    pub include_completed: Option<bool>,
    /// Filter by team key (e.g. 'ENG')
    pub team: Option<String>,
    /// Filter by priority level
    pub priority: Option<PriorityLevel>,
    /// Max results (default 50)
    pub limit: Option<u32>,
    /// Pagination cursor from a previous response
    pub cursor: Option<String>,
}
