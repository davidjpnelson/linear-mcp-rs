use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateGitAutomationStateParams {
    /// The git automation state ID
    pub id: String,
    /// Workflow state ID
    pub state_id: Option<String>,
    /// Target branch ID
    pub target_branch_id: Option<String>,
    /// Git event type
    pub event: Option<String>,
}
