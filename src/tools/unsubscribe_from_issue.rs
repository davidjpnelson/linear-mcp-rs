use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnsubscribeFromIssueParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    pub issue: String,
    /// Optional user email to unsubscribe (defaults to the API key owner)
    pub user: Option<String>,
}
