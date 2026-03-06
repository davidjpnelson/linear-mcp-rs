use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteIssueParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID — WARNING: this permanently deletes the issue
    pub id: String,
}
