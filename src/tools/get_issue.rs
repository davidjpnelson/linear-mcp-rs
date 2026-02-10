use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    pub id: String,
}
