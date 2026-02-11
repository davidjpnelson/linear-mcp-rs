use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveIssueParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID to unarchive
    pub id: String,
}
