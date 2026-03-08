use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteGitAutomationStateParams {
    /// The git automation state ID
    pub id: String,
}
