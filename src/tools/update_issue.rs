use schemars::JsonSchema;
use serde::Deserialize;

use super::list_issues::PriorityLevel;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateIssueParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    pub id: String,
    /// New title
    pub title: Option<String>,
    /// New description (markdown)
    pub description: Option<String>,
    /// New workflow state name (e.g. 'In Progress', 'Done')
    pub status: Option<String>,
    /// New assignee email (use 'none' to unassign)
    pub assignee: Option<String>,
    /// New priority level
    pub priority: Option<PriorityLevel>,
    /// New point estimate
    pub estimate: Option<f64>,
    /// New due date (ISO format, or 'none' to clear)
    #[serde(rename = "dueDate")]
    pub due_date: Option<String>,
    /// Comma-separated label names to set (replaces existing labels)
    pub labels: Option<String>,
    /// Project name to assign (use 'none' to remove from project)
    pub project: Option<String>,
    /// Parent issue identifier (e.g. 'ENG-100'), or 'none' to remove parent
    pub parent: Option<String>,
}
