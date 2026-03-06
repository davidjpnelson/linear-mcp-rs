use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct IssueVcsBranchSearchParams {
    /// Git branch name to search for
    #[serde(rename = "branchName")]
    pub branch_name: String,
}
