use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteGitAutomationTargetBranchParams {
    /// The git automation target branch ID
    pub id: String,
}
