use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateGitAutomationTargetBranchParams {
    /// Team key
    pub team: String,
    /// Branch pattern to match
    pub branch_pattern: String,
    /// Whether the pattern is a regex
    pub is_regex: Option<bool>,
}
