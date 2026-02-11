use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TriageIssueParams {
    /// Issue identifier (e.g. "ENG-123") or UUID
    pub id: String,
    /// Target state name to move to (e.g. "Todo", "Backlog", "In Progress")
    pub state: String,
    /// Assignee email address (optional)
    pub assignee: Option<String>,
    /// Priority: urgent, high, normal, low, none (optional)
    pub priority: Option<String>,
}
