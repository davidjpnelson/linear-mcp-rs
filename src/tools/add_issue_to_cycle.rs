use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddIssueToCycleParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    #[serde(rename = "issueId")]
    pub issue_id: String,
    /// Cycle UUID
    #[serde(rename = "cycleId")]
    pub cycle_id: String,
}
