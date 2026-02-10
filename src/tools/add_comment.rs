use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddCommentParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    #[serde(rename = "issueId")]
    pub issue_id: String,
    /// Comment body (markdown)
    pub body: String,
}
