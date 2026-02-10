use schemars::JsonSchema;
use serde::Deserialize;

use super::list_issues::PriorityLevel;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateIssueParams {
    /// Team key (e.g. 'ENG') — required
    pub team: String,
    /// Issue title
    pub title: String,
    /// Issue description (markdown)
    pub description: Option<String>,
    /// Assignee email address
    pub assignee: Option<String>,
    /// Workflow state name (e.g. 'Todo', 'In Progress')
    pub status: Option<String>,
    /// Priority level
    pub priority: Option<PriorityLevel>,
    /// Point estimate
    pub estimate: Option<f64>,
    /// Due date (ISO format, e.g. '2025-03-01')
    #[serde(rename = "dueDate")]
    pub due_date: Option<String>,
}
