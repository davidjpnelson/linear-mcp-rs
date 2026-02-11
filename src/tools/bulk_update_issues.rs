use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BulkUpdateIssuesParams {
    /// Comma-separated issue identifiers (e.g. "ENG-1,ENG-2,ENG-3") or UUIDs. Max 50.
    pub ids: String,
    /// New state/status name (e.g. "In Progress", "Done")
    pub state: Option<String>,
    /// Assignee email address (use "none" to unassign)
    pub assignee: Option<String>,
    /// Priority: urgent, high, normal, low, none
    pub priority: Option<String>,
    /// Comma-separated label names to add
    #[serde(rename = "addLabels")]
    pub add_labels: Option<String>,
    /// Comma-separated label names to remove
    #[serde(rename = "removeLabels")]
    pub remove_labels: Option<String>,
    /// Project name or UUID to move issues to
    pub project: Option<String>,
    /// Cycle UUID to assign issues to
    pub cycle: Option<String>,
    /// Team key to move issues to (e.g. "ENG")
    pub team: Option<String>,
}
