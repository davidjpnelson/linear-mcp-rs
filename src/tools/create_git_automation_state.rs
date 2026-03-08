use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateGitAutomationStateParams {
    /// Team key
    pub team: String,
    /// Git event type
    pub event: String,
    /// Workflow state ID
    pub state_id: Option<String>,
    /// Target branch ID
    pub target_branch_id: Option<String>,
}
