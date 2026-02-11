use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueHistoryParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    pub id: String,
    /// Max results (default 50)
    pub limit: Option<u32>,
}
