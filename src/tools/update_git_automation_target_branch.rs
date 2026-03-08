use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateGitAutomationTargetBranchParams {
    /// The git automation target branch ID
    pub id: String,
    /// Branch pattern to match
    pub branch_pattern: Option<String>,
    /// Whether the pattern is a regex
    pub is_regex: Option<bool>,
}
